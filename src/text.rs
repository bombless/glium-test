extern crate glium_text;

use glium::backend::glutin_backend::GlutinFacade;

pub struct Text<'a> {
  display: &'a GlutinFacade,
  system: glium_text::TextSystem,
  font: &'a glium_text::FontTexture,
}

impl<'a> Text<'a> {
  pub fn new(display: &'a GlutinFacade, font: &'a glium_text::FontTexture) -> Self {
    let system = glium_text::TextSystem::new(display);    
    Text { display, system, font }
  }
  pub fn draw(&self, n: usize) {
    let matrix = [[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]];
    let text = glium_text::TextDisplay::new(&self.system, self.font, &format!("{}", n));
    glium_text::draw(&text, &self.system, &mut self.display.draw(), matrix, (1.0, 1.0, 0.0, 1.0))
  }
}