#![no_std]
#![no_main]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler
use cortex_m::{iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use f3::hal::delay::Delay;
use f3::hal::{prelude::*,stm32f30x::{self,GPIOA}};
mod lib;
use lib::dht11;

#[entry]
fn main()->!{

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);
    let mut itm = cp.ITM;
    let mut gpioa = dp.GPIOA.split(& mut rcc.ahb);
    let mut pa3 = gpioa.pa3.into_open_drain_output(&mut gpioa.moder,&mut gpioa.otyper);
    pa3.internal_pull_up(&mut gpioa.pupdr,true);
   
    loop{
        
        delay.delay_ms(2000_u32);
        iprintln!(&mut itm.stim[0],"wait for responce");
        let (hum,_, temp,_) = dht11(&mut delay, &mut pa3);
        iprintln!(&mut itm.stim[0],"Tempreture {}*C & Humadity{}%",temp,hum );
    }
}

