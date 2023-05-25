#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::pwm::complementary_pwm::{Ckd, ComplementaryPwm, ComplementaryPwmPin};
use embassy_stm32::pwm::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::pwm::Channel;
use embassy_stm32::time::khz;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led = Output::new(p.PB7, Level::High, Speed::Low);

    // TIM1
    let ch2 = PwmPin::new_ch2(p.PA9);
    let ch3 = PwmPin::new_ch3(p.PA10);
    let ch2n = ComplementaryPwmPin::new_ch2(p.PB0);
    let ch3n = ComplementaryPwmPin::new_ch3(p.PB1);
    let mut tim1_pwm = ComplementaryPwm::new(
        p.TIM1,
        None,
        None,
        Some(ch2),
        Some(ch2n),
        Some(ch3),
        Some(ch3n),
        None,
        None,
        khz(20),
    );

    // TIM8
    let ch2 = PwmPin::new_ch2(p.PC7);
    let ch3 = PwmPin::new_ch3(p.PC8);
    let ch2n = ComplementaryPwmPin::new_ch2(p.PB14);
    let ch3n = ComplementaryPwmPin::new_ch3(p.PB15);
    let mut tim8_pwm = ComplementaryPwm::new(
        p.TIM8,
        None,
        None,
        Some(ch2),
        Some(ch2n),
        Some(ch3),
        Some(ch3n),
        None,
        None,
        khz(20),
    );
    // let max = pwm.get_max_duty();
    tim1_pwm.set_dead_time_clock_division(Ckd::DIV1);
    tim1_pwm.set_dead_time_value(0);
    tim8_pwm.set_dead_time_clock_division(Ckd::DIV1);
    tim8_pwm.set_dead_time_value(0);

    tim1_pwm.enable(Channel::Ch2);
    tim1_pwm.enable(Channel::Ch3);
    tim8_pwm.enable(Channel::Ch2);
    tim8_pwm.enable(Channel::Ch3);

    loop {
        info!("high");
        led.set_high();
        Timer::after(Duration::from_millis(300)).await;

        info!("low");
        led.set_low();
        Timer::after(Duration::from_millis(300)).await;
    }
}
