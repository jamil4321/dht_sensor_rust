#![no_std]
#![no_main]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler
use cortex_m::{iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use f3::hal::{time::MonoTimer,delay::Delay};
use f3::hal::{prelude::*,stm32f30x::{self,GPIOA}};


#[entry]
fn main()->!{

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);
    let mut itm = cp.ITM;
    delay.delay_ms(1000_u32);
    iprintln!(&mut itm.stim[0],"waiting for");
    let mut gpioa = dp.GPIOA.split(& mut rcc.ahb);
    let mut pa3 = gpioa.pa3.into_open_drain_output(&mut gpioa.moder,&mut gpioa.otyper);
    pa3.internal_pull_up(&mut gpioa.pupdr,true);
    // pa3.set_high();
    // delay.delay_us(40_u32);
    pa3.set_low();
    delay.delay_ms(18u32);
    pa3.set_high();
    pa3.into_pull_up_input(&mut gpioa.moder,&mut gpioa.pupdr);
    delay.delay_us(200u32);
    let mut hum_int = response(&mut delay);
    let mut hum_float = response(&mut delay);
    let mut temp_int = response(&mut delay);
    let mut temp_float = response(&mut delay);
    let mut check_sum = response(&mut delay);

    iprintln!(&mut itm.stim[0],"hum data  {:?}.{:?}%",convert_bit(&mut hum_int),convert_bit(&mut hum_float));
    iprintln!(&mut itm.stim[0],"temp data {:?}.{:?}",convert_bit(&mut temp_int),convert_bit(&mut temp_float));
    iprintln!(&mut itm.stim[0],"check sum {:?}",convert_bit(&mut check_sum));


    iprintln!(&mut itm.stim[0],"hum data  {:?}", hum_int);
    iprintln!(&mut itm.stim[0],"hum data  {:?}", hum_float);
    iprintln!(&mut itm.stim[0],"temp data {:?}",temp_int);
    iprintln!(&mut itm.stim[0],"temp data {:?}",temp_float);
    iprintln!(&mut itm.stim[0],"check sum {:?}",check_sum);
    loop{}
}

    }
}


fn dht11 (delay:&mut Delay,pin: &mut f3::hal::gpio::gpioa::PA3<f3::hal::gpio::Output<f3::hal::gpio::OpenDrain>>,itm : &mut ITM)  
{
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
    iprintln!(&mut itm.stim[0],"Tempreture {}*C and Humadity {}%",convert_bit(&mut temp_int),convert_bit(&mut hum_int));

    // (convert_bit(&mut hum_int),convert_bit(&mut hum_float),convert_bit(&mut temp_int),convert_bit(&mut temp_float))
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
