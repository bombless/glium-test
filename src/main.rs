#[macro_use]
extern crate glium;
extern crate svg_now;
extern crate svg;


fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let svg_src = r#"
    <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		<line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	</svg>
    "#;
    let tex = svg_now::render((100, 100), svg::parse(svg_src).unwrap());
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(tex, (100, 100));
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ 1.0, -1.0], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] };

    let vertex4 = Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] };
    let vertex5 = Vertex { position: [-1.0,  1.0], tex_coords: [1.0, 0.0] };
    let vertex6 = Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let uniforms = uniform! { tex: &texture };

    loop {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
