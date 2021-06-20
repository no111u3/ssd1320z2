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

use display_interface_spi::SPIInterfaceNoCS;
use ssd1320::Ssd1320z2;

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

    let spi = Spi::spi1(p.SPI1, (sck, miso, mosi), mode, 8_000_000.hz(), clocks);

    let mut reset = || {
        res.set_high().ok();
        delay.delay_ms(1u16);
        res.set_low().ok();
        delay.delay_ms(10u16);
        res.set_high().ok();
        delay.delay_ms(20u16);
    };

    let iface = SPIInterfaceNoCS::new(spi, dc);

    let mut display = Ssd1320z2::new(iface, cs1, cs2);

    reset();

    display.init();

    display.set_draw_area((10, 10), (40, 40));

    loop {
        delay.delay_ms(500_u16);
        for _ in 0..80 {
            display
                .draw(&[0x77, 0x77, 0x77, 0x77, 0xff, 0xff, 0xff, 0xff])
                .ok();
        }
        for _ in 0..80 {
            display
                .draw(&[0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00])
                .ok();
        }
        led.toggle().ok();
    }
}
