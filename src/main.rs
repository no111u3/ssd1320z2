#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m::peripheral::Peripherals;

use stm32f4xx_hal::{
    delay::Delay,
    prelude::*,
    spi::{self, Spi},
    stm32,
};

use display::Ssd1320;
use display_interface_spi::SPIInterfaceNoCS;

mod command;
mod display;
mod error;

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let cp = Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low().ok();

    let sck = gpiob.pb3.into_alternate_af5();
    let miso = spi::NoMiso;
    let mosi = gpiob.pb5.into_alternate_af5();

    let dc = gpiob.pb4.into_push_pull_output();
    let mut res = gpiob.pb10.into_push_pull_output();
    let cs1 = gpiob.pb13.into_push_pull_output();
    let cs2 = gpiob.pb14.into_push_pull_output();

    let mut delay = Delay::new(cp.SYST, clocks);

    let mode = spi::Mode {
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    };

    let mut spi = Spi::spi1(p.SPI1, (sck, miso, mosi), mode, 8_000_000.hz(), clocks);

    let mut reset = || {
        res.set_high().ok();
        delay.delay_ms(1u16);
        res.set_low().ok();
        delay.delay_ms(10u16);
        res.set_high().ok();
        delay.delay_ms(20u16);
    };

    let iface = SPIInterfaceNoCS::new(spi, dc);

    let mut display = Ssd1320::new(iface, cs1, cs2);

    reset();

    display.init();

    /*#[derive(PartialEq)]
    #[allow(dead_code)]
    enum ExchangeMode {
        Command,
        Data,
    }

    #[derive(PartialEq)]
    enum Controller {
        One,
        Two,
        All,
    }

    let mut oled_wr_byte = |data: u8, mode: ExchangeMode, number: Controller| {
        if mode == ExchangeMode::Command {
            dc.set_low().ok();
        } else {
            dc.set_high().ok();
        }

        if number == Controller::One || number == Controller::All {
            cs1.set_low().ok();
        }
        if number == Controller::Two || number == Controller::All {
            cs2.set_low().ok();
        }

        spi.write(&[data]).ok();

        if number == Controller::One || number == Controller::All {
            cs1.set_high().ok();
        }
        if number == Controller::Two || number == Controller::All {
            cs2.set_high().ok();
        }
        dc.set_high().ok();
    };

    reset();
    oled_wr_byte(0xae, ExchangeMode::Command, Controller::All); // Display OFF
    oled_wr_byte(0xfd, ExchangeMode::Command, Controller::All); // Set Command Lock
    oled_wr_byte(0x12, ExchangeMode::Command, Controller::All);
    oled_wr_byte(0x20, ExchangeMode::Command, Controller::All); // Set Memory addressing mode
    oled_wr_byte(0x00, ExchangeMode::Command, Controller::All);
    oled_wr_byte(0x25, ExchangeMode::Command, Controller::All); //Set Portrait Addressing Mode
    oled_wr_byte(0x00, ExchangeMode::Command, Controller::All); //Normal Addressing Mode
    oled_wr_byte(0x81, ExchangeMode::Command, Controller::All); //Set Contrast Control
    oled_wr_byte(0x6b, ExchangeMode::Command, Controller::All);
    oled_wr_byte(0xa0, ExchangeMode::Command, Controller::One); //Set Seg Remap
    oled_wr_byte(0xa1, ExchangeMode::Command, Controller::Two);
    oled_wr_byte(0xa2, ExchangeMode::Command, Controller::All); //Set Display Start Line
    oled_wr_byte(0x00, ExchangeMode::Command, Controller::All);
    oled_wr_byte(0xa4, ExchangeMode::Command, Controller::All); //Resume to RAM content display
    oled_wr_byte(0xa6, ExchangeMode::Command, Controller::All); //Set Normal Display
    oled_wr_byte(0xa8, ExchangeMode::Command, Controller::All); //Set MUX Ratio
    oled_wr_byte(0x83, ExchangeMode::Command, Controller::All); //1/132 duty
    oled_wr_byte(0xad, ExchangeMode::Command, Controller::All); //Select external or internal IREF
    oled_wr_byte(0x10, ExchangeMode::Command, Controller::All);
    oled_wr_byte(0xbc, ExchangeMode::Command, Controller::All); //Set Pre-charge voltage
    oled_wr_byte(0x1e, ExchangeMode::Command, Controller::All); //
    oled_wr_byte(0xbf, ExchangeMode::Command, Controller::All); //Linear LUT
    oled_wr_byte(0xc8, ExchangeMode::Command, Controller::One); //Set COM Output Scan Direction
                                                                //oled_wr_byte(0xc0, ExchangeMode::Command, Controller::Two);
    oled_wr_byte(0xd3, ExchangeMode::Command, Controller::All); //Set Display Offset
    oled_wr_byte(0x0e, ExchangeMode::Command, Controller::One);
    oled_wr_byte(0x92, ExchangeMode::Command, Controller::Two);
    oled_wr_byte(0xd5, ExchangeMode::Command, Controller::All); //Set Display Clock Divide Ratio/Oscillator Frequency
    oled_wr_byte(0x72, ExchangeMode::Command, Controller::All); //8Hz
    oled_wr_byte(0xd9, ExchangeMode::Command, Controller::All); //Set Pre-charge Period
    oled_wr_byte(0x72, ExchangeMode::Command, Controller::All);
    oled_wr_byte(0xda, ExchangeMode::Command, Controller::All); //Set SEG Pins Hardware Configuration
    oled_wr_byte(0x32, ExchangeMode::Command, Controller::All);
    //oled_wr_byte(0xbd, ExchangeMode::Command, Controller::All); //Set VP
    //oled_wr_byte(0x03, ExchangeMode::Command, Controller::All);
    oled_wr_byte(0xdb, ExchangeMode::Command, Controller::All); //Set VCOMH
    oled_wr_byte(0x30, ExchangeMode::Command, Controller::All);
    oled_wr_byte(0xaf, ExchangeMode::Command, Controller::All); //Display on
    */

    let mut is_first = true;

    loop {
        delay.delay_ms(500_u16);
        for i in 0..80 {
            display
                .draw(&[0x77, 0x77, 0x77, 0x77, 0xff, 0xff, 0xff, 0xff])
                .ok();
        }
        for i in 0..80 {
            display
                .draw(&[0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00])
                .ok();
        }
        led.toggle().ok();
        is_first ^= true;
    }
}
