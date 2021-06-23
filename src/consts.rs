//! Constant describing max supported display size and the display RAM layout

/// SSD1320 constants
pub mod ssd1320 {
    /// The maximum supported display size width in pixels.
    pub const NUM_PIXELS_COLS: u16 = 160;

    /// The maximum supported display size height in pixels.
    pub const NUM_PIXELS_ROWS: u16 = 160;

    /// The number of display RAM columns addresses.
    pub const NUM_BUF_COLS: u16 = NUM_PIXELS_COLS / 2;

    // The highest valid pixel column index.
    pub const PIXEL_COL_MAX: u16 = NUM_PIXELS_COLS - 1;

    // The highest valid pixel row index.
    pub const PIXEL_ROW_MAX: u16 = NUM_PIXELS_ROWS - 1;

    // The highest valid display RAM column address.
    pub const BUF_COL_MAX: u16 = NUM_BUF_COLS - 1;
}

/// SSD1320Z2 (Surenoo SUR383S1000WG01 display) constants
pub mod ssd1320z2 {
    /// The maximum supported display size width in pixels.
    pub const NUM_PIXELS_COLS: u16 = 320;

    /// The maximum supported display size height in pixels.
    pub const NUM_PIXELS_ROWS: u16 = 132;

    /// The number of display RAM columns addresses.
    pub const NUM_BUF_COLS: u16 = NUM_PIXELS_COLS / 2;

    // The highest valid pixel column index.
    pub const PIXEL_COL_MAX: u16 = NUM_PIXELS_COLS - 1;

    // The highest valid pixel row index.
    pub const PIXEL_ROW_MAX: u16 = NUM_PIXELS_ROWS - 1;
}
