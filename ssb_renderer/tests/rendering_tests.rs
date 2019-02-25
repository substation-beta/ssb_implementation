#[cfg(test)]
mod rendering_tests {
    // Imports
    use ssb_renderer::gl_utils::{safe::*, macros::build_program, environment::{GlEnvironment, ColorType, OffscreenContext}, error::GlError};
    use image::{RgbImage, RgbaImage};
    use std::path::Path;

    // TRIANGLE
    // Test resources
    const TRIANGLE_VERTEX_DATA: [f32; 6] = [
        0.0, -0.5,
        -0.5, 0.5,
        0.5, 0.5,
    ];
    const TRIANGLE_VERTEX_SHADER: &str = "#version 150 core

in vec2 position;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
}";
    const TRIANGLE_FRAGMENT_SHADER: &str = "#version 150 core

out vec4 color;

void main()
{
    color = vec4(1.0, 1.0, 1.0, 1.0);
}";

    fn draw_triangle(img_size: (u32, u32)) -> RgbImage {
        // Create empty image
        let img = RgbImage::new(img_size.0, img_size.1);
        // Unpack image
        let width = img.width();
        let height = img.height();
        let mut buffer: Vec<u8> = img.into_raw();
        // Render on image with offscreen rendering context
        OffscreenContext::new(width as u16, height as u16, ColorType::RGB, env!("SAMPLES").parse::<u8>().expect("SAMPLES environment variable not a number!"))
        .expect("Offscreen context required!")
        .process(&mut buffer, || {
            // Link shader program
            let shader_program = build_program(&TRIANGLE_VERTEX_SHADER, &TRIANGLE_FRAGMENT_SHADER);
            shader_program.using();
            // Create vertex attribute storage (required by GL core profile!)
            let vao = VAO::generate();
            vao.bind();
            // Bind vertex data
            let vbo = VBO::generate();
            vbo.bind();
            VBO::data(&TRIANGLE_VERTEX_DATA);
            // Interpret vertex data
            let position_location = shader_program.attrib_location("position");
            VBO::enable_attrib_array(position_location as u32);
            VBO::attrib_pointer(position_location as u32, 2, 0, 0);
            // Draw!
            VBO::draw_arrays(gl32::TRIANGLES, 0, 3);
        }).ok();
        // Return processed image
        return RgbImage::from_raw(width, height, buffer).expect("Image repackaging failed!");
    }

    // Tester
    #[test]
    fn test_triangle_image() {
        // Create OpenGL capable environment
        GlEnvironment::new((3, 2), draw_triangle)
            // Process new image in environment
            .process((800, 800)).expect("Drawing failed!")
            // Save image to disk
            .save(
                Path::new(&env!("CARGO_MANIFEST_DIR"))
                .join("../target/triangle_image.png")
            ).expect("Image saving failed!");
    }

    // QUAD (colored)
    // Test resources
    const QUAD_VERTEX_DATA: [f32; 24] = [
        // Pos     Color
        -0.5, 0.5, 1.0, 0.0, 0.0, 0.5,
        0.5, 0.5, 0.0, 1.0, 0.0, 1.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.5,
        -0.5, -0.5, 1.0, 1.0, 0.0, 1.0
    ];
    const QUAD_INDICES: [u32; 6] = [
        0, 1, 2,
        2, 3, 0
    ];
    const QUAD_VERTEX_SHADER: &str = "#version 150 core

in vec2 position;
in vec4 color;
out vec4 vertex_color;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
    vertex_color = color;
}";
    const QUAD_FRAGMENT_SHADER: &str = "#version 150 core

in vec4 vertex_color;
out vec4 fragment_color;

void main()
{
    fragment_color = vertex_color;
}";

    fn draw_quad(img: RgbaImage) -> RgbaImage {
        // Unpack image
        let width = img.width();
        let height = img.height();
        let mut buffer: Vec<u8> = img.into_raw();
        // Render on image with offscreen rendering context
        OffscreenContext::new(width as u16, height as u16, ColorType::RGBA, env!("SAMPLES").parse::<u8>().expect("SAMPLES environment variable not a number!"))
        .expect("Offscreen context required!")
        .process(&mut buffer, || {
            // Link shader program
            let shader_program = build_program(&QUAD_VERTEX_SHADER, &QUAD_FRAGMENT_SHADER);
            shader_program.using();
            // Create vertex attribute storage (required by GL core profile!)
            let vao = VAO::generate();
            vao.bind();
            // Bind vertex data
            let vbo = VBO::generate();
            vbo.bind();
            VBO::data(&QUAD_VERTEX_DATA);
            // Interpret vertex data
            let position_location = shader_program.attrib_location("position");
            VBO::enable_attrib_array(position_location as u32);
            VBO::attrib_pointer(position_location as u32, 2/*vec2*/, 6/*skip current position+color for next entry*/, 0/*start with first data value*/);
            let color_location = shader_program.attrib_location("color");
            VBO::enable_attrib_array(color_location as u32);
            VBO::attrib_pointer(color_location as u32, 4, 6, 2);
            // Bind indices
            let ebo = EBO::generate();
            ebo.bind();
            EBO::data(&QUAD_INDICES);
            // Enable blending
            Enable(gl32::BLEND);
            BlendFuncSeparate(gl32::SRC_ALPHA, gl32::ONE_MINUS_SRC_ALPHA, gl32::ZERO, gl32::ONE);
            BlendEquation(gl32::FUNC_ADD);
            // Draw!
            EBO::draw_elements(gl32::TRIANGLES, 6);
            if let Some(err) = GlError::from_gl() {
                panic!("draw_elements: {}", err);
            }
        }).ok();
        // Return processed image
        return RgbaImage::from_raw(width, height, buffer).expect("Image repackaging failed!");
    }

    // Tester
    #[test]
    fn test_quad_image() {
        // Get manifest directory
        let dir = env!("CARGO_MANIFEST_DIR");
        // Load image
        let sample_image = image::open(
            Path::new(&dir)
            .join("tests/ayaya.png")
        ).expect("Couldn't load sample image!").to_rgba();
        // Draw on image
        let quad_image = GlEnvironment::new((3, 2), draw_quad).process(sample_image).expect("Drawing failed!");
        // Save image
        quad_image.save(
            Path::new(&dir)
            .join("../target/quad_image.png")
        ).expect("Image saving failed!");
    }
}