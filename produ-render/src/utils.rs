use crate::input::get_elapsed;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, ImmutableBuffer};
use vulkano::device::{Device, Queue};
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{ImageDimensions, ImmutableImage};
use vulkano::memory::Content;
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};
use vulkano::sync::GpuFuture;

pub fn relative_path(local_path: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), local_path].iter().collect()
}

// pub fn load_texture(
//     queue: Arc<Queue>,
//     path: &Path,
//     format: Format,
// ) -> Arc<dyn ImageView + Send + Sync> {
//     let (texture, tex_future) = {
//         let image = image::open(path).unwrap().to_rgba();
//         let (width, height) = image.dimensions();
//         let image_data = image.into_raw().clone();

//         ImmutableImage::from_iter(
//             image_data.iter().cloned(),
//             ImageDimensions::Dim2d { width, height },
//             format,
//             queue.clone(),
//         )
//         .unwrap()
//     };

//     tex_future
//         .then_signal_fence_and_flush()
//         .unwrap()
//         .wait(None)
//         .unwrap();

//     texture
// }

pub fn default_sampler(device: Arc<Device>) -> Arc<Sampler> {
    Sampler::new(
        device,
        Filter::Linear,
        Filter::Linear,
        MipmapMode::Nearest,
        SamplerAddressMode::Repeat,
        SamplerAddressMode::Repeat,
        SamplerAddressMode::Repeat,
        0.0,
        1.0,
        0.0,
        0.0,
    )
    .unwrap()
}

pub fn upload_data<T: Content + 'static + Send + Sync + Copy>(
    device: Arc<Device>,
    data: T,
) -> Arc<CpuAccessibleBuffer<T>> {
    CpuAccessibleBuffer::from_data(device, BufferUsage::all(), false, data).unwrap()
}

pub fn immutable_slice<T: Content + 'static + Send + Sync + Clone>(
    queue: Arc<Queue>,
    slice: &[T],
) -> Arc<ImmutableBuffer<[T]>> {
    // The only reason this isn't used everywhere in render-engine is that
    // creating an immutable buffer also creates a command buffer to copy data
    // into it for some reason, which slows down command buffer submission.
    // Therefore, you should use this for things that are uploaded once, and use
    // upload_data for things you upload every frame (even if you don't mutate
    // between uploads).
    ImmutableBuffer::from_iter(slice.iter().cloned(), BufferUsage::all(), queue)
        .unwrap()
        .0
}

// used for averaging times for benchmarks
// TODO: rename this to AverageTimer to make the difference between it and
// Stopwatch clearer
pub struct Timer {
    name: String,
    total_time: f32,
    samples: u32,
    last_start_time: Instant,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_time: 0.0,
            samples: 0,
            last_start_time: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        // starts the timer.
        self.last_start_time = Instant::now();
    }

    pub fn stop(&mut self) {
        // stops the timer and adds this sample to the totals
        self.total_time += get_elapsed(self.last_start_time);
        self.samples += 1;
    }

    pub fn print(&self) {
        // prints average time taken
        println!(
            "{}: {} ms",
            self.name,
            self.total_time / (self.samples as f32) * 1_000.0
        );
    }
}