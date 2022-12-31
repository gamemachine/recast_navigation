use crate::{common::{navmesh_tile::NavmeshTile, DtVector, DtInt2, DtArea, NavAgentSettings}, bindings::{CreateBuilder, BuildNavmesh, SetSettings, DestroyBuilder}};

use super::{navmesh_build_utils::NavmeshBuildUtils, DtBuildSettings, tile_input_builder::TileInputBuilder, NavBuildSettings};

/// build result codes. Some originate locally some from C
/// Note: ZeroVertCount is fairly common and normal.  It means we didn't pass recast input data that resulted in navmesh geometry.
/// Other errors from the C side shouldn't happen in normal circumstances.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildResultCode {
    None = 10000,
    Success = 0,
    AreaInput = -1001,
    VerticesInput = -1000,
    CreateBuilderFailed = -100,
    
    /// error codes returned by the C side
    RcRasterizeTriangles = 10,
    RcAllocCompactHeightfield = 20,
    RcBuildCompactHeightfield = 30,
    RcErodeWalkableArea = 40,
    RcBuildDistanceField = 50,
    RcBuildRegions = 60,
    RcAllocContourSet = 70,
    RcBuildContours = 80,
    RcAllocPolyMesh = 90,
    RcBuildPolyMesh = 100,
    ZeroVertCount = 110,
    NullVerts = 120,
    RcAllocPolyMeshDetail = 130,
    RcBuildPolyMeshDetail = 140,

    /// should never see these, see the C source for definitions if needed
    CreateDetourMesh10 = 1010,
    CreateDetourMesh11 = 1011,
    CreateDetourMesh12 = 1012,
    CreateDetourMesh13 = 1013,
    CreateDetourMesh14 = 1014,
    CreateDetourMesh15 = 1015,
    CreateDetourMesh16 = 1016,
    CreateDetourMesh17 = 1017
    
}

impl From<i32> for BuildResultCode {
    fn from(item: i32) -> Self {
        match item {
            0 => BuildResultCode::Success,
            -1001 => BuildResultCode::AreaInput,
            -1000 => BuildResultCode::VerticesInput,
            -100 => BuildResultCode::CreateBuilderFailed,

            10 => BuildResultCode::RcRasterizeTriangles,
            20 => BuildResultCode::RcAllocCompactHeightfield,
            30 => BuildResultCode::RcBuildCompactHeightfield,
            40 => BuildResultCode::RcErodeWalkableArea,
            50 => BuildResultCode::RcBuildDistanceField,
            60 => BuildResultCode::RcBuildRegions,
            70 => BuildResultCode::RcAllocContourSet,
            80 => BuildResultCode::RcBuildContours,
            90 => BuildResultCode::RcAllocPolyMesh,
            100 => BuildResultCode::RcBuildPolyMesh,
            110 => BuildResultCode::ZeroVertCount,
            120 => BuildResultCode::NullVerts,
            130 => BuildResultCode::RcAllocPolyMeshDetail,
            140 => BuildResultCode::RcBuildPolyMeshDetail,

            1010 => BuildResultCode::CreateDetourMesh10,
            1011 => BuildResultCode::CreateDetourMesh11,
            1012 => BuildResultCode::CreateDetourMesh12,
            1013 => BuildResultCode::CreateDetourMesh13,
            1014 => BuildResultCode::CreateDetourMesh14,
            1015 => BuildResultCode::CreateDetourMesh15,
            1016 => BuildResultCode::CreateDetourMesh16,
            1017 => BuildResultCode::CreateDetourMesh17,
           
            _ => BuildResultCode::None
        }
    }
}

/// Result of building a navmesh tile
#[derive(Clone, Debug)]
pub struct NavmeshBuildResult {
    pub success: bool,
	pub result_code: BuildResultCode,
    pub tiles_built: i32,
    pub vertice_count: i32,
    pub triangle_count: i32,
    pub tile: Option<NavmeshTile>,
}

impl NavmeshBuildResult {
    pub fn default() -> Self {
        NavmeshBuildResult {
            success: false,
            result_code: BuildResultCode::None,
            tiles_built: 0,
            vertice_count: 0,
            triangle_count: 0,
            tile: None
        }
    }
}

pub struct NavmeshBuilder {
    pub build_settings: NavBuildSettings,
    pub agent_settings: NavAgentSettings,
}

impl NavmeshBuilder {
    pub fn new(build_settings: NavBuildSettings, agent_settings: NavAgentSettings) -> Self {
        NavmeshBuilder {
            build_settings,
            agent_settings,
        }
    }

    pub fn build_tile(&mut self, mut input: TileInputBuilder) -> NavmeshBuildResult {
        let mut result = NavmeshBuildResult::default();

        if input.areas.len() != input.indices.len() / 3 {
            result.result_code = BuildResultCode::AreaInput;
            return result;
        }

        if input.vertices.len() != input.indices.len() {
            result.result_code = BuildResultCode::VerticesInput;
            return result;
        }
        
        Self::normalize_input_heights(&mut input);
        
        let mut tile_bounding_box = NavmeshBuildUtils::calculate_tile_bounding_box(
            self.build_settings,
            input.coord,
        );
        

        tile_bounding_box.min.y = input.bounds.min.y;
        tile_bounding_box.max.y = input.bounds.max.y;
       
        NavmeshBuildUtils::snap_bounding_box_to_cell_height(
            self.build_settings,
            &mut tile_bounding_box,
        );


        let mut dt_build_settings = DtBuildSettings {
            bounding_box: tile_bounding_box,
            tile_position: input.coord,
            tile_size: self.build_settings.tile_size,

            cell_height: self.build_settings.cell_height,
            cell_size: self.build_settings.cell_size,
            region_min_area: self.build_settings.min_region_area,
            region_merge_area: self.build_settings.region_merge_area,
            edge_max_len: self.build_settings.max_edge_len,
            edge_max_error: self.build_settings.max_edge_error,
            detail_sample_dist: self.build_settings.detail_sampling_distance,
            detail_sample_max_error: self.build_settings.max_detail_sampling_error,

            agent_height: self.agent_settings.height,
            agent_radius: self.agent_settings.radius,
            agent_max_climb: self.agent_settings.max_climb,
            agent_max_slope: self.agent_settings.max_slope,
        };

        unsafe {
            let ptr = CreateBuilder();
            if ptr.is_null() {
                result.result_code = BuildResultCode::CreateBuilderFailed;
                return result;
            }

            SetSettings(ptr, &mut dt_build_settings as *mut DtBuildSettings);

            let generated_data_ptr = BuildNavmesh(
                ptr,
                input.vertices.as_mut_ptr(),
                input.vertices.len() as i32,
                input.indices.as_mut_ptr(),
                input.indices.len() as i32,
                input.areas.as_mut_ptr(),
            );
            let generated_data = *generated_data_ptr;
            result.result_code = generated_data.error.into();
            result.success = generated_data.success;

            if generated_data.error == 0
                && generated_data.success
                && generated_data.navmesh_data_length > 0
            {
                let mut data: Vec<u8> = vec![0; generated_data.navmesh_data_length as usize];
                std::ptr::copy_nonoverlapping(
                    generated_data.navmesh_data,
                    data.as_mut_ptr(),
                    generated_data.navmesh_data_length as usize,
                );

                let tile = NavmeshTile { data };
                result.tile = Some(tile);

                DestroyBuilder(ptr);
            }
        }

        result
    }

    fn normalize_input_heights(input: &mut TileInputBuilder) {
        input.bounds.max.y = f32::MIN.max(input.bounds.max.y);
        input.bounds.min.y = f32::MAX.min(input.bounds.min.y);
    }

    pub fn build_test_tile(width: f32) -> NavmeshBuildResult {
        let mut builder = NavmeshBuilder::new(NavBuildSettings::default(), NavAgentSettings::default());
        let coord = DtInt2::new(0, 0);
        let bounds = NavmeshBuildUtils::calculate_tile_bounding_box(builder.build_settings, coord);
        let mut input = TileInputBuilder::new(coord, bounds);

        let vertices:[DtVector;3] = [DtVector::new(0.0, 1.0, 0.0), DtVector::new(0.0, 1.0, width), DtVector::new(width, 1.0, width)];
        input.append_triangle(&vertices, DtArea::WALKABLE);

        let vertices:[DtVector;3] = [DtVector::new(0.0, 1.0, 0.0), DtVector::new(width, 1.0, 0.0), DtVector::new(width, 1.0, width)];
        input.append_triangle(&vertices, DtArea::WALKABLE);

        builder.build_tile(input)
    }
}


#[cfg(test)]
mod tests {

    use crate::{
        building::{
            navmesh_build_utils::NavmeshBuildUtils, tile_input_builder::TileInputBuilder, NavBuildSettings,
        },
        common::{DtArea, DtInt2, DtVector, NavAgentSettings},
    };

    use super::NavmeshBuilder;

    #[test]
    fn create_tile() {
        let mut builder = NavmeshBuilder::new(NavBuildSettings::default(), NavAgentSettings::default());

        let coord = DtInt2::new(0, 0);
        let bounds = NavmeshBuildUtils::calculate_tile_bounding_box(builder.build_settings, coord);
        
        println!("bounds {:?}", bounds);
       
        let mut input = TileInputBuilder::new(coord, bounds);

        let width = 30.0;
        let vertices:[DtVector;3] = [DtVector::new(0.0, 1.0, 0.0), DtVector::new(0.0, 1.0, width), DtVector::new(width, 1.0, width)];
        input.append_triangle(&vertices, DtArea::WALKABLE);

        let vertices:[DtVector;3] = [DtVector::new(0.0, 1.0, 0.0), DtVector::new(width, 1.0, 0.0), DtVector::new(width, 1.0, width)];
        input.append_triangle(&vertices, DtArea::WALKABLE);

        println!("input bounds {:?}", input.bounds);
        let result = builder.build_tile(input);
        println!("build.success {:?} build.error:{:?}", result.success, result.result_code);
        if result.success {
            if let Some(tile) = &result.tile {
                if let Some((vertices, _indices)) = tile.get_tile_vertices() {
                    for vert in vertices.iter() {
                        println!("{:?}", vert);
                    }
                }
            }
        }
    }
}
