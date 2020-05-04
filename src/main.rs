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
    timer::Timer,
    delay::Delay,
};

use cortex_m_rt::entry;
//use embedded_hal::digital::v2::OutputPin;

//use nb;
//use nb::block;

use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_timer_delay::Ws2812;


#[entry]
fn main() -> ! {
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
    //the pll variables are all calculated in the badly named freeze() function
    unsafe { &*RCC::ptr() }.pllcfgr.write(|w| unsafe {
        w.pllm().bits(pllm as u8);
        w.plln().bits(plln as u16);
        w.pllp().bits(pllp as u8);
        w.pllq().bits(pllq as u8);
        w.pllsrc().bit(self.hse.is_some())
    });
    */

    //let clocks = rcc.cfgr.freeze();   //inadequate for full speed. Uses HSI by default

    let clocks = rcc.cfgr
        //.use_hse(25.mhz())        //no noticeable difference between HSE and the default HSI on the 'scope
        .sysclk(72.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split();   //without split(), we access the port and pins in the PAC way

    let pin = gpioa.pa7
        //.into_alternate__afo()
        .into_push_pull_output()
        //.into_open_drain_output()     //creates a sawtooth output
        //.set_speed(stm32f4xx_hal::gpio::Speed::VeryHigh);
        //.internal_pull_up(true)
        //.set_speed(stm32f4xx_hal::gpio::Speed::VeryHigh);   //makes no difference
        //.internal_pull_up(true)
        .set_speed(stm32f4xx_hal::gpio::Speed::VeryHigh);        //makes no difference
        
    //the hal appears to have a bug in internal_pull_up()
    /*
    //so steal the code from the hal and use it here
    let offset = 2 * 7; //pin7
    let on = true;
    let value = if on { 0b10 } else { 0b00 };   //0b01 is pullup, 0b10 is pulldown, 0b11 is reserved
    unsafe {
        &(*stm32::GPIOA::ptr()).pupdr.modify(|r, w| {
            w.bits((r.bits() & !(0b11 << offset)) | (value << offset))
        })
    };
    */
    let timer = Timer::tim1(dp.TIM1, 3.mhz(), clocks);
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut ws = Ws2812::new(timer, pin);

    /*
    //just generate a square wave as fast as possible through bit-banging
    loop {
        block!(timer.wait()).ok();
        pin.set_low().ok();
        block!(timer.wait()).ok();
        pin.set_high().ok();
    }
    */
    
    let mut data: [RGB8; 1] = [RGB8::default(); 1];
    //let empty: [RGB8; 1] = [RGB8::default(); 1];

    //lets just work with 1 LED for now!. 
    //  Orangey?
    data[0] = RGB8 {
        r: 0xFF,
        g: 0xF0,
        b: 0x00,
    };

    loop {
        ws.write(data.iter().cloned()).unwrap();
        delay.delay_ms(500 as u16);  
        //keep writing the same data. We spot errors when the color changes
    }
    

}