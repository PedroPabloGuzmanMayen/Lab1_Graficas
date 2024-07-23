use crate::framebuffer::FrameBuffer;  
use crate::line::Line;

pub trait Polygon {
    fn polygon(&mut self, vec: Vec<[usize; 2]>);
    fn draw_filled_polygon(&mut self, vec: Vec<[usize; 2]>);
}

impl Polygon for FrameBuffer {
    fn polygon(&mut self, vec: Vec<[usize; 2]>){

        for i in 0..vec.len() {
            if i == (vec.len()-1) {
                self.line(vec[i][0], vec[i][1], vec[0][0], vec[0][1])

            }
            else {
                self.line(vec[i][0], vec[i][1], vec[i+1][0], vec[i+1][1])  
            }
        }

    }

    fn draw_filled_polygon(&mut self, vec: Vec<[usize; 2]>) {
        
    }

    fn get_max_limits(vec: Vec<[usize; 2]){
        
    }
}