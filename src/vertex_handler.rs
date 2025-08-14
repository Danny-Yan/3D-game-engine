use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fs};

// Describes a single vertex
// bytemuck::Pod => Plain old data
// bytemuck::Zeroable => can use std::mem:zeroed()
#[repr(C)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2], // OR color: [f32; 3]
}

// #[repr(C)]
// #[derive(Deserialize, Serialize, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct VertexTexCoord {
//     pub position: [f32; 3],
//     pub tex_coords: [f32; 2],
// }

// #[typetag::serde]
// pub trait VertexKind: std::fmt::Debug {}

// #[typetag::serde]
// impl VertexKind for Vertex {}

// #[typetag::serde]
// impl VertexKind for VertexTexCoord {}

// Implement clone and debug somehow for dynamic trait Box<dyn VertexKind>
#[derive(Deserialize, Debug, Clone)]
pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

#[derive(Deserialize, Debug, Clone)]
struct Container {
    contents: Vec<Shape>,
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        // defined how vertex buffer is represented in memory
        wgpu::VertexBufferLayout {
            // How wide a vertex is (24 bytes in this case)
            // Shader will skip over this many bytes to read the next vertex
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,

            // Describes each elm in array per vertex data or per instance data
            step_mode: wgpu::VertexStepMode::Vertex,

            // Individual parts of the vertex (same as vertex struct)
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,                             // Offset in bytes until next attribute starts
                    shader_location: 0, // @location(0) x: vec3<f32> => position field
                    format: wgpu::VertexFormat::Float32x3, // Shape of attribute in shader code (Float32x3 => vec3<f32>)
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1, // @location(1) x: vec3<f32> => color field
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }

        // Macro for the above code
        // const ATTRIBS: [wgpu::VertexAttribute; 2] =
        //     wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        // fn desc() -> wgpu::VertexBufferLayout<'static> {
        //     use std::mem;

        //     wgpu::VertexBufferLayout {
        //         array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
        //         step_mode: wgpu::VertexStepMode::Vertex,
        //         attributes: &Self::ATTRIBS,
        //     }
        // }
    }
}

pub fn parse_json(path: &str) -> anyhow::Result<Vec<Shape>> {
    let data: &str = &fs::read_to_string(path)?;

    // Parse the string of data into serde_json::Value.
    let container: Container = serde_json::from_str(data)?;
    // println!("Deserialize result: {:?}", &container.contents);
    Ok(container.contents)
}

//  Describes a single triangle made of vertices
// Counter-clockwise (As specified in primitive)
// const VERTICES: &[Vertex] = &[
//     Vertex {
//         position: [0.0, 0.5, 0.0],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, 0.0],
//         color: [0.0, 1.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, 0.0],
//         color: [0.0, 0.0, 1.0],
//     },
// ];
