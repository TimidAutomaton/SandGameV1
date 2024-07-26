
use std::time::*;

struct Timer {
    base_time: Instant,
    prev_time: u128,
    new_time: u128,
    diff: u128,
    fps: f32,
}


impl Timer {
    fn new() -> Self {

        let base_time = Instant::now();
        let prev_time = base_time.elapsed().as_micros();
        let new_time = base_time.elapsed().as_micros();
        let diff = new_time - prev_time;
        let fps = 0.0;

        Self {
            base_time,
            prev_time,
            new_time,
            diff,
            fps,
        }
    }

    fn get_time_reset(&mut self) -> u128 {
        self.new_time = self.base_time.elapsed().as_micros();
        self.diff = self.new_time - self.prev_time;
        self.prev_time = self.new_time;
        self.fps = 1.0 / (self.diff as f32 / 1_000_000.0);
        self.diff
    } 


    fn get_time_no_reset(&mut self) -> u128 {
        self.new_time = self.base_time.elapsed().as_micros();
        self.diff = self.new_time - self.prev_time;
        self.fps = 1.0 / (self.diff as f32 / 1_000_000.0);
        self.diff
    }

}


pub struct DebugTools {

    timers: Vec<Timer>,
}

impl DebugTools {
    pub fn new() -> Self {
        Self {
            timers: Vec::new(),
        }
    }

    pub fn add_timer(&mut self) -> usize {
        let new_timer = Timer::new();
        self.timers.push(new_timer);
        self.timers.len() - 1
    }

    pub fn remove_timer(&mut self, timer: usize) -> bool {
        if self.timers.len() < timer {
            self.timers.remove(timer);
            true
        }
        else {
            false
        }
    }

    // resets the timer to 0 and outputs the elapsed time since the last
    // reset.
    pub fn get_time_reset(&mut self, timer: usize) -> u128 {
        self.timers[timer].get_time_reset()
    }

    // Outputs the elapsed time since the last reset
    // may be renamed to read_elapsed_time
    pub fn get_time_no_reset(&mut self, timer: usize) -> u128 {
        self.timers[timer].get_time_no_reset()
    }

    // outputs the elapsed time that was output during a previous
    // update_timer() call
    pub fn read_last_recorded(&self, timer: usize) -> u128 {
        self.timers[timer].diff
    }

    pub fn read_last_recorded_as_fps(&self, timer: usize) -> f32 {
        self.timers[timer].fps
    }
}




const HEXTABLE: [&str; 16] = [  "0", "1", "2", "3",
                                "4", "5", "6", "7",
                                "8", "9", "A", "B",
                                "C", "D", "E", "F",];

pub fn byte_to_hex(byte: u8) -> String {
    let upper_nibble = byte  >> 4;
    let lower_nibble = byte & 0xF;
    let mut output = "".to_string();
    output = output + HEXTABLE[upper_nibble as usize] + HEXTABLE[lower_nibble as usize];
    output
}