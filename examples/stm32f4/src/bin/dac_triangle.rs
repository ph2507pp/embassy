#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dac::{Channel, Dac};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World, dude!");

    let mut dac = Dac::new_1ch(p.DAC, p.PA4);
    dac.set_triangle_generator(Channel::Ch1, 0b1001).unwrap();
    dac.select_trigger_ch1(embassy_stm32::dac::Ch1Trigger::Software).unwrap();
    dac.enable_channel(Channel::Ch1).unwrap();

    loop {
        dac.trigger(Channel::Ch1).unwrap();
    }
}
