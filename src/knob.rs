/// knob.rs
/// This file contains code from implementing the knob struct
/// which is used to adjust the light on our microbit board

use crate::*;

pub type Adc = saadc::Saadc<'static, 1>;

pub struct Knob(Adc);

/// implementation of the Knob struct which represents the potentionmeter
impl Knob {
    // initializes a new instance of the Knob which is calibrated
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }

    // measures the value from the sample function that is then scaled and returns
    // a result based off of some computations
    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0];
        self.0.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        let scaled = raw as f32 / 10_000.0;
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}
