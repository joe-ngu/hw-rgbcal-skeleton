/// rgb.rs
/// This file contains the code for defining and controlling the RGB LEDs

use crate::*;

type RgbPins = [Output<'static, AnyPin>; 3];

pub struct Rgb {
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    tick_time: u64,
}

impl Rgb {
    // function to calculate frame ticks
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    // initialize a new instance of the RGB leds with pins and frame rate as parameters
    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    // this function represents one step in the program
    // it manages when an led should be turned on and off and the duration of each
    async fn step(&mut self, led: usize) {
        // get the current level of the led to determine if it should be turned on
        let level = self.levels[led];
        // if level is greater tha n0
        if level > 0 {
            // turn on led and wait for the duration of on_time, computed by multilying
            // the level value with the tick_time
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            // turn off led 
            self.rgb[led].set_low();
        }
        // wait for the duration of off_time, computed by multilying
        // the remaining level value with the tick_time
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    // function to run the RGB leds handler
    pub async fn run(mut self) -> ! {
        loop {
            // get rgb levels
            self.levels = get_rgb_levels().await;
            // get tick time by using frame rate
            self.tick_time = Self::frame_tick_time(get_frame_rate().await as u64);

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
