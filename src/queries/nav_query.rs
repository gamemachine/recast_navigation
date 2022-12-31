
use crate::{bindings::{QueryDestroy, RawNavqueryPtr, RawNavmeshPtr, QueryCreate, QueryFindStraightPath, QuerySamplePosition, QueryGetLocation, QueryRaycast, QueryHasPath}, common::DtVector};

use super::{NavQuerySettings, DtPathFindQuery, DtPathFindResult, DtRaycastQuery, DtRaycastResult};

struct NavqueryPtr(*mut RawNavqueryPtr);
unsafe impl Send for NavqueryPtr {}

pub struct NavQuery {
    query_ptr: NavqueryPtr,
    result_path: Vec<DtVector>,
    max_path_points: i32
}

impl NavQuery {
    pub fn new(navmesh_ptr: *mut RawNavmeshPtr, max_nodes: i32, max_path_points: i32) -> Option<Self> {
        let query_ptr = unsafe {QueryCreate(navmesh_ptr, max_nodes)};
        if query_ptr.is_null() {
            return None;
        }
        
        let query = NavQuery {
            query_ptr: NavqueryPtr(query_ptr),
            result_path: vec![DtVector::default();max_path_points as usize],
            max_path_points
        };
        Some(query)
    }

    pub fn get_path(&self, len: usize) -> &[DtVector] {
        &self.result_path[0..len]
    }

    pub fn has_path(&self, query_settings: NavQuerySettings, source: DtVector, target: DtVector) -> bool {
        let query = DtPathFindQuery {
            source,
            target,
            find_nearest_poly_extent: query_settings.find_nearest_poly_extent,
            max_path_points: query_settings.max_path_points.min(self.max_path_points),
        };
        
        unsafe {
            QueryHasPath(self.query_ptr.0, &query as *const DtPathFindQuery) == 1
        }
    }

    pub fn find_path(&mut self, query_settings: NavQuerySettings, source: DtVector, target: DtVector) -> i32 {
        let query = DtPathFindQuery {
            source,
            target,
            find_nearest_poly_extent: query_settings.find_nearest_poly_extent,
            max_path_points: query_settings.max_path_points.min(self.max_path_points),
        };
        
        unsafe {
            let mut result = DtPathFindResult {
                path_found: false,
                path_points: self.result_path.as_mut_ptr() as *mut DtVector,
                num_path_points: 0,
            };
            QueryFindStraightPath(self.query_ptr.0, &query as *const DtPathFindQuery, &mut result as *mut DtPathFindResult);
            result.num_path_points
        }
    }

    /// sample_position does not use the detail mesh, height will not match surface
    pub fn sample_position(&self, point: &DtVector, extent: &DtVector) -> Option<DtVector> {
        let mut result = DtVector::default();

        unsafe {
            let res = QuerySamplePosition(self.query_ptr.0, point as *const DtVector, extent as *const DtVector, &mut result as *mut DtVector);
            if res == 1 {
                Some(result)
            } else {
                None
            }
        }
    }

    /// get_location is the same as SamplePosition but it does use the detail mesh, returning the surface height
    /// You call this every frame while moving an agent over the path to get the correct height to place them at
    pub fn get_location(&self, point: &DtVector, extent: &DtVector) -> Option<DtVector> {
        let mut result = DtVector::default();

        unsafe {
            let res = QueryGetLocation(self.query_ptr.0, point as *const DtVector, extent as *const DtVector, &mut result as *mut DtVector);
            if res == 1 {
                Some(result)
            } else {
                None
            }
        }
    }

    /// Does not work as expected.  Need to redo C implementation.
    pub fn raycast(&self, query_settings: NavQuerySettings, source: DtVector, target: DtVector) -> DtRaycastResult {
        let query = DtRaycastQuery {
            source,
            target,
            find_nearest_poly_extent: query_settings.find_nearest_poly_extent,
            max_path_points: query_settings.max_path_points,
        };

        let mut result = DtRaycastResult::default();
        unsafe {
            QueryRaycast(self.query_ptr.0, &query as *const DtRaycastQuery, &mut result as *mut DtRaycastResult);
            result
        }
    }

    pub fn test(&self) {

    }
}

impl Drop for NavQuery {
    fn drop(&mut self) {
        unsafe {QueryDestroy(self.query_ptr.0)};
    }
}


#[cfg(test)]
mod tests {
    use crate::{building::{navmesh_builder::NavmeshBuilder, NavBuildSettings}, common::{navmesh::{Navmesh, NavmeshSettings}, DtVector}, queries::NavQuerySettings};


    #[test]
    fn basic_queries() {
        let result = NavmeshBuilder::build_test_tile(30.0);
        let tile = result.tile.unwrap();

        let build_settings = NavBuildSettings::default();
        let navmesh_settings = NavmeshSettings::default(build_settings, 2048.0, 10);
        let mut navmesh = Navmesh::new(navmesh_settings).unwrap();
        navmesh.add_or_replace_tile(tile);

        let mut query = navmesh.query_pool.pop().unwrap();

        let start = DtVector::new(1.0, 1.0, 1.0);
        let end = DtVector::new(10.0, 1.0, 10.0);
        let path_len = query.find_path(NavQuerySettings::default(), start, end);
        println!("path len {:?}", path_len);
        for point in query.get_path(path_len as usize) {
            println!("{:?}", point);
        }
        let has_path = query.has_path(NavQuerySettings::default(), start, end);
        assert!(has_path);

        let start = DtVector::new(1.0, 1.0, 1.0);
        let end = DtVector::new(1000.0, 1.0, 1000.0);
        let path_len = query.find_path(NavQuerySettings::default(), start, end);
        assert_eq!(0, path_len);

        let result = query.sample_position(&start, &DtVector::new(2.0, 2.0, 2.0));
        assert!(result.is_some());

        let result = query.get_location(&start, &DtVector::new(2.0, 2.0, 2.0));
        assert!(result.is_some());
        let result = result.unwrap();
        println!("{:?}", result);

        let start = DtVector::new(1.0, 4.0, 1.0);
        let end = DtVector::new(10.0, 4.0, 10.0);
        let result = query.raycast(NavQuerySettings::default(), start, end);
        println!("{:?}", result);
        assert!(result.hit);
    }
}