#![no_std]
#![no_main]
/// main.rs
/// This file is the main code that runs our RGB LED control program for the micro:bit v2

/// module declaration
mod knob;
mod rgb;
mod ui;
pub use knob::*;
pub use rgb::*;
pub use ui::*;

// crates to handle panics and print errors for debugging
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// embassy crate and microbit crate used in the program
use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    Button, Microbit,
};
use num_traits::float::FloatCore;

/// static variables used to define
/// RGB LED levels
/// the frame rate
/// the number of levels used in the program
pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);
pub static FRAME_RATE: Mutex<ThreadModeRawMutex, u32> = Mutex::new(10);
pub const LEVELS: u32 = 16;

/// function to get the RGB LED levels
async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

/// function to set the RGB LED levels
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

/// function to get the frame rate
async fn get_frame_rate() -> u32 {
    let frame_rate = FRAME_RATE.lock().await;
    *frame_rate
}

/// function to set the frame rate
async fn set_frame_rate<F>(setter: F) 
where
    F: FnOnce(&mut u32),
{
    let mut frame_rate = FRAME_RATE.lock().await;
    setter(&mut frame_rate);
}

/// main function of the program
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    // initialize the rtt print function for debugging
    rtt_init_print!();
    // initialize the microbit board
    let board = Microbit::default();

    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    // initialize the RGB LED to the corresponding pins on the board
    // RED is on pin P9
    // GREEN is on pin P8
    // BLUE is on pin P16
    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));
    let rgb: Rgb = Rgb::new([red, green, blue], 100);

    // configuration of saadc from embassy nrf crate 
    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );

    // initialize the knob and ui
    let knob = Knob::new(saadc).await;
    let mut ui = Ui::new(knob, board.btn_a, board.btn_b);

    // running the rgb and ui tasks concurrently
    join::join(rgb.run(), ui.run()).await;

    panic!("fell off end of main loop");
}
