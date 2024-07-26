
use crate::falling_sand::elements::*;
pub struct ScreenTexture {
    pub pixel_data: Vec<u8>,
    pub dimensions: (usize, usize),
}

impl ScreenTexture {
    pub fn encode_debug_info(&mut self, surrounding_data: [[Element; 3];3]) {
        // todo: convert surrounding data into pixel data
        let x_start = self.dimensions.0 - 3;
        for y in 0..3 {
            for x in 0..3 {
                let mut color = [0; 4];
                match surrounding_data[y][x].element_type {
                    ElementType::Sand => {
                        color[0] = 0;
                        color[1] = 0;
                        color[2] = 0;
                        color[3] = 0xFF;
                    },
                    ElementType::Empty => {
                        color[0] = 0;
                        color[1] = 0xCC;
                        color[2] = 0xCC;
                        color[3] = 0xFF;
                    },
                    _ => {
                        color[0] = 0xFF;
                        color[1] = 0x00;
                        color[2] = 0x00;
                        color[3] = 0xFF;
                    }
                }
                self.draw_pixel_at_coordinate(x_start + x, y, color);
            }
        }
        
    }

    pub fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        (y * self.dimensions.0 + x) * 4
    }

    pub fn draw_pixel_at_coordinate(&mut self, x: usize, y: usize, color: [u8; 4]) {
        let pixel_index = self.get_pixel_index(x, y);
        self.pixel_data[pixel_index] = color[0];
        self.pixel_data[pixel_index + 1] = color[1];
        self.pixel_data[pixel_index + 2] = color[2];
        self.pixel_data[pixel_index + 3] = color[3];
    }

    fn is_in_bounds(&self, x: i64, y: i64) -> bool {
        if x >= 0 && x < self.dimensions.0 as i64 && y >= 0 && y < self.dimensions.1 as i64 {
            return true
        }
        return false
    }

    pub fn print(&mut self, text: String, position: (i64, i64)) {
        let mut pixel_x = position.0 * 6;
        let mut pixel_y = position.1 * 10;
        //let mut text_vec = Vec::new();
        let text_vec: Vec<char> = text.chars().collect();
        for c in 0..text_vec.len() {
            let char_data;
            
            match text_vec[c] {
                '0' => {
                    char_data = FONT[0];
                },
                '1' => {
                    char_data = FONT[1];
                },
                '2' => {
                    char_data = FONT[2];
                },
                '3' => {
                    char_data = FONT[3];
                },
                '4' => {
                    char_data = FONT[4];
                },
                '5' => {
                    char_data = FONT[5];
                },
                '6' => {
                    char_data = FONT[6];
                },
                '7' => {
                    char_data = FONT[7];
                },
                '8' => {
                    char_data = FONT[8];
                },
                '9' => {
                    char_data = FONT[9];
                },
                '.' => {
                    char_data = FONT[10];
                }
                _ => {
                    char_data = FONT[11];
                },
            }

            for k in 0..8 {
                let new_y = pixel_y + k;
                for i in 0..5 {
                    let new_x  = pixel_x + i;
                    if self.is_in_bounds(new_x, new_y) {
                        if char_data[k as usize][i as usize] == 1 {
                            self.draw_pixel_at_coordinate(new_x as usize, new_y as usize, COLOR_FONT);
                        }    
                    }
                }
            }
            pixel_x += 6;
        }
    }


    pub fn print_filled_rect(&mut self, top_left: (usize, usize), size: (usize, usize), color: [u8; 4]) {
        for y in top_left.1..=(top_left.1 + size.1) {
            for x in top_left.0..=(top_left.0 + size.0) {
                self.draw_pixel_at_coordinate(x, y, color)
            }
        }
    }
    
    pub fn print_empty_rect(&mut self, top_left: (usize, usize), size: (usize, usize), color: [u8; 4]) {
        for y in top_left.1..=(top_left.1 + size.1) {
            self.draw_pixel_at_coordinate(top_left.0, y, color);
            self.draw_pixel_at_coordinate(top_left.0 + size.0 , y, color);
        }
        for x in top_left.0..=(top_left.0 + size.0) {
            self.draw_pixel_at_coordinate(x, top_left.1, color);
            self.draw_pixel_at_coordinate(x, top_left.1 + size.1, color);
        }
    }
    
}




const COLOR_FONT: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];

const FONT: [[[u8; 5]; 8]; 12] = 
    [
        [[0, 1, 1, 1, 0],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [0, 1, 1, 1, 0]],

        [[0, 0, 1, 0, 0],
         [0, 1, 1, 0, 0],
         [0, 0, 1, 0, 0],
         [0, 0, 1, 0, 0],
         [0, 0, 1, 0, 0],
         [0, 0, 1, 0, 0],
         [0, 0, 1, 0, 0],
         [0, 1, 1, 1, 0]],

        [[0, 1, 1, 1, 0],
         [1, 0, 0, 0, 1],
         [0, 0, 0, 0, 1],
         [0, 0, 0, 1, 0],
         [0, 0, 1, 0, 0],
         [0, 1, 0, 0, 0],
         [1, 0, 0, 0, 0],
         [1, 1, 1, 1, 1]],

        [[0, 1, 1, 1, 0],
         [1, 0, 0, 0, 1],
         [0, 0, 0, 0, 1],
         [0, 0, 1, 1, 0],
         [0, 0, 0, 0, 1],
         [0, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [0, 1, 1, 1, 0]],

        [[0, 0, 0, 1, 0],
         [0, 0, 1, 1, 0],
         [0, 1, 0, 1, 0],
         [1, 0, 0, 1, 0],
         [1, 1, 1, 1, 1],
         [0, 0, 0, 1, 0],
         [0, 0, 0, 1, 0],
         [0, 0, 0, 1, 0]],

        [[1, 1, 1, 1, 1],
         [1, 0, 0, 0, 0],
         [1, 0, 0, 0, 0],
         [1, 1, 1, 1, 0],
         [0, 0, 0, 0, 1],
         [0, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [0, 1, 1, 1, 0]],

        [[0, 0, 1, 1, 0],
         [0, 1, 0, 0, 0],
         [1, 0, 0, 0, 0],
         [1, 1, 1, 1, 0],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [0, 1, 1, 1, 0]],

        [[1, 1, 1, 1, 1],
         [0, 0, 0, 0, 1],
         [0, 0, 0, 1, 0],
         [0, 0, 0, 1, 0],
         [0, 0, 1, 0, 0],
         [0, 0, 1, 0, 0],
         [0, 1, 0, 0, 0],
         [0, 1, 0, 0, 0]],

         [[0, 1, 1, 1, 0],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [0, 1, 1, 1, 0],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [0, 1, 1, 1, 0]],

         [[0, 1, 1, 1, 0],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1],
         [0, 1, 1, 1, 1],
         [0, 0, 0, 0, 1],
         [0, 0, 0, 1, 0],
         [0, 1, 1, 0, 0]],

         [[0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 1, 0, 0]],

         [[0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0]],
    ];

