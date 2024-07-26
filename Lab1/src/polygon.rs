use crate::line::Line;
use crate::framebuffer::FrameBuffer;  

pub trait Polygon {
    fn polygon(&mut self, vec: &Vec<[usize; 2]>);
    fn fill_polygon(&mut self, vec:&Vec<[usize;2]>);
}

impl Polygon for FrameBuffer {
    fn polygon(&mut self, vec: &Vec<[usize; 2]>) {
        // Draw the polygon edges
        for i in 0..vec.len() {
            let (x1, y1) = (vec[i][0], vec[i][1]);
            let (x2, y2) = if i == vec.len() - 1 { (vec[0][0], vec[0][1]) } else { (vec[i + 1][0], vec[i + 1][1]) };
            self.line(x1, y1, x2, y2);
        }
    }

    fn fill_polygon(&mut self, vec: &Vec<[usize; 2]>) {
        let (max_x, min_x, max_y, min_y) = get_max_limits(&vec);
        let mut intersections:Vec<usize> = Vec::new();
        let mut x0:isize = 0;
        let mut xf: isize = 0;
        let mut y0:isize = 0;
        let mut yf:isize = 0;
        for y in min_y..=max_y-1 {

            for i in 0..vec.len() {
                x0 = vec[i][0] as isize;
                y0 = vec[i][1] as isize;
                let casted_y = y as isize;
                if i != vec.len()-1 {
                    xf = vec[i+1][0] as isize;
                    yf = vec[i+1][1] as isize;                
                }
                else {
                    xf = vec[0][0] as isize;
                    yf = vec[0][1] as isize;
                }
                
                if (y0 <= casted_y   &&  casted_y < yf ) || (yf <= casted_y && casted_y < y0) {
	                let x = x0 + (casted_y - y0) * (xf - x0) / (yf - y0);
	                intersections.push(x as usize);
	            }      
            }

            if intersections.len() > 1 {
                intersections.sort();

                for k in (0..intersections.len()).step_by(2){

                    if k+1 < intersections.len(){
                        let start = intersections[k];
                        let end = intersections[k + 1];
                        for l in start..=end {
                            self.point(l, y);
                        }

                    }

                }
            }
            intersections.clear();
        }
    }
}



pub fn get_max_limits( vec: &Vec<[usize; 2]>) ->(usize, usize, usize, usize) {
    let x_point_storage: Vec<usize> = vec.iter().map(|arr| arr[0]).collect();
    let y_point_storage: Vec<usize> =vec.iter().map(|arr| arr[1]).collect();

    (*x_point_storage.iter().max().unwrap(),*x_point_storage.iter().min().unwrap(),
    *y_point_storage.iter().max().unwrap()
    ,*y_point_storage.iter().min().unwrap())
}




