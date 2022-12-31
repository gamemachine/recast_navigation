use crate::common::DtVector;


pub mod nav_query;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct NavQuerySettings {
    pub find_nearest_poly_extent: DtVector,
    pub max_path_points: i32
}

impl NavQuerySettings {
    pub fn default() -> Self {
        NavQuerySettings {
            find_nearest_poly_extent: DtVector::new(2.0, 4.0, 2.0),
            max_path_points: 512
        }
    }

    pub fn lenient() -> Self {
        NavQuerySettings {
            find_nearest_poly_extent: DtVector::new(10.0, 10.0, 10.0),
            max_path_points: 2048
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DtPathFindQuery {
    pub source: DtVector,
    pub target: DtVector,
    pub find_nearest_poly_extent: DtVector,
    pub max_path_points: i32
}

impl DtPathFindQuery {
    pub fn new(source: DtVector, target: DtVector, settings: &NavQuerySettings) -> Self {
        DtPathFindQuery {
            source,
            target,
            find_nearest_poly_extent: settings.find_nearest_poly_extent,
            max_path_points: settings.max_path_points
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DtPathFindResult {
    pub path_found: bool,
    pub path_points: *mut DtVector,
    pub num_path_points: i32
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DtRaycastQuery {
    pub source: DtVector,
    pub target: DtVector,
    pub find_nearest_poly_extent: DtVector,
    pub max_path_points: i32
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct DtRaycastResult {
    pub hit: bool,
    pub position: DtVector,
    pub normal: DtVector
}