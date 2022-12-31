use rustc_hash::FxHashSet;


use crate::common::{DtInt2, DtVector2, DtBoundingBox};

use super::NavBuildSettings;

pub struct NavmeshBuildUtils {}

impl NavmeshBuildUtils {

    pub fn get_overlapping_tiles_from_settings(settings: NavBuildSettings, bounding_box: DtBoundingBox) -> FxHashSet<DtInt2> {
        Self::get_overlapping_tiles(settings.tile_size, settings.cell_size, bounding_box)
    }

    /// Return tiles that overlap a given bounding box.
    pub fn get_overlapping_tiles(tile_size: i32, cell_size: f32, bounding_box: DtBoundingBox) -> FxHashSet<DtInt2> {

        let mut map: FxHashSet<DtInt2> = FxHashSet::default();

        let tcs = tile_size as f32 * cell_size;
        let start = bounding_box.min.xz() / tcs;
        let end = bounding_box.max.xz() / tcs;

        let start_tile = DtInt2::new(start.x.floor() as i32, start.y.floor() as i32);
        let end_tile = DtInt2::new(end.x.ceil() as i32, end.y.ceil() as i32);

        for y in start_tile.y..end_tile.y {
            for x in start_tile.x..end_tile.x {
                map.insert(DtInt2::new(x, y));
            }
        }
        map
    }

    /// Snap Y to tile height to avoid height differences between tiles
    pub fn snap_bounding_box_to_cell_height(settings: NavBuildSettings, bounding_box: &mut DtBoundingBox) {
        bounding_box.min.y = f32::floor(bounding_box.min.y / settings.cell_height) * settings.cell_height;
        bounding_box.max.y = f32::ceil(bounding_box.max.y / settings.cell_height) * settings.cell_height;
    }

    /// This does not factor in tile border size, which you need to include in the bounds given to TileInputBuilder.  Or you will have gaps between tiles due to no border data
    /// on the C side the border size is calculated as (agent radius / cell size). bounds is then expanded by border size * cell size.
    /// Or you can just expand the bounds by a set amount of 2-3 which seems to work well.
    /// See NavigationBuilder::BuildNavmesh if you want to try and tighten this up with an exact formula.  I never took the time to do that.
    pub fn calculate_tile_bounding_box(settings: NavBuildSettings, tile_coord: DtInt2) -> DtBoundingBox {

        let tcs = settings.tile_size as f32 * settings.cell_size;
        let tile_min = DtVector2::new(tile_coord.x as f32 * tcs, tile_coord.y as f32 * tcs);
        let tile_max = tile_min + DtVector2::new(tcs, tcs);
        let mut bounding_box = DtBoundingBox::default();
        bounding_box.min.x = tile_min.x;
        bounding_box.min.z = tile_min.y;
        bounding_box.max.x = tile_max.x;
        bounding_box.max.z = tile_max.y;
        bounding_box.min.y = 0.0;
        bounding_box.max.y = 0.0;

        bounding_box
    }

}