//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![no_main]

use panic_halt as _;

use stm32f4xx_hal::{
    prelude::*,
    stm32,      //this is the pac (I think)
    delay::Delay,
    spi::{Spi, Mode, Phase, Polarity}
};

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug};
//use embedded_hal::digital::v2::OutputPin;

use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::{Ws2812};


#[entry]
fn main() -> ! {
    //hprintln!("Hello, world!").unwrap();
    debug::exit(debug::EXIT_SUCCESS);


    // core peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    // device peripherals, using the hal
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr
        //.use_hse(25.mhz())        //no noticeable difference between HSE and the default HSI on the 'scope
        .sysclk(96.mhz())           //96MHz is still compatible with 48MHz USB
        .freeze();

    let gpioa = dp.GPIOA.split();   //without split(), we access the port and pins in the PAC way
    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();

    // Configure SPI with 3Mhz rate
    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        Mode {
            polarity: Polarity::IdleLow,    //this probably matters a lot
            phase: Phase::CaptureOnFirstTransition,
        },
        3_000_000.hz(),
        clocks,
    );

    let mut ws = Ws2812::new(spi);
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {

        let mut data: [RGB8; 7] = [RGB8::default(); 7];

        data[0] = RGB8 {
            r: 0xFF,
            g: 0x00,
            b: 0x00,
        };
        data[1] = RGB8 {
            r: 0x00,
            g: 0xFF,
            b: 0x00,
        };
        data[2] = RGB8 {
            r: 0x00,
            g: 0x00,
            b: 0xFF,
        }; 
        data[3] = RGB8 {
            r: 0xAA,
            g: 0xAA,
            b: 0x00,
        };
        data[4] = RGB8 {
            r: 0xEE,
            g: 0x44,
            b: 0x00,
        };
        data[5] = RGB8 {
            r: 0x88,
            g: 0x88,
            b: 0x88,
        };    
        data[6] = RGB8 {
            r: 0x55,
            g: 0x00,
            b: 0xEE,
        };                      

        //loop to keep retrying if spi overflow error
        let mut error_count = 0;
        loop {
            //we try 10x to send the data
            let result = ws.write(data.iter().cloned());    //.map_err(|_error| {
            if result.is_ok() || error_count >= 10 {
                break;
            } else {
                error_count += 1;
            }
            //we don't currently check what type of error it is. We should!
        }
        if error_count > 0 {
            //hprintln!("Error Count: {}", error_count).unwrap();
        }
        
        delay.delay_ms(10 as u16);  
        //keep writing the same data. We spot errors when the color changes
    }

}