mod framebuffer;
mod line;
mod color;
mod bmp;
mod polygon;
use crate::framebuffer::FrameBuffer;
use crate::line::Line;
use crate::color::Color;
use crate::polygon::{Polygon, get_max_limits};

fn main() {

  let width = 100;
  let height = 100;
  let mut framebuffer = FrameBuffer::new(width, height);
  framebuffer.set_current_color(Color::new(0,255,0));
  framebuffer.point(50,50);
  framebuffer.write_to_bmp("polygonfill.bmp");

}
