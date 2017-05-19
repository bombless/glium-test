
#[macro_use(implement_vertex)]
extern crate glium;
extern crate svg;

enum DrawEvent {
    Line(Vec<Vertex>)
}

impl DrawEvent {
    fn get_shape(self) -> Vec<Vertex> {
        let DrawEvent::Line(line) = self;
        line
    }
}

impl From<svg::SvgEvent> for DrawEvent {
    fn from(s: svg::SvgEvent) -> Self {
        let svg::SvgEvent::Line {
            x1, x2, y1, y2, view_box
        } = s;
        let width = view_box[2] - view_box[0];
        let height = view_box[3] - view_box[1];
        let x1_ = x1 / width * 2.0 - 1.0;
        let x2_ = x2 / width * 2.0 - 1.0;
        let y1_ = y1 / height * 2.0 - 1.0;
        let y2_ = y2 / height * 2.0 - 1.0;

        let x1_blur = x1_ + 0.003;
        let x2_blur = x2_ + 0.003;
        let y1_blur = y1_ + 0.003;
        let y2_blur = y2_ + 0.003;

        DrawEvent::Line(vec![
             Vertex { position: [x1_ as f32, y1_ as f32] }, Vertex { position: [x1_blur as f32, y1_blur as f32] }, Vertex { position: [x2_ as f32, y2_ as f32] },
             Vertex { position: [x2_blur as f32, y2_blur as f32] }, Vertex { position: [x2_ as f32, y2_ as f32] }, Vertex { position: [x1_ as f32, y1_ as f32] },
        ])
    }
}

fn main() {
    use glium::{DisplayBuild,Surface};
    use std::time::Duration;
    use std::thread::sleep;

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world".to_owned())
        .build_glium()
        .unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    let line = svg::parse(r#"
    <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		<line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	</svg>
    "#).unwrap();
    

    let vertex_buffer = glium::VertexBuffer::new(&display, &line.into_iter().map(DrawEvent::from).map(DrawEvent::get_shape).fold(Vec::new(), |mut acc, x| { acc.extend(x); acc})).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;
    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();

    target.finish().unwrap();

    sleep(Duration::from_secs(2))
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
