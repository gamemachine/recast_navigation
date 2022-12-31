use std::ops;

use rapier3d_f64::prelude::{Point, Real};

pub mod navmesh;
pub mod navmesh_tile;

pub fn dt_align4(size: i32) -> i32 {
    (size + 3) & !3
}


#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct DtVector2 {
    pub x: f32,
    pub y: f32
}

impl DtVector2 {
    pub fn new(x: f32, y: f32) -> Self {
        DtVector2 {
            x,
            y
        }
    }
}

impl ops::Mul<f32> for DtVector2 {
    type Output = DtVector2;

    fn mul(self, rhs: f32) -> DtVector2 {

        DtVector2 {x: self.x * rhs, y: self.y * rhs}
    }
}

impl ops::Div<f32> for DtVector2 {
    type Output = DtVector2;

    fn div(self, rhs: f32) -> DtVector2 {

        DtVector2 {x: self.x / rhs, y: self.y / rhs}
    }
}

impl ops::Add<DtVector2> for DtVector2 {
    type Output = DtVector2;

    fn add(self, rhs: DtVector2) -> DtVector2 {

        DtVector2 {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct DtVector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl DtVector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        DtVector {
            x,
            y,
            z
        }
    }

    pub fn xz(&self) -> DtVector2 {
        DtVector2::new(self.x, self.z)
    }

    pub fn min(&self, rhs: Self) -> Self {
        DtVector {x: self.x.min(rhs.x), y: self.y.min(rhs.y), z: self.z.min(rhs.z)}
    }

    pub fn max(&self, rhs: Self) -> Self {
        DtVector {x: self.x.max(rhs.x), y: self.y.max(rhs.y), z: self.z.max(rhs.z)}
    }
}

impl From<Point<Real>> for DtVector {
    fn from(item: Point<Real>) -> Self {
        DtVector {
            x: item.x as f32,
            y: item.y as f32,
            z: item.z as f32
        }
    }
}

impl ops::Mul<f32> for DtVector {
    type Output = DtVector;

    fn mul(self, rhs: f32) -> DtVector {

        DtVector {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::Add<DtVector> for DtVector {
    type Output = DtVector;

    fn add(self, rhs: DtVector) -> DtVector {

        DtVector {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl ops::Sub<DtVector> for DtVector {
    type Output = DtVector;

    fn sub(self, rhs: DtVector) -> DtVector {

        DtVector {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

#[derive(Clone, Copy, Hash, Eq, Debug)]
#[repr(C)]
pub struct DtInt2 {
    pub x: i32,
    pub y: i32,
}

impl DtInt2 {
    pub fn new(x: i32, y: i32) -> Self {
        DtInt2 { x, y}
    }

    pub fn zero() -> Self {
        DtInt2 { x: 0, y: 0}
    }
}

impl PartialEq for DtInt2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
        && self.y == other.y
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct NavAgentSettings {
    /// The height of the entities in this group. Entities can't enter areas with ceilings lower than this value.
    pub height: f32,

    /// The maximum height that entities in this group can climb. 
    pub max_climb: f32,

    /// The maximum incline (in degrees) that entities in this group can climb. Entities can't go up or down slopes higher than this value. 
    pub max_slope: f32,

     /// The larger this value, the larger the area of the navigation mesh entities use. Entities can't pass through gaps of less than twice the radius.
    pub radius: f32
}

impl NavAgentSettings {
    pub fn default() -> Self {
        NavAgentSettings {
            height: 2.0,
            max_climb: 0.4,
            max_slope: 45.0,
            radius: 0.5
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DtTileHeader {
    pub magic: i32,
    pub version: i32,
    pub x: i32,
    pub y: i32,
    pub layer: i32,
    pub user_id: i32,
    pub polycount: i32,
    pub vertcount: i32,
    pub max_linked_count: i32,
    pub detail_mesh_count: i32,
    pub detail_vert_count: i32,
    pub detail_tri_count: i32,
    pub bv_node_count: i32,
    pub off_mesh_con_count: i32,
    pub off_mesh_base: i32,
    pub walkable_height: i32,
    pub walkable_radius: i32,
    pub walkable_climb: i32,
    pub bmin: [f32;3],
    pub bmax: [f32; 3],
    pub bv_quant_factor: f32
}

pub struct DtArea {}

impl DtArea {
    pub const NULL: u8 = 0;
    pub const WALKABLE: u8 = 63;
}


#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct DtBoundingBox {
    pub min: DtVector,
    pub max: DtVector,
}

impl DtBoundingBox {
    pub fn new(min: DtVector, max: DtVector) -> Self {
        DtBoundingBox { min, max }
    }

    pub fn from_center(center: DtVector, size: DtVector) -> Self {
        let extents = size * 0.5;
        DtBoundingBox::new(center - extents, center + extents)
    }

    pub fn merge(&self, point: DtVector) -> Self {
        let mut result = Self::default();
        result.min = self.min.min(point);
        result.max = self.max.max(point);
        result
    }

    pub fn merge_box(&self, rhs: DtBoundingBox) -> Self {
        let mut result = Self::default();
        result.min = self.min.min(rhs.min);
        result.max = self.max.max(rhs.max);
        result
    }

    pub fn intersects(&self, rhs: DtBoundingBox) -> bool {
        if self.min.x > rhs.max.x || rhs.min.x > self.max.x {
            return false;
        }

        if self.min.y > rhs.max.y || rhs.min.y > self.max.y {
            return false;
        }

        if self.min.z > rhs.max.z || rhs.min.z > self.max.z {
            return false;
        }

        true
    }

    pub fn contains_point(&self, point: DtVector) -> bool {
        self.min.x <= point.x
            && self.max.x >= point.x
            && self.min.y <= point.y
            && self.max.y >= point.y
            && self.min.z <= point.z && self.max.z >= point.z
    }

    pub fn expand(&mut self, amount: f32) {
        self.min = DtVector::new(self.min.x - amount, self.min.y - amount, self.min.z - amount);
        self.max = DtVector::new(self.max.x + amount, self.max.y + amount, self.max.z + amount);
    }
}
