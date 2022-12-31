use crate::common::{*};

pub mod navmesh_build_utils;
pub mod navmesh_builder;
pub mod tile_input_builder;
pub mod shape_to_mesh;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DtGeneratedData
{
	pub success: bool,
	pub error: i32,
	pub navmesh_data: *mut u8,
	pub navmesh_data_length: i32
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DtBuildSettings
{
	// Bounding box for the generated navigation mesh
	pub bounding_box: DtBoundingBox,
	pub cell_height: f32,
	pub cell_size: f32,
	pub tile_size: i32,
	pub tile_position: DtInt2,
	pub region_min_area: i32,
	pub region_merge_area: i32,
	pub edge_max_len: f32,
	pub edge_max_error: f32,
	pub detail_sample_dist: f32,
	pub detail_sample_max_error: f32,
	pub agent_height: f32,
	pub agent_radius: f32,
	pub agent_max_climb: f32,
	pub agent_max_slope: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct NavBuildSettings {
    /// The Height of a grid cell in the navigation mesh building steps using heightfields. 
    /// A lower number means higher precision on the vertical axis but longer build times
    pub cell_height: f32,

    /// The Width/Height of a grid cell in the navigation mesh building steps using heightfields. 
    /// A lower number means higher precision on the horizontal axes but longer build times
    pub cell_size: f32,

    /// Tile size used for Navigation mesh tiles, the final size of a tile is CellSize*TileSize
    pub tile_size: i32,

    /// The minimum number of cells allowed to form isolated island areas
    pub min_region_area: i32,

    /// Any regions with a span count smaller than this value will, if possible, 
    /// be merged with larger regions.
    pub region_merge_area: i32,

    /// The maximum allowed length for contour edges along the border of the mesh.
    pub max_edge_len: f32,

    /// The maximum distance a simplfied contour's border edges should deviate from the original raw contour.
    pub max_edge_error: f32,

    /// Sets the sampling distance to use when generating the detail mesh. (For height detail only.)
    pub detail_sampling_distance: f32,

    /// The maximum distance the detail mesh surface should deviate from heightfield data. (For height detail only.)
    pub max_detail_sampling_error: f32
}

impl NavBuildSettings {
    pub fn default() -> Self {
        NavBuildSettings {
            cell_height: 0.2,
            cell_size: 0.3,
            tile_size: 64,
            min_region_area: 2,
            region_merge_area: 20,
            max_edge_len: 12.0,
            max_edge_error: 1.3,
            detail_sampling_distance: 6.0,
            max_detail_sampling_error: 1.0
        }
    }

    pub fn high_quality() -> Self {
        NavBuildSettings {
            cell_height: 0.083,
            cell_size: 0.166,
            tile_size: 64,
            min_region_area: 2,
            region_merge_area: 20,
            max_edge_len: 12.0,
            max_edge_error: 1.3,
            detail_sampling_distance: 6.0,
            max_detail_sampling_error: 0.5
        }
    }
}