//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![no_main]

// dev profile
#[cfg(debug_assertions)]
extern crate panic_semihosting;

// release profile
#[cfg(not(debug_assertions))]
//extern crate panic_halt;
extern crate panic_semihosting;


use stm32f4xx_hal::{
    prelude::*,
    stm32,      //this is the pac (I think)
    delay::Delay,
    spi::{Spi, Mode, NoMiso, Phase, Polarity}
};

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
//use embedded_hal::digital::v2::OutputPin;

//use nb;
use nb::block;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::{Ws2812, MODE};


#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    debug::exit(debug::EXIT_SUCCESS);


    // core peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    // device peripherals, using the hal
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    //let flash = dp.FLASH;

    /*
    // Adjust flash wait states - copied from https://github.com/stm32-rs/stm32f4xx-hal/commit/2abc4a054fc257453a25611047629b7bc04f5c10#diff-c6941ad2425cd611249033bed1925f02
    // not needed
    unsafe {
        flash.acr.modify(|_, w|
            w.latency().bits(0x03)
            .prften().set_bit()
            .icen().set_bit()
            .dcen().set_bit()
        );
    }
    */

    let clocks = rcc.cfgr
        //.use_hse(25.mhz())        //no noticeable difference between HSE and the default HSI on the 'scope
        .sysclk(96.mhz())           //96MHz is still compatible with 48MHz USB
        .freeze();

    
    let gpioa = dp.GPIOA.split();   //without split(), we access the port and pins in the PAC way
    let sck = gpioa.pa5.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();
    
    /*
    let gpiob = dp.GPIOB.split();
    let sck = gpiob.pb3.into_alternate_af5();
    let mosi = gpiob.pb5.into_alternate_af5();
    */

    // Configure SPI with 3Mhz rate
    let mut spi = Spi::spi1(
        dp.SPI1,
        (sck, NoMiso, mosi),
        Mode {
            polarity: Polarity::IdleLow,    //this probably matters a lot
            phase: Phase::CaptureOnFirstTransition,
        },
        //3_000_000.hz(),
        stm32f4xx_hal::time::KiloHertz(3000).into(),
        clocks,
    );


    let mut ws = Ws2812::new(spi);

    let mut delay = Delay::new(cp.SYST, clocks);

    //From CubeMX
    /*
    /* SPI1 parameter configuration*/
    hspi1.Instance = SPI1;
    hspi1.Init.Mode = SPI_MODE_MASTER;
    hspi1.Init.Direction = SPI_DIRECTION_2LINES;
    hspi1.Init.DataSize = SPI_DATASIZE_8BIT;
    hspi1.Init.CLKPolarity = SPI_POLARITY_LOW;
    hspi1.Init.CLKPhase = SPI_PHASE_1EDGE;
    hspi1.Init.NSS = SPI_NSS_SOFT;
    hspi1.Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_2;
    hspi1.Init.FirstBit = SPI_FIRSTBIT_MSB;
    hspi1.Init.TIMode = SPI_TIMODE_DISABLE;
    hspi1.Init.CRCCalculation = SPI_CRCCALCULATION_DISABLE;
    hspi1.Init.CRCPolynomial = 10;

    //no mention is made of pins. It seems they are implied by af5 ????
    */

    /*
    //from spi::init()
    let br = match clocks.0 / 3.mhz() {
        0 => unreachable!(),
        1..=2 => 0b000,
        3..=5 => 0b001,
        6..=11 => 0b010,
        12..=23 => 0b011,
        24..=47 => 0b100,
        48..=95 => 0b101,
        96..=191 => 0b110,
        _ => 0b111,
    };

    dp.SPI1.cr1.write(|w| { w
        .cpha().clear_bit()
        .cpol().clear_bit()
        .mstr().set_bit()
        .br().bits(br)
        .lsbfirst().clear_bit()
        .ssm().set_bit()
        .ssi().set_bit()
        .rxonly().clear_bit()
        .dff().clear_bit()
        .bidimode().clear_bit()
        .spe().set_bit()
    });    
    */

    loop {
        /*
        //let result3 = block!({spi.send(0b01101010)});
        //let sr = dp.spi1.sr.read();
        //let sr = dp.SPI1.sr.read();     //cannot use dp.SPI1 here, it was moved into local "spi" variable
        if !spi.is_ovr() {
        //if sr.ovr().bit_is_set() {
            block!({
                // Some implementations (stm32f0xx-hal) want a matching read
                // We don't want to block so we just hope it's ok this way
                spi.read().ok();
                spi.send(0b10101010)

            });
        }
        */
        let mut data: [RGB8; 7] = [RGB8::default(); 7];
        //let empty: [RGB8; 1] = [RGB8::default(); 1];

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

        //hprintln!("Pre-ws2812 write").unwrap();

        //loop to keep retrying if spi overflow error
        //nb::Error::Other(Error::Overrun)
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
            //hprintln!("ws2812 write error!").unwrap();  //superslow
        }
        if error_count > 0 {
            //hprintln!("Error Count: {}", error_count).unwrap();
        }
        
        delay.delay_ms(10 as u16);  
        //keep writing the same data. We spot errors when the color changes
    }
    

}