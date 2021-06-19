#![no_std]
//! Generic SPI interface for display drivers

mod command;
mod display;
mod error;

use display_interface::{DisplayError, WriteOnlyDataCommand};
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use display::Ssd1320;

/// Variant for Surenoo SUR383S1000WG01
/// Two SSD1320 controllers on board with single interface
#[derive(Copy, Clone, Debug)]
pub struct Ssd1320z2<DI, CS1, CS2> {
    interface: Ssd1320<DI>,
    cs1: CS1,
    cs2: CS2,
}

impl<DI, CS1, CS2> Ssd1320z2<DI, CS1, CS2>
where
    DI: WriteOnlyDataCommand,
    CS1: OutputPin,
    CS2: OutputPin,
{
    /// Create a SSD1320z2 interface
    pub fn new(interface: DI, cs1: CS1, cs2: CS2) -> Self {
        Self {
            interface: Ssd1320::new(interface),
            cs1,
            cs2,
        }
    }

    fn select_one(&mut self) {
        self.cs2.set_high().ok();
        self.cs1.set_low().ok();
    }

    fn select_two(&mut self) {
        self.cs1.set_high().ok();
        self.cs2.set_low().ok();
    }

    fn select_all(&mut self) {
        self.cs1.set_low().ok();
        self.cs2.set_low().ok();
    }

    fn unselect_all(&mut self) {
        self.cs1.set_high().ok();
        self.cs2.set_high().ok();
    }

    pub fn init(&mut self) -> Result<(), DisplayError> {
        self.select_one();
        self.interface.init(0x0e, false, true)?;
        self.select_two();
        self.interface.init(0x92, true, false)?;
        self.unselect_all();

        Ok(())
    }

    pub fn draw(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.select_all();
        self.interface.draw(buffer)?;
        self.unselect_all();

        Ok(())
    }
}
