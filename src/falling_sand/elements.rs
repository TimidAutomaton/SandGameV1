use rand::Rng;

const STATIC_ELEMENTS: [ElementType; 1] = [ElementType::Wall];
const SOLID_ELEMENTS: [ElementType; 3] = [ElementType::Sand, ElementType::Dirt, ElementType::Seed];
const LIQUID_ELEMENTS: [ElementType; 2] = [ElementType::Water, ElementType::Empty];
const GAS_ELMENTS: [ElementType; 2] = [ElementType::Empty, ElementType::Cloud];
const LIVING_ELEMENTS: [ElementType; 1] = [ElementType::Grass];
const SPECIAL_ELEMENTS: [ElementType; 0] = [];

#[derive(PartialEq, Copy, Clone)]
pub enum ElementType {
    Empty,
    Sand,
    Water,
    Dirt,
    Seed,
    Grass,
    Kelp,
    Wall,
    Moss, // extends around walls and objects
    Cloud,
    Egg,
    Frog,
    Tadpole,
    Isopod,
    Minnow,
    Snail,
    SpringTail,
    ScreenEdge,
}


#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Solid,
    Granules,
    Liquid,
    Gas,
}


#[derive(Copy, Clone)]
pub struct Element {
    pub element_type: ElementType,
    pub state: State,
    pub update_toggle: bool,
    pub moisture: u8,
    pub hunger: u8,
    pub growth: u8,
    pub direction: bool,

}

impl Element {
    pub fn new(element_type: ElementType) -> Self {
        let mut rng_seed = rand::thread_rng();
        match element_type {
            ElementType::Empty => {
                return Self {
                    element_type: ElementType::Empty,
                    state: State::Gas,
                    update_toggle: false,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            },
            ElementType::Sand => {
                return Self {
                    element_type: ElementType::Sand,
                    state: State::Granules,
                    update_toggle: false,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            },
            ElementType::Water => {
                return Self {
                    element_type: ElementType::Water,
                    state: State::Liquid,
                    update_toggle: false,
                    moisture: 255,
                    hunger: 0,
                    growth: 0,
                    direction: rng_seed.gen_bool(0.5),
                }
            },
            ElementType::Dirt => {
                return Self {
                    element_type: ElementType::Dirt,
                    state: State::Granules,
                    update_toggle: false,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            },
            ElementType::Wall => {
                return Self {
                    element_type: ElementType::Wall,
                    state: State::Solid,
                    update_toggle: false,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            },
            ElementType::Seed => {
                return Self {
                    element_type: ElementType::Seed,
                    state: State::Granules,
                    update_toggle: false,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            },
            ElementType::Grass => {
                return Self {
                    element_type: ElementType::Grass,
                    state: State::Solid,
                    update_toggle: false,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            },
            ElementType::Kelp => {
                return Self {
                    element_type: ElementType::Kelp,
                    state: State::Solid,
                    update_toggle: false,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            },
            ElementType::Egg => {
                return Self {
                    element_type: ElementType::Egg,
                    state: State::Granules,
                    update_toggle: false,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            },
            ElementType::Minnow => {
                return Self {
                    element_type: ElementType::Minnow,
                    state: State::Granules,
                    update_toggle: false,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            },
            ElementType::ScreenEdge => {
                return Self {
                    element_type: ElementType::ScreenEdge,
                    state: State::Solid,
                    update_toggle: false,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            },

            _ => {
                return Self {
                    element_type: ElementType::Empty,
                    update_toggle: false,
                    state: State::Gas,
                    moisture: 0,
                    hunger: 0,
                    growth: 0,
                    direction: false,
                }
            }
        }
    }

    pub fn switch_direction(&mut self) {
        if self.direction {
            self.direction = false;
        }
        else {
            self.direction = true;
        }
    }
}


//const ElementProperties [element; 7] = [
  //  Element {
  //      element_type: Sand,
  //      i
 //   }
//]
