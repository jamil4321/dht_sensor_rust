#![no_std]
#![no_main]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler
use cortex_m::{iprint, iprintln};
use cortex_m_rt::entry;
use f3::hal::delay::Delay;
use f3::hal::{prelude::*,stm32f30x::{self}};
use dht_sensor_rust::dht11;

#[entry]
fn main()->!{
    // Initialize Peripherals 
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    // Initialize Flash & RCC
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Initialize Clock
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Initialize Delay
    let mut delay = Delay::new(cp.SYST, clocks);
    // Initialize ITM
    let mut itm = cp.ITM;

    let mut gpioa = dp.GPIOA.split(& mut rcc.ahb);
    // set pin to open drain output mode
    let mut pa3 = gpioa.pa3.into_open_drain_output(&mut gpioa.moder,&mut gpioa.otyper);
    pa3.internal_pull_up(&mut gpioa.pupdr,true);
   
    // infinite loop begin
    loop{
        // wait for 2sec make sure Dht11 is ready
        delay.delay_ms(2000_u32);
        iprintln!(&mut itm.stim[0],"waiting for responce");
        
        // Call function dht11 and it will return tuple of 4 value  
        let (hum,_, temp,_) = dht11(&mut delay, &mut pa3);

        // print the value
        iprintln!(&mut itm.stim[0],"Tempreture {}*C & Humadity{}%",temp,hum );
    }
}

