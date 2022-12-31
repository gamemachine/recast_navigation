
use std::{sync::Arc};

use crossbeam::queue::ArrayQueue;
use rustc_hash::{FxHashSet};

use crate::{bindings::{CreateNavmesh, DestroyNavmesh, RemoveTile, AddTile, RawNavmeshPtr}, queries::nav_query::NavQuery, building::NavBuildSettings};

use super::{DtInt2, navmesh_tile::NavmeshTile};


struct NavmeshPtr(*mut RawNavmeshPtr);
unsafe impl Send for NavmeshPtr {}

pub struct NavQueryPool {
    queries: ArrayQueue<NavQuery>,
    initialized_size: usize
}

impl NavQueryPool {
    fn new(size: usize, navmesh_ptr: *mut RawNavmeshPtr, max_nodes: i32, max_path_points: i32) -> Option<Self> {
        let pool = NavQueryPool {
            queries: ArrayQueue::new(size),
            initialized_size: size
        };
        for _ in 0..size {
            if let Some(query) = NavQuery::new(navmesh_ptr, max_nodes, max_path_points) {
                pool.queries.push(query).unwrap_or_default();
            } else {
                return None;
            }
            
        }
        Some(pool)
    }

    pub fn is_full(&self) -> bool {
        self.queries.len() == self.initialized_size
    }
   
    pub fn clear(&self) {
        while let Some(query) = self.queries.pop() {
            drop(query);
        }
    }

    pub fn pop(&self) -> Option<NavQuery> {
        self.queries.pop()
    }

    pub fn push(&self, query: NavQuery) {
        self.queries.push(query).unwrap_or_default();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NavmeshSettings {
    pub tile_size: i32,
    pub cell_size: f32,
    pub map_size: f32,
    pub query_pool_size: usize,
    pub query_max_nodes: i32,
    pub query_max_path_points: i32
    
}

impl NavmeshSettings {
    pub fn new(build_settings: NavBuildSettings, map_size: f32, query_pool_size: usize, query_max_nodes: i32, query_max_path_points: i32) -> Self {
        NavmeshSettings {
            tile_size: build_settings.tile_size,
            cell_size: build_settings.cell_size,
            map_size,
            query_pool_size,
            query_max_nodes,
            query_max_path_points
        }
    }

    pub fn default(build_settings: NavBuildSettings, map_size: f32, query_pool_size: usize) -> Self {
        NavmeshSettings {
            tile_size: build_settings.tile_size,
            cell_size: build_settings.cell_size,
            map_size,
            query_pool_size,
            query_max_nodes: 4096,
            query_max_path_points: 512
        }
    }
}

/// https://groups.google.com/g/recastnavigation/c/irmJ5uonNnM  - The gist of thread safety in recast

/// navmesh queries have pointers into the navmesh data on the C side.  So it's not safe to run queries while mutating the navmesh (adding/removing tiles)
/// Building tiles is completely separate and can be done from any/multiple threads.

/// There isn't really a good way to wrap this safely without compromises.  And performance is paramount.
/// Requiring mutexes is too much of a perf hit.
/// Taking the result path vector out of the query would result in not needing mutable access.  But that complicates the api forcing you to manage a pool of path result vectors.
/// Moving the navmesh and it's queries in and out of an Arc and having a notion of an update/query mode works. But I felt that was overly complex here.
/// So we just do simple reference counting.  Mutating functions on the navmesh require all the allocated queries to be in the pool.  Ie not in use.
/// 
/// So the usage pattern for parallel access is you clone the arc wrapped query pool, then take/return queries as needed.

/// Queries should be destroyed on the C++ side before the navmesh is destroyed.  Our drop implementation handles that

pub struct Navmesh {
    navmesh_ptr: NavmeshPtr,
    tile_coords: FxHashSet<DtInt2>,
    pub query_pool: Arc<NavQueryPool>
}

impl Navmesh {

    pub fn ceil_pow2(mut i: i32) -> i32 {
        i -= 1;
        i |= i >> 1;
        i |= i >> 2;
        i |= i >> 4;
        i |= i >> 8;
        i |= i >> 16;
        i + 1
    }

    // recast samples set max tiles to align to 1 << bits.  Vs just checking that it doesn't exceed 1 << largest mask.
    //  Appears to just be an artifact of samples using 32 bit polyrefs but retain the logic just in case it matters.
    pub fn calculate_max_tile_bits(tile_size: i32, cell_size: f32, map_size: f32) -> i32 {
        let tcs = tile_size as f32 * cell_size;
        let w = map_size / tcs;
        let max_tiles = (w * w).ceil() as i32;
        (Self::ceil_pow2(max_tiles) as f32).log(2.0) as i32
    }

    pub fn new(settings: NavmeshSettings) -> Option<Self> {
        let tile_bits = Self::calculate_max_tile_bits(settings.tile_size, settings.cell_size, settings.map_size);
        let poly_bits = 8; // 256 polys per tile.  20 max.

        let cell_tile_size = settings.tile_size as f32 * settings.cell_size;

        let ptr = unsafe {CreateNavmesh(cell_tile_size, tile_bits, poly_bits)};
        if ptr.is_null() {
            return None;
        }

        let navmesh_ptr = NavmeshPtr(ptr);
        let query_pool = NavQueryPool::new(settings.query_pool_size, navmesh_ptr.0, settings.query_max_nodes, settings.query_max_path_points);
        
        let query_pool = Arc::new(query_pool.unwrap());

        let navmesh = Navmesh {
            navmesh_ptr,
            tile_coords: FxHashSet::default(),
            query_pool
        };

        Some(navmesh)
    }

    pub fn raw_ptr(&self) -> *mut RawNavmeshPtr {
        self.navmesh_ptr.0
    }

    pub fn add_or_replace_tile(&mut self, mut tile: NavmeshTile) -> bool {
        if !self.query_pool.is_full() {
            return false;
        }

        let coord = tile.coord();
        self.remove_tile_internal(&coord);
        self.tile_coords.insert(coord);

        unsafe {
            AddTile(self.navmesh_ptr.0, tile.data.as_mut_ptr() as *mut u8, tile.data.len() as i32) == 1
        }
    }

    pub fn remove_tile(&mut self, coord: &DtInt2) -> bool {
        if !self.query_pool.is_full() {
            return false;
        }
        self.remove_tile_internal(coord)
    }

    fn remove_tile_internal(&mut self, coord: &DtInt2) -> bool {
        if self.tile_coords.remove(coord) {
            unsafe {
                if RemoveTile(self.navmesh_ptr.0, coord as *const DtInt2) == 1 {
                    self.tile_coords.remove(coord);
                    return true;
                }
            }
        }
        false
    }
    
}

impl Drop for Navmesh {
    fn drop(&mut self) {
        self.query_pool.clear();
        unsafe {DestroyNavmesh(self.navmesh_ptr.0)};
    }
}

#[cfg(test)]
mod tests {
    use crate::{building::{navmesh_builder::NavmeshBuilder, NavBuildSettings}, common::navmesh::NavmeshSettings};

    use super::Navmesh;


    #[test]
    fn create_drop() {
        let result = NavmeshBuilder::build_test_tile(30.0);
        let tile = result.tile.unwrap();

        let build_settings = NavBuildSettings::default();
        let navmesh_settings = NavmeshSettings::default(build_settings, 2048.0, 10);
        let mut navmesh = Navmesh::new(navmesh_settings).unwrap();
        assert!(navmesh.add_or_replace_tile(tile));
       
    }

}