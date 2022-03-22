use nalgebra_glm::*;
use produ_render::collection::Data;
use produ_render::mesh::PrimitiveTopology;
use produ_render::mesh::{self, Mesh};
use produ_render::object::ObjectPrototype;
use produ_render::render_passes;
use produ_render::system::{Pass, System};
use produ_render::test_teapot::{INDICES, NORMALS, VERTICES};
use produ_render::utils::relative_path;
use produ_render::window::Window;
use std::collections::HashMap;
use tobj;
use vulkano::pipeline::vertex::Vertex;
fn main() {
    let (mut window, queue) = Window::new();
    let device = queue.device().clone();
    let render_pass = render_passes::multisampled_with_depth(device.clone(), 4);

    let mut system = System::new(
        queue.clone(),
        vec![Pass {
            name: "geometry",
            images_created_tags: vec!["resolve_color", "multisampeld_color", "multisampled_depth"],
            images_needed_tags: vec![],
            render_pass: render_pass.clone(),
        }],
        // custom images, we use none
        HashMap::new(),
        "resolve_color",
    );

    window.set_render_pass(render_pass.clone());

    // create buffers for model matrix, light and materials
    let model_data: Mat4 = translate(&Mat4::identity(), &vec3(0.0, -6.0, 0.0)).into();

    let mut light = Light {
        position: [10.0, 0.0, 0.0, 0.0],
        ambient: [0.3, 0.3, 0.3, 0.0],
        diffuse: [1.3, 1.3, 1.3, 0.0],
        specular: [1.5, 1.5, 1.5, 0.0],
    };

    let material_data = Material { shininess: 76.8 };

    // let (mut models, _materials) =
    //     load_obj(&relative_path("meshes/raptor.obj")).expect("couldn't load OBJ");
    // let basic_mesh = convert_meshes(&[models.remove(0)]).remove(0);
    // let mesh = add_tangents(&basic_mesh);

    // let start_time = std::time::Instant::now();

    // let diffuse_texture = load_texture(
    //     queue.clone(),
    //     &relative_path("textures/raptor-diffuse.png"),
    //     Format::R8G8B8A8Srgb,
    // );
    // let specular_texture = load_texture(
    //     queue.clone(),
    //     &relative_path("textures/raptor-specular.png"),
    //     Format::R8G8B8A8Unorm,
    // );
    // let normal_texture = load_texture(
    //     queue.clone(),
    //     &relative_path("textures/raptor-normal.png"),
    //     Format::R8G8B8A8Unorm,
    // );
    // println!("Time taken to load textures: {}s", get_elapsed(start_time));
    let vertices = VERTICES.iter().collect();
    let normals = NORMALS.iter().collect();
    let indices = INDICES.iter().map(|index| index).collect();
    let mesh = tobj::load_obj(file_name, load_options)
    let mesh1 = tobjMesh {
        positions: vertices,
        vertex_color: vec![],
        texcoords: vec![],
        indices: indices.clone(),
        face_arities: vec![],
        normals: vec![],
        texcoord_indices: vec![],
        normal_indices: vec![],
        material_id: None,
    };
    let mesh = Mesh {
        vertices,
        indices: indices.clone(),
    };

    let mut object = ObjectPrototype {
        vs_path: relative_path("shaders/lighting/object_vert.glsl"),
        fs_path: relative_path("shaders/lighting/object_frag.glsl"),
        fill_type: PrimitiveTopology::TriangleList,
        read_depth: true,
        write_depth: true,
        mesh,

        // 00 model 01 material 10 camera 20 light 30 diff 31 spec 32 norm
        collection: (
            (model_data, material_data),
            // (camera_data,),
            (light.clone(),),
        ),
        custom_dynamic_state: None,
    }
    .build(queue.clone(), render_pass.clone());

    println!("Hello, World");

    while !window.update() {
        // // update camera and camera buffer
        // camera.update(window.get_frame_info());
        // let camera_data = camera.get_data();

        // // update light
        // let time = get_elapsed(start_time);
        // let light_x = (time / 4.0).sin() * 20.0;
        // let light_z = (time / 4.0).cos() * 20.0;
        // light.position = [light_x, 0.0, light_z, 0.0];

        // object.collection.1.data.0 = camera_data;
        // object.collection.2.data.0 = light.clone();

        // object.collection.1.upload(device.clone());
        // object.collection.2.upload(device.clone());

        // draw
        system.start_window(&mut window);
        // system.add_object(&object);
        // system.finish_to_window(&mut window);
    }
}

#[derive(Clone, Copy)]
struct Light {
    position: [f32; 4],
    ambient: [f32; 4],
    diffuse: [f32; 4],
    specular: [f32; 4],
}

#[derive(Clone, Copy)]
struct Material {
    shininess: f32,
}

impl Data for Light {}
impl Data for Material {}
