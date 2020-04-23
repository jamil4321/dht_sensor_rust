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

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut dth_data_vec:Vec<u8, consts::U32> = Vec::new();
    let mut count = 0;
    iprintln!(&mut itm.stim[0],"wait for dth ready");
    delay.delay_ms(1000_u32);
    let mut pa3 = gpioa.pa3.into_open_drain_output(&mut gpioa.moder,&mut gpioa.otyper);
    pa3.internal_pull_up(&mut gpioa.pupdr,true);
    pa3.set_low();
    delay.delay_ms(18_u32);
    pa3.set_high();
    while set_bit(){}
    iprintln!(&mut itm.stim[0],"bit is clear");
    while clear_bit(){}
    while set_bit(){}
    for i in 0..41{
        delay.delay_us(20_u32);
        if set_bit(){
            dth_data_vec.push(1).is_err();
        }else{
            dth_data_vec.push(0).is_err();
        }
    }
    iprintln!(&mut itm.stim[0],"vector Print{:?}",dth_data_vec);

    loop{}
}
fn set_bit()->bool{
    unsafe{
        let gpioa_pin = &*GPIOA::ptr();
        gpioa_pin.idr.read().idr3().bit_is_set()
    }
}
fn clear_bit()->bool{
    unsafe{
        let gpioa_pin = &*GPIOA::ptr();
        gpioa_pin.idr.read().idr3().bit_is_clear()
    }
}
