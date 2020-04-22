//! Initialization code

#![no_std]
#![no_main]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler
use cortex_m::{iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use f3::hal::{time::MonoTimer,delay::Delay};
use f3::hal::{prelude::*,stm32f30x::{self,GPIOA,RCC}};

#[entry]
fn main()->!{

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
   

    let mut delay = Delay::new(cp.SYST, clocks);
    let mono_time = MonoTimer::new(cp.DWT, clocks);

    let mut itm = cp.ITM;

    unsafe{
        let r = &*RCC::ptr();
        r.ahbenr.modify(|_, w| w.iopaen().set_bit());

        let gpioa = &*GPIOA::ptr();
        delay.delay_ms(1000_u16);
        gpioa.moder.modify(|_,w|w.moder3().output());
        gpioa.otyper.write(|w| w.ot3().bit(true));
        delay.delay_us(20_u32);
        gpioa.otyper.write(|w| w.ot3().bit(false));
        delay.delay_ms(18_u32);
    
        // gpioa.otyper.write(|w| w.ot1().set_bit());
        iprintln!(&mut itm.stim[0],"Programm in set bit");
        while gpioa.otyper.read().ot3().bit_is_set(){};
        iprintln!(&mut itm.stim[0],"Programm in clear bit");
        let instance = mono_time.now();
        while gpioa.otyper.read().ot3().bit_is_clear(){};
        let elapsed = instance.elapsed();

        iprintln!(&mut itm.stim[0],"{}",elapsed as f32 / mono_time.frequency().0 as f32 * 1e6);

    }
    

    loop{}
}
