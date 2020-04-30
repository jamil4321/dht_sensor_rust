#![no_std]


#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler
use cortex_m::{iprint, iprintln, peripheral::ITM};
use f3::hal::{time::MonoTimer,delay::Delay};
use f3::hal::{prelude::*,stm32f30x::{self,GPIOA,RCC}};



pub fn dht11 (delay:&mut Delay,pin: &mut f3::hal::gpio::gpioa::PA3<f3::hal::gpio::Output<f3::hal::gpio::OpenDrain>>,itm : &mut ITM)  
->(u8,u8,u8,u8) {
    let mut delay = delay;
    let mut pa3 = pin; 
    pa3.set_high();
    pa3.set_low();
    delay.delay_ms(18u32);
    pa3.set_high();
    while set_bit(){}

    while clear_bit(){}
    while set_bit(){}
    let mut hum_int = response(&mut delay);
    let mut hum_float = response(&mut delay);
    let mut temp_int = response(&mut delay);
    let mut temp_float = response(&mut delay);
    let mut check_sum = response(&mut delay);

    (convert_bit(&mut hum_int),convert_bit(&mut hum_float),convert_bit(&mut temp_int),convert_bit(&mut temp_float))
}


fn response(delay:&mut Delay)->[u8;8]{
    let mut data = [0u8;8];
    let delay = delay;
    for byte in data.iter_mut(){
        while clear_bit(){}
        delay.delay_us(25u32);
        if set_bit(){
            *byte = 1
        }else{
            *byte = 0;
        }
         while set_bit(){}
        //  delay.delay_us(10_u32);
    }
    data
}
fn convert_bit(data:&mut[u8;8]) -> u8{

    let arr = [128,64,32,16,8,4,2,1];
    let vec = data;
    let mut int = 0;
    for i in 0..8{
        int = int + arr[i]*vec[i]
    }
    int
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
