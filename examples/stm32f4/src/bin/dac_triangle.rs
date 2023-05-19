#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dac::{Channel, Dac, Ch1Trigger};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World, dude!");

    let mut dac1 = Dac::new_1ch(p.DAC, p.PA4);
    dac1.set_triangle_generator(Channel::Ch1, 0b1001).unwrap();
    dac1.select_trigger_ch1(Ch1Trigger::Software).unwrap();
    dac1.enable_trigger(Channel::Ch1).unwrap();
    dac1.enable_channel(Channel::Ch1).unwrap();

    loop {
        dac1.trigger(Channel::Ch1).unwrap();
    }




}
