use rand::prelude::*;
use winit::event::ElementState;

pub mod elements;
mod update_functions;
pub mod screen_texture;

use crate::falling_sand::screen_texture::*;
use crate::falling_sand::elements::*;

// use crate::falling_sand::


pub struct SandBoard {
    granules: Vec<Element>,
    pub width: usize,
    pub height: usize,
    pub is_floor: bool, 
    pub right_sand: usize,
    pub left_sand: usize,
    pub unchanged: usize,
    pub unaccounted: usize,
    update_cycle: bool,
}

impl SandBoard {
    pub fn new(width: usize, height: usize) -> Self {
        let indices = width * height;
        let mut granules = Vec::new();

        for i in 0..indices {
            let element = Element::new(ElementType::Empty);
            granules.push(element);
        }
        
        Self {
            granules,
            width,
            height,
            is_floor: true,
            right_sand: 0,
            left_sand: 0,
            unchanged: 0,
            unaccounted: 0,
            update_cycle: true,
        }
    }

    pub fn reset(&mut self) {
        let indices = self.width * self.height;
        let mut granules = Vec::new();
        for i in 0..indices {
            let element = Element::new(ElementType::Empty);
            granules.push(element);
        }
        self.granules = granules;
    }

    #[allow(dead_code)]
    pub fn first_ten(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index_from_coordinates(x, y);
                if index < 10 {
                    self.granules[index] = Element::new(ElementType::Sand);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn middle(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index_from_coordinates(x, y);
                if y == self.height / 2 {
                    self.granules[index] = Element::new(ElementType::Sand);
                }
            }
        }
    }


    #[allow(dead_code)]
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index_from_coordinates(x, y);
                let rand_val: f64 = rng.gen();
                if rand_val > 0.5 {
                    self.granules[index] = Element::new(ElementType::Sand);
                }
            }
        }
    }

    pub fn randomize_top(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                if y < self.height / 2 {
                    let index = self.get_index_from_coordinates(x, y);
                    let rand_val: f64 = rng.gen();
                    if rand_val > 0.5 {
                        self.granules[index] = Element::new(ElementType::Sand);
                    }
                    
                }
            }
        }
    }
    pub fn randomize_ocean(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                if y < ((self.height / 10) * 9) {
                    let index = self.get_index_from_coordinates(x, y);
                    let rand_val: f64 = rng.gen();
                    if rand_val > 0.95 {
                        self.granules[index] = Element::new(ElementType::Sand);
                    }
                    else if rand_val > 0.15{
                        self.granules[index] = Element::new(ElementType::Water);
                    }
                    
                }
            }
        }
    }

    pub fn boring_ocean(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                if y < ((self.height / 20) * 19) {
                    let index = self.get_index_from_coordinates(x, y);
                    self.granules[index] = Element::new(ElementType::Water);
                }
                else {
                    let index = self.get_index_from_coordinates(x, y);
                    self.granules[index] = Element::new(ElementType::Sand);
                }
            }
        }
    }

    pub fn boring_sand(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                if y > ((self.height / 20) * 19) {
                    let index = self.get_index_from_coordinates(x, y);
                    self.granules[index] = Element::new(ElementType::Sand);
                }
            }
        }
    }

    fn get_index_from_coordinates(&self, x: usize, y: usize) -> usize {
        return y * self.width + x
    }
    fn get_coordinates_from_index(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        return (x, y)
    }

    pub fn output_texture(&self) -> ScreenTexture {
        let max_index = self.width * self.height;
        let mut pixel_data: Vec<u8> = Vec::new();
        for i in 0..max_index {
            if self.granules[i].element_type == ElementType::Sand {
                pixel_data.push(COLORS_YELLOW[0]);
                pixel_data.push(COLORS_YELLOW[1]);
                pixel_data.push(COLORS_YELLOW[2]);
                pixel_data.push(COLORS_YELLOW[3]);
            }
            else if self.granules[i].element_type == ElementType::Water {
                pixel_data.push(COLORS_BLUE[0]);
                pixel_data.push(COLORS_BLUE[1]);
                pixel_data.push(COLORS_BLUE[2]);
                pixel_data.push(COLORS_BLUE[3]);
            }
            else if self.granules[i].element_type == ElementType::Wall {
                pixel_data.push(COLORS_GREY[0]);
                pixel_data.push(COLORS_GREY[1]);
                pixel_data.push(COLORS_GREY[2]);
                pixel_data.push(COLORS_GREY[3]);
            }
            else if self.granules[i].element_type == ElementType::Dirt {
                pixel_data.push(COLORS_BROWN[0]);
                pixel_data.push(COLORS_BROWN[1]);
                pixel_data.push(COLORS_BROWN[2]);
                pixel_data.push(COLORS_BROWN[3]);
            }
            else if self.granules[i].element_type == ElementType::Seed {
                pixel_data.push(COLORS_LIGHT_GREEN[0]);
                pixel_data.push(COLORS_LIGHT_GREEN[1]);
                pixel_data.push(COLORS_LIGHT_GREEN[2]);
                pixel_data.push(COLORS_LIGHT_GREEN[3]);
            }
            else if self.granules[i].element_type == ElementType::Grass {
                pixel_data.push(COLORS_PALE_YELLOW[0]);
                pixel_data.push(COLORS_PALE_YELLOW[1]);
                pixel_data.push(COLORS_PALE_YELLOW[2]);
                pixel_data.push(COLORS_PALE_YELLOW[3]);
            }
            else if self.granules[i].element_type == ElementType::Kelp {
                pixel_data.push(COLORS_DARK_GREEN[0]);
                pixel_data.push(COLORS_DARK_GREEN[1]);
                pixel_data.push(COLORS_DARK_GREEN[2]);
                pixel_data.push(COLORS_DARK_GREEN[3]);
            }
            else if self.granules[i].element_type == ElementType::Minnow {
                pixel_data.push(COLORS_DARK_RED[0]);
                pixel_data.push(COLORS_DARK_RED[1]);
                pixel_data.push(COLORS_DARK_RED[2]);
                pixel_data.push(COLORS_DARK_RED[3]);
            }
            else if self.granules[i].element_type == ElementType::Egg {
                pixel_data.push(COLORS_DARK_ORANGE[0]);
                pixel_data.push(COLORS_DARK_ORANGE[1]);
                pixel_data.push(COLORS_DARK_ORANGE[2]);
                pixel_data.push(COLORS_DARK_ORANGE[3]);
            }
            else {
                pixel_data.push(COLORS_MARIO_BACKGROUND[0]);
                pixel_data.push(COLORS_MARIO_BACKGROUND[1]);
                pixel_data.push(COLORS_MARIO_BACKGROUND[2]);
                pixel_data.push(COLORS_MARIO_BACKGROUND[3]);
            }
        }

        let dimensions = (self.width, self.height);

        ScreenTexture {
            pixel_data,
            dimensions
        }

    }

    pub fn add_granules(&mut self, x: usize, y: usize, radius: usize, granule_type: ElementType) -> usize {
        let mut added_granules = 0;
        let rad: i64 = radius as i64;
        for k in -rad..=rad {
            for i in -rad..=rad {
                let new_x = x as i64 + i;
                let new_y = y as i64 + k;
                if self.is_in_coordinate_bounds_signed(new_x, new_y) {
                    if SandBoard::get_distance((new_x as f64, new_y as f64), (x as f64, y as f64)) < radius as f64 {
                        let index = self.get_index_from_coordinates(new_x as usize, new_y as usize);
                        if self.granules[index].element_type == ElementType::Empty {
                            added_granules += 1;
                        }
                        self.add_granule(new_x as usize, new_y as usize, granule_type);
                    }
                }    
            }
        }   
        return added_granules
    }

    fn get_distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
        let x_dist = point1.0 - point2.0;
        let y_dist = point1.1 - point2.1;
        return f64::powf(f64::powf(x_dist, 2.0) + f64::powf(y_dist, 2.0), 0.5)
    }

    fn add_granule(&mut self, x: usize, y: usize, granule_type: ElementType) {
        if self.is_in_coordinate_bounds(x, y) {
            let index = self.get_index_from_coordinates(x, y);
            self.granules[index] = Element::new(granule_type);
        }
    }

    fn update_granule(&mut self, x: usize, y: usize, granule_type: ElementType) {
        let index = self.get_index_from_coordinates(x, y);
        self.granules[index] = Element::new(granule_type);
        self.granules[index].update_toggle = self.update_cycle;
    }

    pub fn tick(&mut self) {
        let max_index = self.height * self.width;
        let mut rng = rand::thread_rng();

        //let mut vec: Vec<usize> = (0..max_index).collect();
        //vec.shuffle(&mut rng);

        for i in 0..max_index {
            //let rand_index = vec[i];
            

            let mut index = i;
            if (i / self.width) % 2 == 0 {
                let x = i % self.width;
                index =  i - x + (self.width - x - 1);
            }

            if self.granules[index].element_type == ElementType::Empty {
                continue;
            }

            let surrounding_granule = self.get_surrounding_from_index(index);
            self.granule_tick_new(index);
        }
        if self.update_cycle {
            self.update_cycle = false;
        }
        else {
            self.update_cycle = true;
        }
    } 


    pub fn get_surrounding(&self, x: usize, y: usize) -> [[Element; 3]; 3] {
        if self.is_in_coordinate_bounds(x, y) {
            let center_index = self.get_index_from_coordinates(x, y);
            return self.get_surrounding_from_index(center_index)
        }
        return [[Element::new(ElementType::Empty); 3]; 3]
    }


    fn get_surrounding_from_index(&self, center_index: usize) -> [[Element; 3]; 3] {
        let mut grid = [[Element::new(ElementType::Empty); 3]; 3];
        let (center_x, center_y) = self.get_coordinates_from_index(center_index);
        let x_start: i64 = center_x as i64 - 1;
        let y_start: i64 = center_y as i64 - 1;
        for k in 0..=2 {
            if y_start == -1 && k == 0 {
                grid[k] = [Element::new(ElementType::ScreenEdge); 3];
                continue
            }
            else if center_y == (self.height - 1) && k == 2{
                grid[k] = [Element::new(ElementType::ScreenEdge); 3];
                continue
            }
            for i in 0..=2 {
                if x_start == -1 && i == 0 {
                    grid[k][i] = Element::new(ElementType::ScreenEdge);
                    continue
                }
                else if center_x == (self.width - 1) && i == 2 {
                    grid[k][i] = Element::new(ElementType::ScreenEdge);
                    continue
                }
                let x = (x_start + i as i64) as usize;
                let y = (y_start + k as i64) as usize;
                let index = self.get_index_from_coordinates(x, y);
                grid[k][i] = self.granules[index];
            }
        }
        return grid
    }

    fn is_in_coordinate_bounds(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            return true
        }
        return false
    }

    fn is_in_coordinate_bounds_signed(&self, x: i64, y: i64) -> bool {
        if x >= 0 && x < self.width as i64 && y >= 0 && y < self.height as i64 {
            return true
        }
        return false
    }

    pub fn get_granule_count(&self) -> usize {
        let mut count = 0;
        for i in 0..self.granules.len() {
            if self.granules[i].element_type != ElementType::Empty {
                count += 1
            }
        }
        return count
    }

}



pub const COLORS_YELLOW: [u8; 4] = [0xCC, 0xCC, 0x00, 0xFF];
pub const COLORS_BLUE: [u8; 4] = [0x00, 0xCC, 0xFF, 0xFF];
pub const COLORS_MARIO_BACKGROUND: [u8; 4] = [0x09, 0x26, 0x41, 0xFF];
pub const COLORS_BACKGROUND: [u8; 4] = [0x21, 0x13, 0x38, 0xFF];
pub const COLORS_BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];
pub const COLORS_WHITE: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
pub const COLORS_GREY: [u8; 4] = [0xAA, 0xAA, 0xAA, 0xFF];
pub const COLORS_BROWN: [u8; 4] = [0x80, 0x5D, 0x3C, 0xFF];
pub const COLORS_LIGHT_GREEN: [u8; 4] = [0xAB, 0xF7, 0xB1, 0xFF];
pub const COLORS_PALE_YELLOW: [u8; 4] = [0xFF, 0xFC, 0xD3, 0xFF];
pub const COLORS_DARK_GREEN: [u8; 4] = [0x17, 0x35, 0x18, 0xFF];
pub const COLORS_DARK_RED: [u8; 4] = [0xFF, 0x00, 0x00, 0xFF];
pub const COLORS_DARK_ORANGE: [u8; 4] = [0xE7, 0x96, 0x8B, 0xFF];


#[cfg(test)]
mod sand_test {
    use super::*; 

    #[test]
    fn coordinate_test() {
        let width = 15;
        let height = 50;
        let sand = SandBoard::new(width, height);
        let max = width * height;

        for i in 0..max {
            let (x, y) = sand.get_coordinates_from_index(i);
            let index = sand.get_index_from_coordinates(x, y);
            println!("original: {} r: {}, ({}, {})", i, index, x, y);
        }

    }
}