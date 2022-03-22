use std::sync::Arc;

use crate::input::{EventHandler, FrameInfo};
use crate::utils::Timer;
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::image::SwapchainImage;
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::render_pass::RenderPass;
use vulkano::swapchain::Surface;
use vulkano::sync::GpuFuture;
use vulkano::Version;
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::render_passes;
use crate::vk_window::VkWindow;
// Todo: Add event handler for input
pub struct Window {
    vk_window: VkWindow,
    queue: Arc<Queue>,
    render_pass: Arc<RenderPass>,
    recenter: bool,
    update_timer: Timer,
    event_handler: EventHandler,
}

impl Window {
    pub fn new() -> (Self, Arc<Queue>) {
        let instance = get_instance();

        let queue = get_queue(instance.clone());
        let device = queue.device().clone();

        let event_loop = EventLoop::new();
        let event_handler = EventHandler::new(events_loop);
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .expect("Expected to create a window for vulkan instance, (‡▼益▼)");

        // surface.window().hide_cursor(true);

        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
        let swapchain_caps = surface.capabilities(physical).unwrap();

        let render_pass = render_passes::basic(device.clone());

        let vk_window = VkWindow::new(
            queue.device().clone(),
            queue.clone(),
            surface.clone(),
            render_pass.clone(),
            swapchain_caps.clone(),
        );

        let window = Self {
            vk_window,
            queue: queue.clone(),
            render_pass,
            recenter: true,
            update_timer: Timer::new("Avg. time to update window"),
        };

        (window, queue)
    }

    pub fn present_future<F: GpuFuture + 'static>(&mut self, future: F) {
        self.vk_window.present_image(self.queue.clone(), future)
    }

    pub fn next_image(&mut self) -> Arc<SwapchainImage<winit::window::Window>> {
        self.vk_window.next_image()
    }

    pub fn get_future(&mut self) -> Box<dyn GpuFuture> {
        self.vk_window.get_future()
    }

    pub fn get_surface(&self) -> Arc<Surface<winit::window::Window>> {
        self.vk_window.get_surface()
    }

    pub fn set_recenter(&mut self, state: bool) {
        self.recenter = state;
    }

    fn recenter_cursor(&mut self) {
        let dimensions = self.get_dimensions();

        self.vk_window
            .get_surface()
            .window()
            .set_cursor_position(winit::dpi::LogicalPosition {
                x: (dimensions[0] as f64) / 2.0,
                y: (dimensions[1] as f64) / 2.0,
            })
            .expect("Couldn't re-set cursor position!");
    }

    pub fn get_dimensions(&self) -> [u32; 2] {
        self.vk_window.get_dimensions()
    }

    pub fn set_render_pass(&mut self, new_render_pass: Arc<RenderPass>) {
        self.vk_window.set_render_pass(new_render_pass);
        self.vk_window.rebuild();
    }

    pub fn update(&mut self) -> bool {
        self.update_timer.start();

        // returns whether to exit the program or not
        // TODO: return an enum or move the done-checking to its own function
        let done = self.event_handler.update(self.get_dimensions());
        if self.recenter {
            self.recenter_cursor();
        }

        self.update_timer.stop();

        done
    }
}

fn get_instance() -> Arc<Instance> {
    //  This gives us the Required Extensions to Start a window"
    let required_extensions = vulkano_win::required_extensions();
    println!(
        "Required Extensions to Start a window {:?}",
        required_extensions
    );

    let instance = Instance::new(None, Version::V1_1, &required_extensions, None)
        .expect("Failed to create vulkan instance ");
    println!("Our Instance: {:?}", instance);
    return instance;
}

fn get_queue(instance: Arc<Instance>) -> Arc<Queue> {
    let physical_device = PhysicalDevice::from_index(&instance, 0).unwrap();
    let family_friendly_queues = physical_device
        .queue_families()
        .find(|&q| q.supports_graphics())
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
    return queue;
}
