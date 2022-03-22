use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use vulkano::buffer::{BufferAccess, ImmutableBuffer};
use vulkano::device::{Device, Queue};
pub use vulkano::impl_vertex;
pub use vulkano::pipeline::input_assembly::PrimitiveTopology;
// use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use crate::utils::immutable_slice;
use vulkano::pipeline::depth_stencil::{Compare, DepthStencil};
use vulkano::pipeline::shader::ShaderModule;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::render_pass::{RenderPass, Subpass};

use vulkano::image::view::ImageView;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::viewport::Viewport;
use vulkano::render_pass::{Framebuffer, FramebufferAbstract};
use vulkano::sync::{self, FlushError, GpuFuture};
use vulkano::Version;
use vulkano_win::VkSurfaceBuild;

// use crate::shaders::ShaderSystem;
// use crate::utils::immutable_slice;

use std::any::Any;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Mesh<V: Vertex> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}

pub trait Vertex: vulkano::pipeline::vertex::Vertex + Clone {}

impl<V: vulkano::pipeline::vertex::Vertex + Clone> Vertex for V {}

pub trait MeshAbstract {
    fn get_vbuf(&self, queue: Arc<Queue>) -> Arc<dyn BufferAccess + Send + Sync>;
    fn get_ibuf(&self, queue: Arc<Queue>) -> Arc<ImmutableBuffer<[u32]>>;
    fn get_vtype(&self) -> Arc<dyn VertexTypeAbstract>;
}

impl<V: Vertex> MeshAbstract for Mesh<V> {
    fn get_vbuf(&self, queue: Arc<Queue>) -> Arc<dyn BufferAccess + Send + Sync> {
        immutable_slice(queue, &self.vertices)
    }

    fn get_ibuf(&self, queue: Arc<Queue>) -> Arc<ImmutableBuffer<[u32]>> {
        immutable_slice(queue, &self.indices)
    }

    fn get_vtype(&self) -> Arc<dyn VertexTypeAbstract> {
        Arc::new(VertexType {
            phantom: PhantomData::<V>,
        })
    }
}

#[derive(Clone)]
pub struct VertexType<V: Vertex + Send + Sync + Clone> {
    pub phantom: PhantomData<V>,
}

impl<V: Vertex + Send + Sync + Clone> VertexType<V> {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            phantom: PhantomData::<V>,
        })
    }
}

// TODO: properly implement clone and partialeq
pub trait VertexTypeAbstract: Any {
    fn create_pipeline(
        &self,
        device: Arc<Device>,
        fill_type: PrimitiveTopology,
        render_pass: Arc<RenderPass>,
        read_depth: bool,
        write_depth: bool,
    ) -> Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

    fn clone(&self) -> Arc<dyn VertexTypeAbstract>;
}

impl<V: Vertex + Send + Sync + Clone + 'static> VertexTypeAbstract for VertexType<V> {
    fn create_pipeline(
        &self,
        device: Arc<Device>,
        fill_type: PrimitiveTopology,
        render_pass: Arc<RenderPass>,
        read_depth: bool,
        write_depth: bool,
    ) -> Arc<dyn GraphicsPipelineAbstract + Send + Sync> {
        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        if !read_depth && !write_depth {
            // no depth buffer at all
            Arc::new(
                GraphicsPipeline::start()
                    .vertex_input_single_buffer::<V>()
                    .vertex_shader(vs.main_entry_point(), ())
                    .primitive_topology(fill_type)
                    .viewports_dynamic_scissors_irrelevant(1)
                    .fragment_shader(fs.main_entry_point(), ())
                    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                    .build(device)
                    .unwrap(),
            )
        } else {
            let mut stencil = DepthStencil::disabled();
            stencil.depth_compare = if read_depth {
                Compare::LessOrEqual
            } else {
                Compare::Always
            };
            stencil.depth_write = write_depth;

            Arc::new(
                GraphicsPipeline::start()
                    .vertex_input_single_buffer::<V>()
                    .vertex_shader(vs.main_entry_point(), ())
                    .primitive_topology(fill_type)
                    .viewports_dynamic_scissors_irrelevant(1)
                    .fragment_shader(fs.main_entry_point(), ())
                    .depth_stencil(stencil)
                    .render_pass(Subpass::from(render_pass, 0).unwrap())
                    .build(device)
                    .unwrap(),
            )
        }
    }

    fn clone(&self) -> Arc<dyn VertexTypeAbstract> {
        Arc::new(Self {
            phantom: PhantomData::<V>,
        })
    }
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/../shaders/shaded-teapot/vert.glsl"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/../shaders/shaded-teapot/frag.glsl"
    }
}
