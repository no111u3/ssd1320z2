#![no_std]
//! Generic SPI interface for display drivers

mod command;
mod display;
mod error;

/// Variant for Surenoo SUR383S1000WG01
/// Two SSD1320 controllers on board with single interface
#[derive(Copy, Clone, Debug)]
struct Ssd1320z2 {}

impl Ssd1320z2 {
    pub fn new() -> Self {
        Self {}
    }
}
