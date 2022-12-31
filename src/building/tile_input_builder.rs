use crate::common::{DtVector, DtInt2, DtBoundingBox};

/// Input geometry for a tile.
pub struct TileInputBuilder {
    pub coord: DtInt2,
    pub bounds: DtBoundingBox,
    pub vertices: Vec<DtVector>,
    pub indices: Vec<i32>,
    pub areas: Vec<u8>
}

impl TileInputBuilder {
    pub fn new(coord: DtInt2, bounds: DtBoundingBox) -> Self {
        TileInputBuilder {
            coord,
            bounds,
            vertices: Vec::new(),
            indices: Vec::new(),
            areas: Vec::new()
        }
    }

    /// 3 indices per triangle
    pub fn append(&mut self, vertices: &[DtVector], indices: &[i32], area: u8) {
        let vbase: i32 = self.vertices.len() as i32;

        // Copy vertices and expand box if needed
        for i in 0..vertices.len() {
            let vertice = vertices[i];
            self.vertices.push(vertice);
            self.bounds = self.bounds.merge(vertice);
        }

        // Copy indices with offset applied
        for i in 0..indices.len()  {
            self.indices.push(indices[i] + vbase);
        }

        let triangle_count = indices.len() / 3;
        for _ in 0..triangle_count {
            self.areas.push(area);
        }
    }

    pub fn append_triangle(&mut self, vertices: &[DtVector], area: u8) {
        let vbase = self.vertices.len();

        for i in 0..vertices.len() {
            let vertice = vertices[i];
            self.vertices.push(vertice);
            self.indices.push((vbase + i) as i32);
            self.bounds = self.bounds.merge(vertice);
        }
        self.areas.push(area);
    }
}