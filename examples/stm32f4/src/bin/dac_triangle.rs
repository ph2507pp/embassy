#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dac::{Channel, Dac, Ch1Trigger, Ch2Trigger};
use {defmt_rtt as _, panic_probe as _};
use embassy_stm32::Peripheral;
use embassy_stm32::pac::timer::TimBasic;
use embassy_stm32::time::khz;
use embassy_stm32::pac;
use embassy_stm32::timer::Basic16bitInstance;
use embassy_stm32::timer::low_level::GeneralPurpose16bitInstance;
use embassy_time::Duration;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World, dude!");

    // let mut dac1 = Dac::new_1ch(p.DAC, p.PA4);
    



    let mut dac = Dac::new_2ch(p.DAC, p.PA4, p.PA5);

    dac.set_triangle_generator(Channel::Ch1, 0b1001).unwrap();
    dac.select_trigger_ch1(Ch1Trigger::Software).unwrap();
    dac.enable_trigger(Channel::Ch1).unwrap();
    dac.enable_channel(Channel::Ch1).unwrap();



    let tim = p.TIM4;
    setup_tim(tim).await;


    dac.set_triangle_generator(Channel::Ch2, 0b1001).unwrap();
    dac.select_trigger_ch2(Ch2Trigger::Tim5).unwrap();
    dac.enable_trigger(Channel::Ch2).unwrap();
    dac.enable_channel(Channel::Ch2).unwrap();

    loop {
        dac.trigger(Channel::Ch1).unwrap();
        //dac.trigger(Channel::Ch2).unwrap();
    }




}



async fn setup_tim<T: Basic16bitInstance>(mut tim: impl Peripheral<P = T> + embassy_stm32::timer::low_level::Basic16bitInstance){
    
    // peri_ref = 
    
    // let out;
    //     unsafe{
    //         let cnt = T::regs_gp16().cnt().read();
    //         out = cnt.0;
    //     }
    //     out;
    //     info!("CNT{}", out);
    
    
    tim.set_frequency(khz(1));
    tim.set_trog_update_event();
    tim.start();


    let r = tim.readCR1();
    info!("CR1: {}", r);
    // unsafe{
    //     T::regs_gp16().cr1().modify(|reg| {
    //         reg.set_cen(true);
    //     });
    // }
    
    // unsafe{
    //     T::regs_gp16().cr2().modify(|reg| {
    //         reg.set_mms(pac::timer::vals::Mms::UPDATE);
    //     });
    // }
    // Timer::after(Duration::from_millis(300)).await;
    // let out_cr1;
    //     unsafe{
    //         let cr1 = T::regs_gp16().cr1().read();
    //         out_cr1 = cr1.0;
    //     }
    //     info!("CR1: {}", out_cr1);

    
    //     let out_cr2;
    //     unsafe{
    //         let cr2 = T::regs_gp16().cr2().read();
    //         out_cr2 = cr2.0;
    //     }
    //     info!("CR2: {}", out_cr2);
    // let out;
    //     unsafe{
    //         let cnt = T::regs_gp16().cnt().read();
    //         out = cnt.0;
    //     }

    
    // info!("CNT{}", out);

}