use rand::prelude::*;
use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
use crate::SandBoard;

use super::{Element, ElementType};



enum LiquidCollision {
    None,
    Wall,
    OpenSpace(usize),
}


impl SandBoard {

    fn get_relative_index(&mut self, index: usize, x_relative: i64, y_relative: i64) -> Option<usize> {
        let (x, y) = self.get_coordinates_from_index(index);
        let new_x = x as i64 + x_relative;
        let new_y = y as i64 + y_relative;
        if self.is_in_coordinate_bounds_signed(new_x, new_y) {
            return Some(self.get_index_from_coordinates(new_x as usize, new_y as usize));
            
        }
        else {
            return None
        }
    }

    pub fn granule_tick_new(&mut self, index: usize) {
        if self.granules[index].update_toggle == self.update_cycle {
            return
        }
        let element_type = self.granules[index].element_type;
        match element_type {
            ElementType::Empty => {

            },
            ElementType::Sand => {
                self.update_sand(index);
            },
            ElementType::Water => {
                self.update_water(index);
            },
            ElementType::Wall => {

            },
            ElementType::Dirt => {
                self.update_dirt(index);
            },
            ElementType::Seed => {
                self.update_seed(index);
            },
            ElementType::Grass => {
                self.update_grass(index);
            },
            ElementType::Kelp => {
                self.update_kelp(index);
            },
            ElementType::Egg => {
                self.update_egg(index);
            },
            ElementType::Minnow => {
                self.update_minnow(index);
            },
            _ => {

            }

        }
    }

    pub fn is_falling_diagonal(&mut self, index: usize) -> bool {
        let mut rng = rand::thread_rng();
        let rng_val: f64 = rng.gen(); 
        let mut directions = Vec::new();
        if rng_val > 0.5 {
            directions.push(self.get_relative_index(index, -1, 1));
            directions.push(self.get_relative_index(index, 1, 1));
        } 
        else {
            directions.push(self.get_relative_index(index, 1, 1));
            directions.push(self.get_relative_index(index, -1, 1));
        }

        for direction in directions {
            if direction.is_some() {
                if (self.granules[direction.unwrap()].element_type == ElementType::Empty ||
                    self.granules[direction.unwrap()].element_type == ElementType::Water) &&
                    self.granules[direction.unwrap()].element_type != self.granules[index].element_type {
                        let current_granule = self.granules[index];
                        self.granules[index] = self.granules[direction.unwrap()];
                        self.granules[direction.unwrap()] = current_granule;
                        self.granules[direction.unwrap()].update_toggle = self.update_cycle;
                        return true
                    }
            }
        }
        return false
    }

    pub fn is_falling_diagonal_air(&mut self, index: usize) -> bool {
        let mut rng = rand::thread_rng();
        let rng_val: f64 = rng.gen(); 
        let mut directions = Vec::new();
        if rng_val > 0.5 {
            directions.push(self.get_relative_index(index, -1, 1));
            directions.push(self.get_relative_index(index, 1, 1));
        } 
        else {
            directions.push(self.get_relative_index(index, 1, 1));
            directions.push(self.get_relative_index(index, -1, 1));
        }

        for direction in directions {
            if direction.is_some() {
                if self.granules[direction.unwrap()].element_type == ElementType::Empty &&
                    self.granules[direction.unwrap()].element_type != self.granules[index].element_type {
                        let current_granule = self.granules[index];
                        self.granules[index] = self.granules[direction.unwrap()];
                        self.granules[direction.unwrap()] = current_granule;
                        self.granules[direction.unwrap()].update_toggle = self.update_cycle;
                        return true
                    }
            }
        }
        return false
    }
    

    fn update_sand(&mut self, index: usize) {
        let current_granule = self.granules[index];
        let (x, y) = self.get_coordinates_from_index(index);
        if let Some(below_index) = self.get_relative_index(index, 0, 1) {
            let granule_below = self.granules[below_index];
            if granule_below.element_type == ElementType::Empty || granule_below.element_type == ElementType::Water {
                self.granules[index] = granule_below;
                self.granules[below_index] = current_granule;
                self.granules[below_index].update_toggle = self.update_cycle;
                return
            }
        }
        else {
            if !self.is_floor {
                self.update_granule(x, y, ElementType::Empty);
            }
            return
        }
        if self.is_falling_diagonal(index) {
            return
        }
    }

    fn update_dirt(&mut self, index: usize) {
        let current_granule = self.granules[index];
        let (x, y) = self.get_coordinates_from_index(index);
        if let Some(below_index) = self.get_relative_index(index, 0, 1) {
            let granule_below = self.granules[below_index];
            if granule_below.element_type == ElementType::Empty || granule_below.element_type == ElementType::Water {
                self.granules[index] = granule_below;
                self.granules[below_index] = current_granule;
                self.granules[below_index].update_toggle = self.update_cycle;
                return
            }
        }
        else {
            if !self.is_floor {
                self.update_granule(x, y, ElementType::Empty);
            }
            return
        }
        if self.is_falling_diagonal(index) {
            return
        }
    }

    fn update_water(&mut self, index: usize) {
        let current_granule = self.granules[index];
        if let Some(below_index) = self.get_relative_index(index, 0, 1) {
            let granule_below = self.granules[below_index];
            if granule_below.element_type == ElementType::Empty {
                self.granules[index] = granule_below;
                self.granules[below_index] = current_granule;
                self.granules[below_index].update_toggle = self.update_cycle;
                return
            }
        }
        else {
            if !self.is_floor {
                let (x, y) = self.get_coordinates_from_index(index);
                self.update_granule(x, y, ElementType::Empty);
            }
            return
        }
        if self.is_falling_diagonal(index) {
            return
        }
        
        self.move_forward(index);
    }



    fn look_forward(&mut self, index: usize, direction: bool, distance: usize) -> LiquidCollision {
        let (x, y) = self.get_coordinates_from_index(index);
        for i in 1..=distance {
            let mut new_x = x as i64;
            if !direction {
                new_x -= i as i64;
            }
            else {
                new_x += i as i64;
            }
            if self.is_in_coordinate_bounds_signed(new_x, x as i64) {
                let new_index = self.get_index_from_coordinates(new_x as usize, y);
                if self.granules[new_index].element_type == ElementType::Empty {
                    return LiquidCollision::OpenSpace(new_index)
                } 
                else if self.granules[new_index].element_type != ElementType::Water{
                    return LiquidCollision::Wall
                }
            }
            else {
                return LiquidCollision::Wall
            }
        }
        return LiquidCollision::None
    }


    pub fn move_forward(&mut self, index: usize) -> bool {
        let dispersion = 5;

        match self.look_forward(index, self.granules[index].direction, dispersion) {
            LiquidCollision::Wall => {
                self.granules[index].switch_direction();
                return false
            },
            LiquidCollision::OpenSpace(new_index) => {
                let current_granule = self.granules[index];
                self.granules[index] = self.granules[new_index];
                self.granules[new_index] = current_granule;
                self.granules[new_index].update_toggle = self.update_cycle;
                return true
            },
            LiquidCollision::None => {
                match self.look_forward(index, !self.granules[index].direction, dispersion) {
                    LiquidCollision::Wall => {
                        self.granules[index].switch_direction();
                        return false
                    },
                    LiquidCollision::OpenSpace(new_index) => {
                        let current_granule = self.granules[index];
                        self.granules[index] = self.granules[new_index];
                        self.granules[new_index] = current_granule;
                        self.granules[new_index].update_toggle = self.update_cycle;
                        return true
                    },
                    LiquidCollision::None => {
                        return false
                    }
                }
            }
        }
        
    }

    fn update_seed(&mut self, index: usize) {
        let current_granule = self.granules[index];
        if let Some(below_index) = self.get_relative_index(index, 0, 1) {
            let granule_below = self.granules[below_index];
            match granule_below.element_type {
                ElementType::Empty => {
                    self.granules[index] = granule_below;
                    self.granules[below_index] = current_granule;
                    self.granules[below_index].update_toggle = self.update_cycle;
                    return
                },
                ElementType::Dirt => {
                    let mut rng = rand::thread_rng();
                    if let Some(above_index) = self.get_relative_index(index, 0, -1) {
                        if self.granules[above_index].element_type == ElementType::Water {
                            self.granules[index].element_type = ElementType::Kelp;
                            self.granules[index].growth = rng.gen_range(30..self.height as u8);
                            self.granules[index].hunger = rng.gen_range(1..20);
                            self.granules[below_index].update_toggle = self.update_cycle;
                            return
                        }
                    }
                    self.granules[index].element_type = ElementType::Grass;
                    self.granules[below_index].update_toggle = self.update_cycle;
                    return
                },
                ElementType::Sand => {
                    let mut rng = rand::thread_rng();
                    if let Some(above_index) = self.get_relative_index(index, 0, -1) {
                        if self.granules[above_index].element_type == ElementType::Water {
                            self.granules[index].element_type = ElementType::Kelp;
                            self.granules[index].growth = rng.gen_range(30..self.height as u8);
                            self.granules[index].hunger = rng.gen_range(1..20);
                            self.granules[below_index].update_toggle = self.update_cycle;
                            return
                        }
                    }
                    let val = rng.gen_range(2..20);
                    self.granules[index].element_type = ElementType::Grass;
                    self.granules[index].growth = val;
                    self.granules[below_index].update_toggle = self.update_cycle;
                    return
                },
                _ => {

                }
            }
          
        }
        else {
            if !self.is_floor {
                let (x, y) = self.get_coordinates_from_index(index);
                self.update_granule(x, y, ElementType::Empty);
            }
            return
        }
        if self.is_falling_diagonal(index) {
            return
        }
    }

    fn update_grass(&mut self, index: usize) {
        self.grow_grass(index);
    }

    fn grow_grass(&mut self, index: usize) -> bool {
        if self.granules[index].growth == 0 {
            return false
        }
        let mut rng = rand::thread_rng();
        let rng_val: f64 = rng.gen(); 
        let mut directions = Vec::new();
        
        directions.push(self.get_relative_index(index, 0, -1));
        directions.push(self.get_relative_index(index, -1, -1));
        directions.push(self.get_relative_index(index, 1, -1));




        if rng_val > 0.2 {
            let starting = directions.remove(0);
            directions.shuffle(&mut rng);
            directions.insert(0, starting);
        }
        else if rng_val > 0.1 {
            let starting = directions.remove(1);
            directions.shuffle(&mut rng);
            directions.insert(0, starting);
        }
        else {
            let starting = directions.remove(2);
            directions.shuffle(&mut rng);
            directions.insert(0, starting);
        }

        for direction in directions.iter() {
            if direction.is_some() {
                if self.granules[direction.unwrap()].element_type == ElementType::Grass {
                    self.granules[index].update_toggle = self.update_cycle;
                    return false
                }
            }

        }

        for direction in directions {
            if direction.is_some() {
                if self.granules[direction.unwrap()].element_type == ElementType::Empty {
                    self.granules[direction.unwrap()].element_type = ElementType::Grass;
                    self.granules[direction.unwrap()].growth = self.granules[index].growth - 1;
                    self.granules[direction.unwrap()].update_toggle = self.update_cycle;
                    self.granules[index].update_toggle = self.update_cycle;

                    let new_rng: f64 = rng.gen();
                    //if new_rng < 0.25 {
                    //    return true;
                    //}
                    return true;
                }
            }
        }
        return false
    }


    fn update_kelp(&mut self, index: usize) {
        let mut rng = rand::thread_rng();
        let rng_val: f64 = rng.gen(); 
        let growth_chance = self.granules[index].hunger as f64 / 100.0;
        if rng_val < (growth_chance) {
            self.grow_kelp(index);
        }
        
    }

    fn grow_kelp(&mut self, index: usize) -> bool {
        if self.granules[index].growth == 0 {
            return false
        }
        let mut rng = rand::thread_rng();
        let rng_val: f64 = rng.gen(); 
        let mut directions = Vec::new();
        
        directions.push(self.get_relative_index(index, 0, -1));
        directions.push(self.get_relative_index(index, -1, -1));
        directions.push(self.get_relative_index(index, 1, -1));




        if rng_val > 0.2 {
            let starting = directions.remove(0);
            directions.shuffle(&mut rng);
            directions.insert(0, starting);
        }
        else if rng_val > 0.1 {
            let starting = directions.remove(1);
            directions.shuffle(&mut rng);
            directions.insert(0, starting);
        }
        else {
            let starting = directions.remove(2);
            directions.shuffle(&mut rng);
            directions.insert(0, starting);
        }

        for direction in directions.iter() {
            if direction.is_some() {
                if self.granules[direction.unwrap()].element_type == ElementType::Kelp {
                    self.granules[index].update_toggle = self.update_cycle;
                    return false;
                }
            }

        }

        for direction in directions {
            if direction.is_some() {
                if self.granules[direction.unwrap()].element_type == ElementType::Water {
                    self.granules[direction.unwrap()].element_type = ElementType::Kelp;
                    let mut growth_penalty = rng.gen_range(1..3);
                    if growth_penalty > self.granules[index].growth {
                        growth_penalty = self.granules[index].growth;
                    }
                    self.granules[direction.unwrap()].growth = self.granules[index].growth - growth_penalty;
                    self.granules[direction.unwrap()].hunger = self.granules[index].hunger;
                    self.granules[direction.unwrap()].update_toggle = self.update_cycle;
                    self.granules[index].update_toggle = self.update_cycle;
                    
                    let new_rng: f64 = rng.gen();
                    if new_rng < 0.25 {
                        return true;
                    }
                    
                }
            }
        }
        return false;
    }

    /* 
    fn update_egg(&mut self, index: usize) {
        // if on land
        // isopod
        // if in shallow water?
        // tadpole
        // if in deep water
        // minnow

        let mut rng = rand::thread_rng();


        let mut is_on_ground = false;
        if let Some(below_index) = self.get_relative_index(index, 0, 1) {
            if self.granules[below_index].element_type == ElementType::Water || 
                self.granules[below_index].element_type == ElementType::Empty {
                    let granule_below = self.granules[below_index];
                    self.granules[below_index] = self.granules[index];
                    self.granules[index] = granule_below;
                    self.granules[index].update_toggle = self.update_cycle;
                    self.granules[below_index].update_toggle = self.update_cycle;
                    return
            }
            else if self.is_falling_diagonal(index) {
                return
            }
            else {
                self.granules[index].update_toggle = self.update_cycle;
                is_on_ground = true;
            }
            
        }
        else {
            is_on_ground = true;
        }


        if is_on_ground {
            if let Some(above_index) = self.get_relative_index(index, 0, -1){
                if self.granules[above_index].element_type != ElementType::Water {
                    // I don't have an easy access to fps, sooooo we'll say it's 35
                    let rough_hatch_time_seconds = 10.0;
                    let probability =  1.0 / (rough_hatch_time_seconds * 35.0);
                    if rng.gen::<f64>() < probability {
                        self.granules[index].element_type = ElementType::Isopod;
                    }
                }
                else {
                    // this doesn't need to be a loop
                    // it could just pick a point and say "that's (not) water",
                    let depth = 20;
                    let mut is_shallow = false;
                    for i in 0..20 {
                        if let Some(depth_check_index) = self.get_relative_index(index, 0, -i) {
                            if self.granules[above_index].element_type != ElementType::Water {
                                is_shallow = true;
                            }
                        }
                    }
                    if is_shallow {
                        let rough_hatch_time_seconds = 10.0;
                        let probability =  1.0 / (rough_hatch_time_seconds * 35.0);
                        if rng.gen::<f64>() < probability {
                            self.granules[index].element_type = ElementType::Tadpole;
                        }
                    }
                    else {
                        let rough_hatch_time_seconds = 10.0;
                        let probability =  1.0 / (rough_hatch_time_seconds * 35.0);
                        if rng.gen::<f64>() < probability {
                            let mut depth = 20;
                            for i in 1..self.height {
                                let y = -1 * (i as i64);
                                if let Some(depth_check_index) = self.get_relative_index(index, 0, y) {
                                    if self.granules[depth_check_index].element_type != ElementType::Water {
                                        depth = i;
                                        break;
                                    }
                                }
                                else {
                                    depth = i;
                                    break;
                                }
                                if depth > 255 {
                                    depth = 255;
                                    break;
                                }
                            }
                            if depth < 4 {
                                return;
                            }
                            self.granules[index].element_type = ElementType::Minnow;
                            self.granules[index].moisture = rng.gen_range(3..(depth - 3) as u8);
                            if rng.gen::<f64>() < 0.5 {
                                self.granules[index].direction = true;
                            }
                            else {
                                self.granules[index].direction = false;
                            }
                            
                        }
                    }
                    
                }
            }
        }

    }
    */

    fn is_falling_down_air(&mut self, index: usize) -> bool {
        if let Some(below_index) = self.get_relative_index(index, 0, 1) {
            if self.granules[below_index].element_type == ElementType::Empty {
                let granule_below = self.granules[below_index];
                self.granules[below_index] = self.granules[index];
                self.granules[index] = granule_below;
                self.granules[index].update_toggle = self.update_cycle;
                self.granules[below_index].update_toggle = self.update_cycle;
                return true;
            }
            else {
                self.granules[index].update_toggle = self.update_cycle;
                return false;
            }
            
        }
        else if !self.is_floor {
            self.granules[index].element_type = ElementType::Empty;
            return true;
        }
        return false
    }

    fn is_falling_down(&mut self, index: usize) -> bool {
        if let Some(below_index) = self.get_relative_index(index, 0, 1) {
            if self.granules[below_index].element_type == ElementType::Water || 
                self.granules[below_index].element_type == ElementType::Empty {
                    let granule_below = self.granules[below_index];
                    self.granules[below_index] = self.granules[index];
                    self.granules[index] = granule_below;
                    self.granules[index].update_toggle = self.update_cycle;
                    self.granules[below_index].update_toggle = self.update_cycle;
                    return true;
            }
            else {
                self.granules[index].update_toggle = self.update_cycle;
                return false;
            }
            
        }
        else if !self.is_floor {
            self.granules[index].element_type = ElementType::Empty;
            return true;
        }

        return false;
    }

    fn update_egg(&mut self, index: usize) {
        let mut rng = rand::thread_rng();

        if self.is_falling_down(index) {
            return
        }
        else if self.is_falling_diagonal(index) {
            return
        }

        let rough_hatch_time_seconds = 1.0;
        let probability =  1.0 / (rough_hatch_time_seconds * 35.0);
        if rng.gen::<f64>() < probability {
            let mut depth = 20;
            for i in 1..self.height {
                let y = -1 * (i as i64);
                if let Some(depth_check_index) = self.get_relative_index(index, 0, y) {
                    if self.granules[depth_check_index].element_type != ElementType::Water &&
                        self.granules[depth_check_index].element_type != ElementType::Minnow {
                        depth = i;
                        break;
                    }
                }
                else {
                    depth = i;
                    break;
                }
                if depth > 255 {
                    depth = 255;
                    break;
                }
            }
            if depth < 20 {
                return;
            }
            self.granules[index].element_type = ElementType::Minnow;
            self.granules[index].moisture = rng.gen_range(3..(depth - 3) as u8);
            if rng.gen::<f64>() < 0.5 {
                self.granules[index].direction = true;
            }
            else {
                self.granules[index].direction = false;
            }
            
        }
        
    }

    fn update_minnow(&mut self, index: usize) {

        if self.is_falling_down_air(index) {
            return
        }
        else if self.is_falling_diagonal_air(index) {
            return
        }

        let mut rng = rand::thread_rng();
        let mut direction = 1;
        if !self.granules[index].direction {
            direction = -1;
        }
        let mut y = 0;
        if self.granules[index].moisture > 0 && rng.gen::<f64>() < 0.3 {
            y = 1;
            self.granules[index].moisture -= 1;
        }
        if let Some(new_index) = self.get_relative_index(index, direction, -y) {
            if self.granules[new_index].element_type == ElementType::Water {
                let temp = self.granules[new_index];
                self.granules[new_index] = self.granules[index];
                self.granules[index] = temp;
            }
            else {
                self.granules[index].switch_direction()
            }
            
        }
        else {
            self.granules[index].switch_direction()
        }
    }
    

    /* 
    fn update_minnow(&mut self, index: usize) {
        // height is distance from bottom? top?
        // 

        let buffer_distance = 2;

        for i in 1..buffer_distance {
            let mut x = i;
            if !self.granules[index].direction {
                x = -i;
            }
            let mut y = 0;
            if self.granules[index].moisture > 0 {
                y = 1;
                self.granules[index].moisture -= 1;
            }
            if let Some(new_index) = self.get_relative_index(index, x, -y) {
                if self.granules[new_index].element_type == ElementType::Water {
                    let temp = self.granules[new_index];
                    self.granules[new_index] = self.granules[index];
                    self.granules[index] = temp;
                }
                else {
                    self.granules[index].switch_direction()
                }
                
            }
            else {
                self.granules[index].switch_direction()
            }
        }
    }
*/
}
