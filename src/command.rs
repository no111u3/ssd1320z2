//! Display commands

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

/// SSD1320 Commands

/// Commands
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Command {
    /// Set addressing mode
    AddressMode(AddrMode),
    /// Setup column start and end address
    /// values range from 0-79
    ColumnAddress(u8, u8),
    /// Setup row start and end address
    /// values range from 0-159
    RowAddress(u8, u8),
    /// Set portrait addressing mode
    PortraitAddressMode(PortraitAddrMode),
    /// Set contrast. Higher number is higher contrast. Default = 0x7F
    Contrast(u8),
    /// Reverse columns from 79-0
    SegmentRemap(bool),
    /// Set display start line from 0-159
    StartLine(u8),
    /// Turn entire display on. If set, all pixels will
    /// be set to on, if not, the value in memory will be used.
    AllOn(bool),
    /// Invert display.
    Invert(bool),
    /// Set multipex ratio from 16-160 (MUX+1)
    Multiplex(u8),
    /// Select external or internal I REF.
    InternalIref(bool),
    /// Turn display on or off.
    DisplayOn(bool),
    /// Set pre-charge volage level
    /// must be smaller than COM deselect volage level
    PreChargeLevel(PreChargeLvl),
    /// No mapped command TODO: search or reverse meaning of command
    VP,
    /// The default Lineral Gray Scale table is in unit
    /// of DCLK's as follow
    /// GS0 level pulse width = 0
    /// GS1 level pulse width = 4
    /// GS2 level pulse width = 8
    /// GS3 level pulse width = 12
    /// ...
    /// GS14 level pulse width = 56
    /// GS15 level pulse width = 60
    LineralLUT,
    /// Scan from COM[n-1] to COM0 (where N is mux ratio)
    ReverseComDir(bool),
    /// Set vertical shift
    DisplayOffset(u8),
    /// Set up display clock.
    /// First value is oscillator frequency, increasing with higher value
    /// Second value is divide ratio (1, 2, 4, 8 ... 256)
    DisplayClockDiv(u8, u8),
    /// Set up phase 1 and 2 of precharge period. Each value must be in the range 1 - 15.
    PreChargePeriod(u8, u8),
    /// Setup com hardware configuration
    /// First value indicates sequential (false) or alternative (true)
    /// pin configuration. Second value disables (false) or enables (true)
    /// left/right remap.
    ComPinConfig(bool, bool),
    /// Set Vcomh Deselect level
    VcomhDeselect(VcomhLevel),
    /// Display lock
    DisplayLock(bool),
}

impl Command {
    /// Send command to SSD1320
    pub fn send<DI>(self, iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: WriteOnlyDataCommand,
    {
        // Transform command into a fixed size array of 7 u8 and the real length for sending
        let (data, len) = match self {
            Command::AddressMode(mode) => ([0x20, mode as u8, 0, 0, 0, 0, 0], 2),
            Command::ColumnAddress(start, end) => ([0x21, start, end, 0, 0, 0, 0], 3),
            Command::RowAddress(start, end) => ([0x22, start, end, 0, 0, 0, 0], 3),
            Command::PortraitAddressMode(mode) => ([0x25, mode as u8, 0, 0, 0, 0, 0], 2),
            Command::Contrast(val) => ([0x81, val, 0, 0, 0, 0, 0], 2),
            Command::SegmentRemap(remap) => ([0xA0 | (remap as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::StartLine(line) => ([0xA2, 0x3F & line, 0, 0, 0, 0, 0], 2),
            Command::AllOn(on) => ([0xA4 | (on as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::Invert(inv) => ([0xA6 | (inv as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::Multiplex(ratio) => ([0xA8, ratio, 0, 0, 0, 0, 0], 2),
            Command::InternalIref(on) => ([0xAD, (on as u8) << 4, 0, 0, 0, 0, 0], 2),
            Command::DisplayOn(on) => ([0xAE | (on as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::PreChargeLevel(level) => ([0xBC, level as u8, 0, 0, 0, 0, 0], 2),
            Command::VP => ([0xBD, 0x03, 0, 0, 0, 0, 0], 2),
            Command::LineralLUT => ([0xBF, 0, 0, 0, 0, 0, 0], 1),
            Command::ReverseComDir(rev) => ([0xC0 | ((rev as u8) << 3), 0, 0, 0, 0, 0, 0], 1),
            Command::DisplayOffset(offset) => ([0xD3, offset, 0, 0, 0, 0, 0], 2),
            Command::DisplayClockDiv(fosc, div) => {
                ([0xD5, ((0xF & fosc) << 4) | (0xF & div), 0, 0, 0, 0, 0], 2)
            }
            Command::PreChargePeriod(phase1, phase2) => (
                [0xD9, ((0xF & phase2) << 4) | (0xF & phase1), 0, 0, 0, 0, 0],
                2,
            ),
            Command::ComPinConfig(alt, lr) => (
                [
                    0xDA,
                    0x2 | ((alt as u8) << 4) | ((lr as u8) << 5),
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                2,
            ),
            Command::VcomhDeselect(level) => ([0xDB, (level as u8) << 4, 0, 0, 0, 0, 0], 2),
            Command::DisplayLock(lock) => ([0xFD, 0x12 | ((lock as u8) << 2), 0, 0, 0, 0, 0], 2),
        };

        // Send command over the interface
        iface.send_commands(U8(&data[0..len]))
    }
}

/// Address mode
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum AddrMode {
    /// Horizontal mode
    Horizontal = 0b00,
    /// Vertical mode
    Vertical = 0b01,
}

/// Address mode
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum PortraitAddrMode {
    /// Horizontal mode
    Normal = 0b00,
    /// Vertical mode
    Portrait = 0b01,
}

/// Pre-charge level
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum PreChargeLvl {
    /// 0.10 * Vcc
    V010 = 0b00000,
    /// 0.50 * Vcc
    V050 = 0b11110,
    /// 0.5133 * Vcc
    V05133 = 0b11111,
}

/// Vcomh Deselect level
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum VcomhLevel {
    /// 0.72 * Vcc
    V072 = 0b000,
    /// 0.76 * Vcc
    V076 = 0b010,
    /// 0.80 * Vcc
    V080 = 0b100,
    /// 0.84 * Vcc
    V084 = 0b110,
}
