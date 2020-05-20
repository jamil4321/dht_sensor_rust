#![no_std]


#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler
use f3::hal::delay::Delay;
use f3::hal::{prelude::*,stm32f30x::GPIOA};


pub fn dht11 (delay:&mut Delay,pin: &mut f3::hal::gpio::gpioa::PA3<f3::hal::gpio::Output<f3::hal::gpio::OpenDrain>>)  
-> (u8,u8,u8,u8){
    // Initlialize Delay
    let mut delay = delay;
    // Initliaze Pin PA3
    let mut pa3 = pin; 
    // Set Pin High
    pa3.set_high();
    // Delay for 100 microSecond
    delay.delay_us(100u32);
    // Set Pin Low 
    pa3.set_low();
    // Delay for 18 microSecond
    delay.delay_ms(18u32);
    // Set Pin High
    pa3.set_high();
    // Check Set bit
    while set_bit(){}
     // Check Clear bit
    while clear_bit(){}
    // Check Set bit
    while set_bit(){}
    // Get Data for humidaty Intiger Data
    let mut hum_int = response(&mut delay);
    // Get Data for humidaty Float Data
    let mut hum_float = response(&mut delay);
    // Get Data for Temp intiger Data
    let mut temp_int = response(&mut delay);
    // Get Data for Temp Float Data
    let mut temp_float = response(&mut delay);
    // Get Data for Check Sum 
    let mut check_sum = response(&mut delay);

    // Return The Data
    (convert_bit(&mut hum_int),convert_bit(&mut hum_float),convert_bit(&mut temp_int),convert_bit(&mut temp_float))
}

// check bit set or not
fn set_bit()->bool{
    unsafe{
        let gpioa_pin = &*GPIOA::ptr();
        gpioa_pin.idr.read().idr3().bit_is_set()
    }
}
// check bit is clear or not
fn clear_bit()->bool{
    unsafe{
        let gpioa_pin = &*GPIOA::ptr();
        gpioa_pin.idr.read().idr3().bit_is_clear()
    }
}
// Geting Response and convert it to Array
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
    }
    data
}

// Convert bit to Byte 
fn convert_bit(data:&mut[u8;8]) -> u8{

    let arr = [128,64,32,16,8,4,2,1];
    let vec = data;
    let mut int = 0;
    for i in 0..8{
        int = int + arr[i]*vec[i]
    }
    int
}