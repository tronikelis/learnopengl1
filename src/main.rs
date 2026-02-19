use std::f32::consts::PI as PIF32;

use glfw::{Action, Context, Key, fail_on_errors};
use nalgebra_glm as glm;

const VERTEX_SHADER_SRC: &'static str = r#"
    #version 330 core

    layout (location = 0) in vec3 inPosition;
    layout (location = 1) in vec2 inTextureCoordinates;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    out vec2 outTextureCoordinates;

    void main() {
        gl_Position = projection * view * model * vec4(inPosition, 1.0);
        outTextureCoordinates = inTextureCoordinates;
    };
"#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 330 core

    in vec2 outTextureCoordinates;

    uniform sampler2D ourTexture;

    out vec4 fragColor;

    void main() {
       fragColor = texture(ourTexture, outTextureCoordinates);
    };
"#;

/// converts degrees to radians
fn radians(degrees: f32) -> f32 {
    degrees * PIF32 / 180.0
}

fn main() {
    let mut glfw_init = glfw::init(glfw::fail_on_errors!()).unwrap();

    glfw_init.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw_init.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw_init.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw_init
        .create_window(
            1280,
            720,
            "Hello this is window",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_resizable(true);
    window.set_framebuffer_size_callback(|window, width, height| {
        opengl::gl_viewport(0, 0, width, height);
    });

    opengl::gl_enable(opengl::ffi::GL_DEPTH_TEST);

    let container_image = image::ImageReader::open("./resources/container.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let texture = opengl::Texture::new(opengl::ffi::GL_TEXTURE_2D);
    texture.bind();

    texture.image_2d(
        0,
        opengl::ffi::GL_RGB,
        container_image.width() as _,
        container_image.height() as _,
        opengl::ffi::GL_RGB,
        opengl::ffi::GL_UNSIGNED_BYTE,
        container_image.to_rgb8().as_ptr(),
    );
    texture.generate_mipmap();

    let vertex_shader = opengl::Shader::new(opengl::ffi::GL_VERTEX_SHADER)
        .source(&[VERTEX_SHADER_SRC])
        .compile()
        .unwrap();
    let fragment_shader = opengl::Shader::new(opengl::ffi::GL_FRAGMENT_SHADER)
        .source(&[FRAGMENT_SHADER_SRC])
        .compile()
        .unwrap();

    let mut program = opengl::Program::new();
    program.attach_shader(vertex_shader);
    program.attach_shader(fragment_shader);

    program.link_program().unwrap();
    program.use_program();

    let vao = opengl::VertexArray::new();
    vao.bind();

    // [x,y,z, tx, ty]
    let vertices: [f32; _] = [
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5,
        -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5,
        0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5,
        0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, -0.5,
        1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0,
        0.0, -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
        -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5,
        1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0,
        0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5,
        -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0,
        -0.5, 0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
    ];

    let vbo = opengl::Buffer::new(opengl::ffi::GL_ARRAY_BUFFER);
    vbo.bind();
    vbo.data(&vertices, opengl::ffi::GL_STATIC_DRAW);

    // vertices
    opengl::Buffer::vertex_attrib_pointer(
        0,
        3,
        opengl::ffi::GL_FLOAT,
        false,
        (5 * size_of::<f32>()) as _,
        0,
    );
    opengl::Buffer::enable_vertex_attrib_array(0);

    // texture coordinates
    opengl::Buffer::vertex_attrib_pointer(
        1,
        2,
        opengl::ffi::GL_FLOAT,
        false,
        (5 * size_of::<f32>()) as _,
        3 * size_of::<f32>(),
    );
    opengl::Buffer::enable_vertex_attrib_array(1);

    let cubes = [
        glm::vec3(0.0_f32, 0.0, 0.0),
        glm::vec3(2.0_f32, 5.0, -15.0),
        glm::vec3(-1.5_f32, -2.2, -2.5),
        glm::vec3(-3.8_f32, -2.0, -12.3),
        glm::vec3(2.4_f32, -0.4, -3.5),
        glm::vec3(-1.7_f32, 3.0, -7.5),
        glm::vec3(1.3_f32, -2.0, -2.5),
        glm::vec3(1.5_f32, 2.0, -2.5),
        glm::vec3(1.5_f32, 0.2, -1.5),
        glm::vec3(-1.3_f32, 1.0, -1.5),
    ];

    let mut camera_pos = glm::vec3::<f32>(0.0, 0.0, 3.0);
    let camera_front = glm::vec3::<f32>(0.0, 0.0, -1.0);
    let camera_up = glm::vec3::<f32>(0.0, 1.0, 0.0);

    let yaw: f32 = -90.0;
    let pitch: f32 = 0.0;

    let direction = glm::vec3::<f32>(radians(yaw).cos(), radians(yaw).sin(), radians(pitch).sin());

    let mut last_frame: f32 = 0.0;
    let mut delta_time: f32 = 0.0;

    while !window.should_close() {
        let current_time = glfw_init.get_time() as f32;
        delta_time = current_time - last_frame;
        last_frame = current_time;

        let camera_speed: f32 = 10.0 * delta_time;

        glfw_init.poll_events();
        match window.get_key(Key::E) {
            Action::Press => {
                camera_pos += camera_front * camera_speed;
            }
            _ => {}
        }
        match window.get_key(Key::D) {
            Action::Press => {
                camera_pos -= camera_front * camera_speed;
            }
            _ => {}
        }
        match window.get_key(Key::F) {
            Action::Press => {
                camera_pos += -camera_up.cross(&camera_front).normalize() * camera_speed;
            }
            _ => {}
        }
        match window.get_key(Key::S) {
            Action::Press => {
                camera_pos -= -camera_up.cross(&camera_front).normalize() * camera_speed;
            }
            _ => {}
        }
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }

        let viewport = opengl::get_viewport();

        opengl::gl_clear_color(0.2, 0.3, 0.3, 1.0);
        opengl::gl_clear(opengl::ffi::GL_COLOR_BUFFER_BIT | opengl::ffi::GL_DEPTH_BUFFER_BIT);

        let projection_matrix = glm::perspective(
            viewport[2] as f32 / viewport[3] as f32,
            radians(60.0),
            0.1,
            100.0,
        );
        let view_matrix = glm::look_at(&camera_pos, &(camera_pos + camera_front), &camera_up);

        for (i, cube) in cubes.iter().enumerate() {
            let model_matrix = glm::translate(&glm::identity(), &cube);
            let model_matrix = glm::rotate(
                &model_matrix,
                radians(-70.0 * (i + 1) as f32) * glfw_init.get_time() as f32,
                &glm::vec3(1.0, 0.5, 0.0),
            );

            program.use_program();
            program.set_uniform_matrix4fv("model", false, &[model_matrix.as_slice()]);
            program.set_uniform_matrix4fv("view", false, &[view_matrix.as_slice()]);
            program.set_uniform_matrix4fv("projection", false, &[projection_matrix.as_slice()]);

            vao.bind();
            opengl::draw_arrays(opengl::ffi::GL_TRIANGLES, 0, 36);
        }

        window.swap_buffers();
    }
}
