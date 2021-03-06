//! SSD1320 OLED display driver.
//!
//! TODO: Create a normal documentation

use crate::command::{AddrMode, Command, PortraitAddrMode, PreChargeLvl, VcomhLevel};
use crate::error::Error;

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use display_interface_spi::SPIInterfaceNoCS;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

/// SSD1320 driver.
#[derive(Copy, Clone, Debug)]
pub struct Ssd1320<DI> {
    interface: DI,
}

impl<DI> Ssd1320<DI>
where
    DI: WriteOnlyDataCommand,
{
    /// Create a SSD1320 interface
    pub fn new(interface: DI) -> Self {
        Self { interface }
    }

    /// Initialise the display in one of the available addressing modes.
    /// TODO: Add address setup
    pub fn init(
        &mut self,
        display_offset: u8,
        regmap: bool,
        com_reverse: bool,
    ) -> Result<(), DisplayError> {
        Command::DisplayLock(false).send(&mut self.interface)?;
        Command::DisplayOn(false).send(&mut self.interface)?;
        Command::DisplayClockDiv(0x7, 0x2).send(&mut self.interface)?;
        Command::Multiplex(0x83).send(&mut self.interface)?;
        Command::DisplayOffset(display_offset).send(&mut self.interface)?;
        Command::AddressMode(AddrMode::Horizontal).send(&mut self.interface)?;
        Command::PortraitAddressMode(PortraitAddrMode::Normal).send(&mut self.interface)?;
        Command::StartLine(0).send(&mut self.interface)?;
        Command::Contrast(0x70).send(&mut self.interface)?;
        Command::SegmentRemap(regmap).send(&mut self.interface)?;
        Command::AllOn(false).send(&mut self.interface)?;
        Command::Invert(false).send(&mut self.interface)?;
        Command::InternalIref(true).send(&mut self.interface)?;
        Command::PreChargeLevel(PreChargeLvl::V050).send(&mut self.interface)?;
        Command::LineralLUT.send(&mut self.interface)?;
        Command::ReverseComDir(com_reverse).send(&mut self.interface)?;
        Command::PreChargePeriod(0xa, 0x0).send(&mut self.interface)?;
        Command::ComPinConfig(true, false).send(&mut self.interface)?;
        Command::VP.send(&mut self.interface)?;
        Command::VcomhDeselect(VcomhLevel::V080).send(&mut self.interface)?;
        Command::DisplayOn(true).send(&mut self.interface)?;
        Ok(())
    }

    /// Send a raw buffer to the display.
    pub fn draw(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.interface.send_data(U8(&buffer))
    }

    /// Turn the display on or off. The display can be drawn to and retains all
    /// of its memory even while off.
    pub fn set_display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        Command::DisplayOn(on).send(&mut self.interface)
    }

    /// Set the position in the framebuffer of the display limiting where any sent data should be
    /// drawn. This method can be used for changing the affected area on the screen as well
    /// as (re-)setting the start point of the next `draw` call.
    pub fn set_draw_area(&mut self, start: (u8, u8), end: (u8, u8)) -> Result<(), DisplayError> {
        Command::ColumnAddress(start.0 / 2, end.0 / 2).send(&mut self.interface)?;

        Command::RowAddress(start.1.into(), (end.1).into()).send(&mut self.interface)?;

        Ok(())
    }

    /// Set the column address (column 2px)in the framebuffer of the display where any sent data should be
    /// drawn.
    pub fn set_column(&mut self, column: u8) -> Result<(), DisplayError> {
        Command::ColumnAddress(column, 160 / 2 - 1).send(&mut self.interface)
    }

    /// Set the page address in the framebuffer of the display where any sent data
    /// should be drawn.
    pub fn set_row(&mut self, row: u8) -> Result<(), DisplayError> {
        Command::RowAddress(row, 0x83).send(&mut self.interface)
    }

    /// Reset the display.
    pub fn reset<RST, DELAY, PinE>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), Error<(), PinE>>
    where
        RST: OutputPin<Error = PinE>,
        DELAY: DelayMs<u8>,
    {
        inner_reset(rst, delay)
    }
}

fn inner_reset<RST, DELAY, PinE>(rst: &mut RST, delay: &mut DELAY) -> Result<(), Error<(), PinE>>
where
    RST: OutputPin<Error = PinE>,
    DELAY: DelayMs<u8>,
{
    rst.set_high().map_err(Error::Pin)?;
    delay.delay_ms(1);
    rst.set_low().map_err(Error::Pin)?;
    delay.delay_ms(10);
    rst.set_high().map_err(Error::Pin)?;
    delay.delay_ms(20);

    Ok(())
}
