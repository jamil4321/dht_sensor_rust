//! Initialization code

#![no_std]
#![no_main]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler
use cortex_m::{iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use f3::hal::{gpio,time::MonoTimer,delay::Delay};
use f3::hal::{prelude::*,stm32f30x::{self,GPIOA,RCC}};
use heapless::{consts, Vec};


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

    delay.delay_ms(1000_u32);
    unsafe{
        let r1 = &*RCC::ptr();
        let gpioa = &*GPIOA::ptr();
        let mut dth_data_vec:Vec<u8, consts::U32> = Vec::new();
        r1.ahbenr.modify(|_, w| w.iopaen().set_bit());
        gpioa.moder.modify(|_,w|w.moder3().output());
        gpioa.otyper.modify(|_,w|w.ot3().set_bit());
        gpioa.pupdr.write(|w| w.pupdr3().bits(0x01));

        gpioa.bsrr.write(|w| w.bs3().clear_bit());
        delay.delay_ms(18_u32);
        gpioa.bsrr.write(|w| w.bs3().set_bit());
        delay.delay_us(20_u32);
        while gpioa.idr.read().idr3().bit_is_set(){}
        iprintln!(&mut itm.stim[0],"bit is clear");
        while gpioa.idr.read().idr3().bit_is_clear(){}
        for i in 0..41{
            delay.delay_us(35_u32);
            if gpioa.idr.read().idr3().bit_is_set(){
                dth_data_vec.push(1).is_err();
            }else{
                dth_data_vec.push(0).is_err();
            }
            gpioa.bsrr.write(|w| w.bs3().clear_bit());
        }
    iprintln!(&mut itm.stim[0],"vector Print{:?}",dth_data_vec);

    }

    loop{}
}