use crate::*;

struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

/// implementation of the UiState struct
impl UiState {
    // This method prints the current state of the UI to the console.
    fn show(&self) {
        // names refer to the three state of the led lights
        let names = ["red", "green", "blue"];
        rprintln!();
        // prints the current state of the led lights (names and levels)
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        // prints the current frame rate
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

// define the default value for the UiState struct
impl Default for UiState {
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

/// implementation of the Ui struct
impl Ui {
    // function creates a new instance of the Ui struct with each of its components
    // the knob for adjusting the RGB leds
    // buttons a and b for user input
    // the UI state
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    pub async fn update_led_state(&mut self, rgb_color: usize, level: u32) {
        if level != self.state.levels[rgb_color] {
            self.state.levels[rgb_color] = level;
            self.state.show();
            set_rgb_levels(|rgb| {
                *rgb = self.state.levels;
            })
            .await;
        }
    }

    // function to run the UI state handler
    pub async fn run(&mut self) -> ! {
        // rgb led setup

        // RED   - 0: default color
        // GREEN - 1: if button b is pressed
        // BLUE  - 2: if button a is pressed
        let mut rgb_color = 0;
        if self._button_b.is_low() {
            rgb_color = 1;
        } else if self._button_a.is_low() {
            rgb_color = 2;
        }
        // measure rgb led levels
        self.state.levels[rgb_color] = self.knob.measure().await;
        // set the rgb led levels
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        // show the current state of the UI to the console
        self.state.show();

        // frame rate step value
        let frame_rate_step = 10;
        
        loop {
            let level = self.knob.measure().await;

            // if no butons pressed, change frame rate in steps of 10 frames per second 
            if self._button_a.is_high() && self._button_b.is_high() {
                self.state.frame_rate = ((level * frame_rate_step) + frame_rate_step) as u64;
                set_frame_rate(|frame_rate| {
                    *frame_rate = self.state.frame_rate as u32;
                }).await
            }
            // if both buttons pressed, update the RED led level
            else if self._button_a.is_low() && self._button_b.is_low() {
                self.update_led_state(0, level).await;
            }
            // if button b pressed, update the GREEN led level
            else if self._button_b.is_low() {
                self.update_led_state(1, level).await;
            }
            // if button a pressed, update the BLUE led level
            else if self._button_a.is_low() {
                self.update_led_state(2, level).await;
            }

            Timer::after_millis(50).await;
        }
    }
}
