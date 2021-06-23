//! Buffered graphics mode.

use crate::{consts::ssd1320z2, error::Error, Ssd1320z2};

use core::convert::TryInto;

use display_interface::{DisplayError, WriteOnlyDataCommand};
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use embedded_graphics_core::{
    pixelcolor::{Gray4, GrayColor},
    prelude::*,
};

#[derive(Copy, Clone, Debug)]
pub struct BufferedSsd1320z2<DI, CS1, CS2> {
    display: Ssd1320z2<DI, CS1, CS2>,
    framebuffer: [u8; ssd1320z2::NUM_BUF_COLS as usize * ssd1320z2::NUM_PIXELS_ROWS as usize],
}

impl<DI, CS1, CS2> BufferedSsd1320z2<DI, CS1, CS2>
where
    DI: WriteOnlyDataCommand,
    CS1: OutputPin,
    CS2: OutputPin,
{
    /// Create a SSD1320z2 interface
    pub fn new(interface: DI, cs1: CS1, cs2: CS2) -> Self {
        Self {
            display: Ssd1320z2::new(interface, cs1, cs2),
            framebuffer: [0; ssd1320z2::NUM_BUF_COLS as usize
                * ssd1320z2::NUM_PIXELS_ROWS as usize],
        }
    }

    /// Reset and init the display.
    pub fn init<RST, DELAY, PinE>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), Error<(), PinE>>
    where
        RST: OutputPin<Error = PinE>,
        DELAY: DelayMs<u8>,
    {
        self.display.reset(rst, delay)?;
        self.display.init();

        Ok(())
    }

    /// Updates the display from the framebuffer.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.display.draw(&self.framebuffer)
    }
}

impl<DI, CS1, CS2> OriginDimensions for BufferedSsd1320z2<DI, CS1, CS2> {
    fn size(&self) -> Size {
        Size::new(
            ssd1320z2::NUM_PIXELS_COLS as u32,
            ssd1320z2::NUM_PIXELS_ROWS as u32,
        )
    }
}

impl<DI, CS1, CS2> DrawTarget for BufferedSsd1320z2<DI, CS1, CS2> {
    type Color = Gray4;

    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            const X_END: u32 = ssd1320z2::PIXEL_COL_MAX as u32;
            const Y_END: u32 = ssd1320z2::PIXEL_ROW_MAX as u32;
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // (320, 132)). `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.
            if let Ok((x @ 0..=X_END, y @ 0..=Y_END)) = coord.try_into() {
                // Calculate the index in the framebuffer.
                let index = x / 2 + y * ssd1320z2::NUM_BUF_COLS as u32;
                let shift = if x % 2 == 0 { 0 } else { 4 };
                self.framebuffer[index as usize] &= !(0xf << shift);
                self.framebuffer[index as usize] |= (color.luma() << shift);
            }
        }

        Ok(())
    }
}
