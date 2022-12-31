use std::mem;

use crate::{common::{DtInt2, DtTileHeader}};

use super::{DtVector, dt_align4};

pub struct DtPoly {
    pub first_link: u32,
    pub vertices: [u16;6],
    pub neighbors: [u16;6],
    pub flags: u16,
    pub vertex_count: u8,
    pub area_and_type: u8
}

#[derive(Clone, Debug)]
pub struct NavmeshTile {
    pub data: Vec<u8>
}

impl NavmeshTile {
    
    /// The coordinate of the tile is embedded inside the tile data header
    pub fn coord(&self) -> DtInt2 {
        let header = Self::read_header(&self.data);
        DtInt2::new(header.x, header.y)
    }

    pub fn read_header(navmesh_data: &[u8]) -> DtTileHeader {
        let header = navmesh_data.as_ptr() as *const DtTileHeader;
        unsafe {*header}
    }

    /// return the navigation mesh vertices/indices in this tile.  Useful for visual displays of the navmesh
    pub fn get_tile_vertices(&self) -> Option<(Vec<DtVector>, Vec<i32>)> {
        
        if self.data.is_empty() {
            return None;
        }
        let header = Self::read_header(&self.data);
        if header.vertcount == 0 {
            return None;
        }

        let header_size = dt_align4(mem::size_of::<DtTileHeader>() as i32) as usize;
        let verts_size = dt_align4(mem::size_of::<f32>() as i32 * 3 * header.vertcount) as usize;
        
        let slice = &self.data[header_size..self.data.len()];

        let mut vertices: Vec<DtVector> = Vec::new();
        let verts_ptr = slice.as_ptr() as *const DtVector;

        unsafe {
            for i in 0..header.vertcount {
                let vertice = verts_ptr.offset(i as isize);
                let vertice = *vertice;
                vertices.push(vertice);
            }
        }

        let mut indices: Vec<i32> = Vec::new();
        let slice = &self.data[(header_size + verts_size)..self.data.len()];
        let poly_ptr = slice.as_ptr() as *const DtPoly;
        unsafe {
            for i in 0..header.polycount {
                let poly = poly_ptr.offset(i as isize);
                for j in 0..((*poly).vertex_count - 3) {
                    indices.push((*poly).vertices[0] as i32);
                    indices.push((*poly).vertices[(j + 1) as usize] as i32);
                    indices.push((*poly).vertices[(j + 2) as usize] as i32);
                }
            }
        }
        
        Some((vertices, indices))
    }
}