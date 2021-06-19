use std::sync::Arc;
use vulkano::buffer::{cpu_access::CpuAccessibleBuffer, BufferUsage};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, DynamicState, SubpassContents,
};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::image::view::ImageView;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::{Framebuffer, FramebufferAbstract, RenderPass, Subpass};
use vulkano::swapchain::{
    self, AcquireError, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
    SwapchainCreationError,
};
use vulkano::sync::{self, FlushError, GpuFuture};
use vulkano::Version;
use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
fn main() {
    //?================================================ Config Viewport ================================================
    let viewport_origin: [f32; 2] = [0.0, 0.0];

    //?================================================= Main Function =================================================
    //  This gives us the Required Extensions to Start a window"
    let required_extensions = vulkano_win::required_extensions();
    println!(
        "Required Extensions to Start a window {:?}",
        required_extensions
    );

    let instance = Instance::new(None, Version::V1_1, &required_extensions, None)
        .expect("Failed to create vulkan instance ");
    println!("Our Instance: {:?}", instance);

    // find the name of the gpu's
    println!("Here are the devices we found");
    let physical_device = PhysicalDevice::from_index(&instance, 0).unwrap();
    println!(
        "Name: {}",
        physical_device.properties().device_name.as_ref().unwrap()
    );

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .expect("Expected to create a window for vulkan instance, (‡▼益▼)");

    let family_friendly_queues = physical_device
        .queue_families()
        .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
        .unwrap();
    println!("Picked Queue: {:?}", family_friendly_queues);

    let device_ext = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    println!("Device Extensions: {:?}", device_ext);

    let (device, mut queues) = Device::new(
        physical_device,
        physical_device.supported_features(),
        &device_ext,
        [(family_friendly_queues, 0.5)].iter().cloned(),
    )
    .unwrap();
    let queue = queues.next().unwrap();
    println!("queue: {:?}", queue);

    //? Time for a swapchain 😭
    let (mut swapchain, images) = {
        let capabilities = surface.capabilities(physical_device).unwrap();
        let alpha = capabilities
            .supported_composite_alpha
            .iter()
            .next()
            .unwrap();
        let format = capabilities.supported_formats[0].0;
        let dimensions: [u32; 2] = surface.window().inner_size().into();
        Swapchain::start(
            // Create the swapchain in this `device`'s memory.
            device.clone(), // The surface where the images will be presented.
            surface.clone(),
        )
        // How many buffers to use in the swapchain.
        .num_images(capabilities.min_image_count)
        // The format of the images.
        .format(format)
        // The size of each image.
        .dimensions(dimensions)
        // What the images are going to be used for.
        .usage(ImageUsage::color_attachment())
        // What transformation to use with the surface.
        .transform(SurfaceTransform::Identity)
        // How to handle the alpha channel.
        .composite_alpha(alpha)
        // How to present images.
        .present_mode(PresentMode::Fifo)
        // How to handle fullscreen exclusivity
        .fullscreen_exclusive(FullscreenExclusive::Default)
        .build()
        .unwrap()
    };

    // println!("swapchain: {:?}", swapchain);

    let vertex_buffer = {
        #[derive(Default, Debug, Clone)]
        struct Vertex {
            position: [f32; 2],
        }
        vulkano::impl_vertex!(Vertex, position);

        CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            [
                Vertex {
                    position: [-0.5, -0.5],
                },
                Vertex {
                    position: [0.5, -0.5],
                },
                Vertex {
                    position: [0.0, 0.5],
                },
            ]
            .iter()
            .cloned(),
        )
        .unwrap()
    };
    //println!("Buffer: {:?}", vertex_buffer);

    mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: "
                #version 450
                
                layout(location = 0) in vec2 position;
                layout(location = 1) in vec4 vertex;
                
                void main() {
                    gl_Position = vec4(position,0.0, 1.0);
                }"
        }
    }

    mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            src: "
                    #version 450
                    
                    layout(location = 0) out vec4 f_color;
                    in vec4 vertex_color;
                    
                    void main() {
                        f_color = vec4(103.0, 58.0, 183.0, 1.0);
                    }"
        }
    }

    let vs = vs::Shader::load(device.clone()).unwrap();
    let fs = fs::Shader::load(device.clone()).unwrap();
    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(device.clone(),
         attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            }
        }, pass: {
            color: [color],
            depth_stencil: {}

        })
        .unwrap(),
    );

    // println!("Render Pass: {:?}", render_pass);

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    // println!("Pipeline: {:?}", pipeline);

    let mut dynamic_state = DynamicState {
        line_width: None,
        viewports: None,
        scissors: None,
        compare_mask: None,
        write_mask: None,
        reference: None,
    };

    println!("Dynamic State: {:?}", dynamic_state);

    let mut framebuffers = window_size_dependent_setup(
        &images,
        render_pass.clone(),
        &mut dynamic_state,
        viewport_origin,
    );

    let mut recreate_swapchain = false;

    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            recreate_swapchain = true;
        }
        Event::RedrawEventsCleared => {
            previous_frame_end.as_mut().unwrap().cleanup_finished();

            if recreate_swapchain {
                let dimensions: [u32; 2] = surface.window().inner_size().into();
                let (new_swapchain, new_images) =
                    match swapchain.recreate().dimensions(dimensions).build() {
                        Ok(r) => r,
                        Err(SwapchainCreationError::UnsupportedDimensions) => return,
                        Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                    };
                swapchain = new_swapchain;
                framebuffers = window_size_dependent_setup(
                    &new_images,
                    render_pass.clone(),
                    &mut dynamic_state,
                    viewport_origin,
                );
                recreate_swapchain = false;
            }

            let (image_num, suboptimal, aquire_future) =
                match swapchain::acquire_next_image(swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("Failed to acquire next image: {:?}", e),
                };

            if suboptimal {
                recreate_swapchain = true;
            }

            let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into()];

            let mut builder = AutoCommandBufferBuilder::primary(
                device.clone(),
                queue.family(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();

            builder
                .begin_render_pass(
                    framebuffers[image_num].clone(),
                    SubpassContents::Inline,
                    clear_values,
                )
                .unwrap()
                .draw(
                    pipeline.clone(),
                    &dynamic_state,
                    vertex_buffer.clone(),
                    (),
                    (),
                    vec![],
                )
                .unwrap()
                .end_render_pass()
                .unwrap();

            let command_buffer = builder.build().unwrap();

            let future = previous_frame_end
                .take()
                .unwrap()
                .join(aquire_future)
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                .then_signal_fence_and_flush();

            match future {
                Ok(future) => {
                    previous_frame_end = Some(future.boxed());
                }
                Err(FlushError::OutOfDate) => {
                    recreate_swapchain = true;
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
                Err(e) => {
                    println!("Failed to flush future: {:?}", e);
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
            }
        }
        _ => (),
    })
}

fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPass>,
    dynamic_state: &mut DynamicState,
    viewport_origin: [f32; 2],
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();
    let viewport = Viewport {
        origin: viewport_origin,
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            let view = ImageView::new(image.clone()).unwrap();
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(view)
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}
