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
//(377, 249) (411, 197) (436, 249)
  let width = 1000;
  let height = 1000;
  let mut framebuffer = FrameBuffer::new(width, height);


    // Clear the framebuffer with a white background
  framebuffer.set_background_color(Color::new(255,255,255));
  framebuffer.clear();

    // Set the current drawing color to black
  framebuffer.set_current_color(Color::new(0,200,0));

    // Draw some lines using Bresenham's algorithm
  let vec:Vec<[usize; 2]> = vec![[413, 177], [448, 159], [502, 88], [553, 53], [535, 36], [676, 37], [660, 52], [750, 145], [761, 179], [672, 192], [659, 214], [615, 214], [632, 230], [580, 230], [597, 215], [552, 214], [517, 144], [466, 180]


  ];
  let vec2:Vec<[usize; 2]> = vec![[413, 177], [448, 159], [502, 88], [553, 53], [535, 36], [676, 37], [660, 52], [750, 145], [761, 179], [672, 192], [659, 214], [615, 214], [632, 230], [580, 230], [597, 215], [552, 214], [517, 144], [466, 180]


  ];

  let vec3:Vec<[usize; 2]> = vec![[682, 175], [708, 120], [735, 148], [739, 170]];
  let vec4:Vec<[usize; 2]> = vec![[682, 175], [708, 120], [735, 148], [739, 170]];
  
  framebuffer.polygon(vec);
  //framebuffer.polygon(vec3);
  framebuffer.fill_polygon(vec2);
  framebuffer.write_to_bmp("polygonfill.bmp");

}
