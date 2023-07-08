use bytemuck::{Pod, Zeroable};
use vulkano::pipeline::graphics::vertex_input::Vertex as VVertex;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod, VVertex)]
pub struct Vertex {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
}
