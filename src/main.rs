#![no_std]
#![no_main]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler
use cortex_m::{iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use f3::hal::{gpio,time::MonoTimer,delay::Delay};
use f3::hal::{prelude::*,stm32f30x::{self,GPIOA,RCC,TIM6,rcc,tim16}};
use heapless::Vec;
use heapless::consts::*;


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
    iprintln!(&mut itm.stim[0],"waiting for");
    let mut gpioa = dp.GPIOA.split(& mut rcc.ahb);
    let mut pa3 = gpioa.pa3.into_open_drain_output(&mut gpioa.moder,&mut gpioa.otyper);
    pa3.internal_pull_up(&mut gpioa.pupdr,true);
    pa3.set_high();
    delay.delay_us(40_u32);
    pa3.set_low();
    delay.delay_ms(20_u32);
    pa3.set_high();
    pa3.into_pull_up_input(&mut gpioa.moder,&mut gpioa.pupdr);

    let mut hum_int = response(&mut delay);
    let hum_float = response(&mut delay);

    iprintln!(&mut itm.stim[0],"hum vec {:?}",convert_bit(&mut hum_int));
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
fn response(delay:&mut Delay)->Vec<u8,heapless::consts::U8>{
    let mut data = Vec::new();
    let delay = delay;
    for i in 0..8{
        while clear_bit(){}
        delay.delay_us(35_u32);
        if set_bit(){
            data.push(1).is_err();
        }else{
            data.push(0).is_err();
        }
        while set_bit(){}
    }
    data
}

fn convert_bit(data:&mut Vec<u8,heapless::consts::U8>) -> u8{

    let mut arr = [128,64,32,16,8,4,2,1];
    let mut vec = data;
    let mut int = 0;
    for i in 0..8{
        int = int + arr[i]*vec[i]
    }
    int
}