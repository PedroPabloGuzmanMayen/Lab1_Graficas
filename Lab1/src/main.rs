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

  let width = 1000;
  let height = 1000;
  let mut framebuffer = FrameBuffer::new(width, height);
  framebuffer.set_background_color(Color::new(0,0,0));
  framebuffer.clear();
  framebuffer.set_current_color(Color::new(255,255,255));
  let vec:Vec<[usize; 2]> = vec![[165, 380], [185, 360], [180, 330], [207, 345], [233, 330], [230, 360], [250, 380], [220, 385], [205, 410], [193, 383]
  ];
  framebuffer.polygon(&vec);
  framebuffer.set_current_color(Color::new(255,255,0));
  framebuffer.fill_polygon(&vec);
  let vec2:Vec<[usize; 2]> = vec![[321, 335], [288, 286], [339, 251], [374, 302]];
  let vec3:Vec<[usize; 2]> = vec![[377, 249], [411, 197], [436, 249]];
  framebuffer.polygon(&vec);
  framebuffer.set_current_color(Color::new(255,255,0));
  framebuffer.fill_polygon(&vec);
  framebuffer.set_current_color(Color::new(255,255,255));
  framebuffer.polygon(&vec2);
  framebuffer.set_current_color(Color::new(0,0,255));
  framebuffer.fill_polygon(&vec2);
  framebuffer.set_current_color(Color::new(255,255,255));
  framebuffer.polygon(&vec3);
  framebuffer.set_current_color(Color::new(255,0,0));
  framebuffer.fill_polygon(&vec3);
  framebuffer.write_to_bmp("polygonfill.bmp");

}