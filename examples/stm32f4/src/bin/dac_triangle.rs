#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dac::{Ch1Trigger, Ch2Trigger, Channel, Dac, TimTrog};
use embassy_stm32::time::khz;
use {defmt_rtt as _, panic_probe as _};

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

    let mut tim = TimTrog::new(p.TIM4, khz(1));
    tim.set_trog_event();

    dac.set_triangle_generator(Channel::Ch2, 0b1001).unwrap();
    dac.select_trigger_ch2(Ch2Trigger::Tim5).unwrap();
    dac.enable_trigger(Channel::Ch2).unwrap();
    dac.enable_channel(Channel::Ch2).unwrap();

    loop {
        dac.trigger(Channel::Ch1).unwrap();
        //dac.trigger(Channel::Ch2).unwrap();
    }
}
