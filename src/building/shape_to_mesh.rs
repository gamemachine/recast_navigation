use rapier3d_f64::{prelude::*};

use crate::common::DtVector;


pub struct ShapeToMesh {}

impl ShapeToMesh {

    pub fn shape_to_mesh(shared_shape: &SharedShape) -> Option<(Vec<DtVector>, Vec<i32>)> {
        
        match shared_shape.shape_type() {
            ShapeType::Cuboid => {
                let out = shared_shape.as_cuboid().unwrap().to_trimesh();
                Some(Self::trimesh_output_to_mesh(out.0, out.1))
            },
            ShapeType::Capsule => {
                let out = shared_shape.as_capsule().unwrap().to_trimesh(4, 4);
                Some(Self::trimesh_output_to_mesh(out.0, out.1))
            },
            ShapeType::Triangle => {
                let out = shared_shape.as_triangle().unwrap();
                let mut triangles:  Vec<(usize, Triangle)> = Vec::new();
                triangles.push((0, *out));
                Some(Self::triangles_output_to_mesh(triangles))
            },
            ShapeType::TriMesh => {
                let trimesh = shared_shape.as_trimesh().unwrap();
                
                let triangles: Vec<(usize, Triangle)> = trimesh.triangles().enumerate().collect();
                Some(Self::triangles_output_to_mesh(triangles))
            },
            
            ShapeType::HeightField => {
                let out = shared_shape.as_heightfield().unwrap().to_trimesh();
                Some(Self::trimesh_output_to_mesh(out.0, out.1))
            },
          
            ShapeType::Cylinder => {
                let out = shared_shape.as_cylinder().unwrap().to_trimesh(4);
                Some(Self::trimesh_output_to_mesh(out.0, out.1))
            },
            _ => {
                None
            }
        }
    }

    pub fn weld_vertices(vertices: &Vec<DtVector>, indices: &Vec<u32>) -> (Vec<u32>, Vec<DtVector>) {
        let remap_table = meshopt::generate_vertex_remap(vertices, Some(indices));

        let remapped_indices = meshopt::remap_index_buffer(Some(indices), remap_table.0, &remap_table.1);
        let remapped_vertices = meshopt::remap_vertex_buffer(vertices, remap_table.0, &remap_table.1);

        (remapped_indices, remapped_vertices)
    }

    fn triangles_output_to_mesh(triangles: Vec<(usize, Triangle)>) -> (Vec<DtVector>, Vec<i32>) {
        let mut vertices: Vec<DtVector> = Vec::new();
        let mut indices: Vec<i32> = Vec::new();

        for (i, tri) in triangles {
            vertices.push(tri.a.into());
            vertices.push(tri.b.into());
            vertices.push(tri.c.into());

            let i = i as i32;
            indices.push(i * 3);
            indices.push(i * 3 + 1);
            indices.push(i * 3 + 2);
        }

        (vertices, indices)
    }

    fn trimesh_output_to_mesh(verts: Vec<Point<Real>>, tris: Vec<[u32;3]>) -> (Vec<DtVector>, Vec<i32>) {
        let mut vertices: Vec<DtVector> = Vec::new();
        let mut indices: Vec<i32> = Vec::new();
        let mut index = 0;
        for triangle_indices in tris {
            vertices.push(verts[triangle_indices[0] as usize].into());
            vertices.push(verts[triangle_indices[1] as usize].into());
            vertices.push(verts[triangle_indices[2] as usize].into());

            indices.push(index);
            indices.push(index + 1);
            indices.push(index + 2);
            index += 3;
        }
        (vertices, indices)
    }
}