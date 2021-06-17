//! SSD1320 OLED display driver.
//!
//! TODO: Create a normal documentation

use crate::command::{AddrMode, Command, PortraitAddrMode, PreChargeLvl, VcomhLevel};
use crate::error::Error;
use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use display_interface_spi::{SPIInterface, SPIInterfaceNoCS};
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

/// SSD1320 driver.
#[derive(Copy, Clone, Debug)]
pub struct Ssd1320<DI, CS1, CS2> {
    interface: DI,
    cs1: CS1,
    cs2: CS2,
}

impl<DI, CS1, CS2> Ssd1320<DI, CS1, CS2>
where
    DI: WriteOnlyDataCommand,
    CS1: OutputPin,
    CS2: OutputPin,
{
    /// Create a SSD1320 interface
    pub fn new(interface: DI, cs1: CS1, cs2: CS2) -> Self {
        Self {
            interface,
            cs1,
            cs2,
        }
    }

    /// Initialise the display in one of the available addressing modes.
    /// TODO: Add address setup
    pub fn init_one(
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
        Command::Contrast(0x6b).send(&mut self.interface)?;
        Command::SegmentRemap(regmap).send(&mut self.interface)?;
        Command::AllOn(false).send(&mut self.interface)?;
        Command::Invert(false).send(&mut self.interface)?;
        Command::InternalIref(true).send(&mut self.interface)?;
        Command::PreChargeLevel(PreChargeLvl::V050).send(&mut self.interface)?;
        Command::LineralLUT.send(&mut self.interface)?;
        Command::ReverseComDir(com_reverse).send(&mut self.interface)?;
        Command::PreChargePeriod(0xc, 0x2).send(&mut self.interface)?;
        Command::ComPinConfig(true, true).send(&mut self.interface)?;
        Command::VP.send(&mut self.interface)?;
        Command::VcomhDeselect(VcomhLevel::V080).send(&mut self.interface)?;
        Command::DisplayOn(true).send(&mut self.interface)?;
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), DisplayError> {
        self.cs1.set_low().ok();
        self.cs2.set_high().ok();
        self.init_one(0x0e, false, true)?;
        self.cs2.set_low().ok();
        self.cs1.set_high().ok();
        self.init_one(0x92, true, false)?;
        self.cs1.set_high().ok();
        self.cs2.set_high().ok();

        Ok(())
    }

    pub fn draw(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.cs1.set_low().ok();
        self.cs2.set_low().ok();
        self.interface.send_data(U8(&buffer))?;
        self.cs1.set_high().ok();
        self.cs2.set_high().ok();

        Ok(())
    }
}
