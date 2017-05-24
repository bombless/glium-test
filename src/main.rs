#[macro_use]
extern crate glium;
extern crate svg_now;
extern crate svg;
extern crate glium_text;

pub struct View {
    x: f32, w: f32, y: f32, h: f32
}

impl View {
    pub fn with(&self, val: [f32; 2]) -> [f32; 2] {
        let x = val[0];
        let y = val[1];
        let re_x = (x - -1.0) / 2.0;
        let re_y = (y - -1.0) / 2.0;
        [self.x + re_x * self.w, self.y + re_y * self.h]
    }
    pub fn new(x: f32, w: f32, y: f32, h: f32) -> Self {
        View { x, w, y, h }
    }
}


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn verticies(pos: usize) -> Vec<Vertex> {
    let view = position(pos);
    let view = View::new(view[0], view[1], view[2], view[3]);

    let vertex1 = Vertex { position: view.with([-0.9, -0.9]), tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: view.with([ 0.9, -0.9]), tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: view.with([ 0.9,  0.9]), tex_coords: [1.0, 1.0] };

    let vertex4 = Vertex { position: view.with([ 0.9,  0.9]), tex_coords: [1.0, 1.0] };
    let vertex5 = Vertex { position: view.with([-0.9,  0.9]), tex_coords: [1.0, 0.0] };
    let vertex6 = Vertex { position: view.with([-0.9, -0.9]), tex_coords: [0.0, 0.0] };
    vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]
}

fn position(idx: usize) -> [f32; 4] {
    assert!(idx < 12);
    let row = idx / 3;
    let col = idx % 3;
    [-1.0 + 2.0 / 4.0 * row as f32, 2.0/4.0, -1.0 + 2.0 / 3.0 * col as f32, 2.0/3.0]
}

fn main() {
    use glium::{DisplayBuild, Surface};
    let window = glium::glutin::WindowBuilder::new().with_dimensions(400, 300);
    let display = window.build_glium().unwrap();

    
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let font = glium_text::FontTexture::new(&display, std::fs::File::open(&std::path::Path::new("c:/Windows/Fonts/simsunb.ttf")).unwrap(), 24).unwrap();
    println!("font ready");

    let text = text::Text::new(&display, &font);
    println!("text ready");

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

    
    const DATA: &'static [&'static str] = &[
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
           <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="15" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="15" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="15" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="15" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="15" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="15" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="15" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="30" x2="15" y2="15" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="15" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="15" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="15" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="15" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>
            "#,
            ""
    ];

    loop {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        for (idx, svg) in DATA.iter().enumerate() {
            let shape = verticies(idx);
            let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

            let image = if svg.len() > 0 {
                let events = svg::parse(svg).unwrap();            
                let tex = svg_now::render((100, 100), events);
                glium::texture::RawImage2d::from_raw_rgba_reversed(tex, (100, 100))
            } else {
                const GREY: u8 = 12 * 16 + 12;
                glium::texture::RawImage2d::from_raw_rgba_reversed(vec![GREY, GREY, GREY, 255], (1, 1))
            };
            let texture = glium::texture::Texture2d::new(&display, image).unwrap();
            let uniforms = uniform! { tex: &texture };
            target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        }

        text.draw(1);
        println!("text drawed");
        
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

mod text;
