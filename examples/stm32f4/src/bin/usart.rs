#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::dma::NoDma;
use embassy_stm32::interrupt;
use embassy_stm32::usart::{Config, Uart};
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());

    let config = Config::default();
    let irq = interrupt::take!(USART3);
    let mut usart = Uart::new(p.USART3, p.PB11, p.PB10, irq, NoDma, NoDma, config);

    unwrap!(usart.blocking_write(b"Hello Embassy World!\r\n"));
    info!("wrote Hello, starting echo");

    let mut buf = [0u8; 1];
    loop {
        unwrap!(usart.blocking_read(&mut buf));
        unwrap!(usart.blocking_write(&buf));
    }
}
