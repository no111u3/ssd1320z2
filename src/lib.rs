#![no_std]
//! Generic SPI interface for display drivers

mod command;
mod display;
mod error;

use display_interface::{DisplayError, WriteOnlyDataCommand};
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use display::Ssd1320;

#[derive(Copy, Clone, Debug)]
struct Frame {
    start: (u16, u16),
    end: (u16, u16),
}

impl Frame {
    fn new() -> Self {
        Self {
            start: (0, 0),
            end: (320, 132),
        }
    }

    fn normalize(&self) -> Self {
        Self {
            start: (self.start.0 % 160, self.start.1),
            end: (self.end.0 % 160, self.end.1),
        }
    }

    fn split_to_two(&self) -> (Self, Self) {
        let start_x = self.start.0;
        let end_x = self.end.0;

        (
            Self {
                start: (start_x, self.start.1),
                end: (159, self.end.1),
            },
            Self {
                start: (0, self.start.1),
                end: (end_x, self.end.1),
            },
        )
    }

    fn as_u8(&self) -> ((u8, u8), (u8, u8)) {
        (
            (self.start.0 as u8, self.start.1 as u8),
            (self.end.0 as u8, self.end.1 as u8),
        )
    }
}

/// Variant for Surenoo SUR383S1000WG01
/// Two SSD1320 controllers on board with single interface
#[derive(Copy, Clone, Debug)]
pub struct Ssd1320z2<DI, CS1, CS2> {
    interface: Ssd1320<DI>,
    frame: Frame,
    position: (u16, u16),
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
            frame: Frame::new(),
            position: (0, 0),
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

    pub fn set_draw_area(
        &mut self,
        start: (u16, u16),
        end: (u16, u16),
    ) -> Result<(), DisplayError> {
        self.frame = Frame { start, end };
        if start.0 < 160 && end.0 >= 160 {
            let (one, two) = self.frame.split_to_two();
            let one = one.normalize().as_u8();
            let two = two.normalize().as_u8();
            self.position = (0, 0);
            self.select_one();
            self.interface.set_draw_area(one.0, one.1)?;
            self.select_two();
            self.interface.set_draw_area(two.0, two.1)?;
        } else {
            if start.0 < 160 {
                self.select_one();
            } else {
                self.select_two();
            }
            let adopted_frame = self.frame.normalize().as_u8();
            self.interface
                .set_draw_area(adopted_frame.0, adopted_frame.1)?;
        }
        self.unselect_all();

        Ok(())
    }

    pub fn draw(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        let Frame { start, end } = self.frame;
        if start.0 < 160 && end.0 >= 160 {
            // TODO: rewrite to more efficient way
            let x_size = end.0 - start.0;
            let x_limit = 160 - start.0;
            let y_size = end.1 - start.1;

            for i in buffer.iter() {
                if self.position.0 <= x_limit {
                    self.select_one();
                } else {
                    self.select_two();
                }
                self.interface.draw(&[*i])?;
                if self.position.0 < x_size - 1 {
                    self.position.0 += 2;
                } else {
                    if self.position.1 < y_size {
                        self.position.1 += 1;
                    } else {
                        self.position.1 = 0;
                    }
                    self.position.0 = 0;
                }
            }
        } else {
            if start.0 < 160 {
                self.select_one();
            } else {
                self.select_two();
            }
            self.interface.draw(buffer)?;
        }
        self.unselect_all();

        Ok(())
    }
}
