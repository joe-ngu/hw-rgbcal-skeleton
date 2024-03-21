# rgbcal: RGB LED calibration tool

HW Completed by: Joseph Nguyen

Bart Massey 2024

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

**XXX This tool is _mostly_ finished! Please wire your
hardware up (see below), finish it, comment it, and use it
to find good values. Then document those values in this
README.**

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

## Wiring

Connect the RGB LED to the MB2 as follows:

- Red to P9 (GPIO1)
- Green to P8 (GPIO2)
- Blue to P16 (GPIO3)
- Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

- Pin 1 to Gnd
- Pin 2 to P2
- Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
jus always controls Blue. You should see the color change
from green to teal-blue as you turn the knob clockwise.)

- No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160.
- A button held: Change the blue level from off to on over
  16 steps.
- B button held: Change the green level from off to on over
  16 steps.
- A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scanout code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason.

I think the frame rate is probably set higher than it needs
to be right now: it can be tuned lower.

## RGB Calibration

In order to achieve white light, the RGB values that I found worked best for me was:
RED: 15
GREEN: 6
BLUE: 7
Frame rate: 70

## Write-up

First I started doing the wiring part and it was probably one of the more difficult parts for me
because I am not too familiar with handling hardware. Next, I went through and added comments to the code.
This was a good way for me to learn what the code was doing and I was able to reuse some of the logic
presented in the code for the other functionalities of the program discussed later. The first thing I edited
was the ui.rs file. Instead of having the logic only deal with the blue led, I would have the logic apply to
either red, green, or blue depending on which buttons were being pressed. Finally, I had to handle the frame
rate updates and this was probably the trickiest part of the software for me. I first created functions to
handle frame rate just like the functions to handle leds in the main.rs file. I reused some of the logic there.
Next I updated the ui.rs file approriately as well as the rgb.rs file to ensure the frame rate was handled
correctly.

Overall this was a tough assignment for me, but an interesting one where I learned a lot about how to use the
embassy crate to control leds. While working on this project, I wonder if there were alternative ways to control
the leds. For instance, could the microbit's microhpone be used to control the leds with sound instead of the knob?
The knob felt a little flimsy to me and in a real world scenario if it were to break, I wonder how we would handle
that.
