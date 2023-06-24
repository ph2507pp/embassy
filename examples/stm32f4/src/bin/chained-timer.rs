#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::pwm::complementary_pwm::{Ckd, ComplementaryPwmExtTriggerCH1, ComplementaryPwmPin, InputPin};
use embassy_stm32::pwm::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::pwm::{CaptureCompare16bitInstance, Channel};
use embassy_stm32::time::{hz, khz};
use embassy_time::{Delay, Duration, Ticker, Timer};
use {defmt_rtt as _, panic_probe as _};

fn limit(mut val: i16, max: u16) -> u16 {
    if val < 0 {
        val = 0;
    } else if val > max as i16 {
        val = max as i16;
    }
    val as u16
}

/// [angle]: +- 45°
fn set_phase_shift<T>(tim: &mut SimplePwm<T>, angle: f32)
where
    T: CaptureCompare16bitInstance,
{
    let max = tim.get_max_duty() - 1;
    let mid = (max) / 2;
    let min_pulse = mid / 2;

    let d: i16 = ((mid - min_pulse + 1) as f32 * angle / 45.0) as i16;

    let d_ch1 = mid as i16 - d + 1;
    let d_ch1 = limit(d_ch1, max);

    let d_ch2 = mid as i16 + d + 1;
    let d_ch2 = limit(d_ch2, max);

    tim.set_duty(Channel::Ch1, d_ch1);
    tim.set_duty(Channel::Ch2, d_ch2);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut _led = Output::new(p.PB7, Level::High, Speed::Low);

    // TIM1

    let ch1 = InputPin::new_ch1(p.PA8);
    let ch2 = PwmPin::new_ch2(p.PA9);
    let ch3 = PwmPin::new_ch3(p.PA10);
    let ch2n = ComplementaryPwmPin::new_ch2(p.PB0);
    let ch3n = ComplementaryPwmPin::new_ch3(p.PB1);
    let mut tim1_pwm = ComplementaryPwmExtTriggerCH1::new(
        p.TIM1,
        Some(ch1),
        Some(ch2),
        Some(ch2n),
        Some(ch3),
        Some(ch3n),
        None,
        None,
        3,
    );

    // TIM8
    let ch1 = InputPin::new_ch1(p.PC6);
    let ch2 = PwmPin::new_ch2(p.PC7);
    let ch3 = PwmPin::new_ch3(p.PC8);
    let ch2n = ComplementaryPwmPin::new_ch2(p.PB14);
    let ch3n = ComplementaryPwmPin::new_ch3(p.PB15);
    let mut tim8_pwm = ComplementaryPwmExtTriggerCH1::new(
        p.TIM8,
        Some(ch1),
        Some(ch2),
        Some(ch2n),
        Some(ch3),
        Some(ch3n),
        None,
        None,
        3,
    );
    // let max = pwm.get_max_duty();
    tim1_pwm.set_dead_time_clock_division(Ckd::DIV1);
    tim1_pwm.set_dead_time_value(0x08);
    tim1_pwm.set_duty(Channel::Ch2, 2);
    tim1_pwm.set_invert_channel(Channel::Ch2, true);
    tim1_pwm.set_duty(Channel::Ch3, 2);
    tim1_pwm.set_invert_channel(Channel::Ch3, true);
    tim8_pwm.set_dead_time_clock_division(Ckd::DIV1);
    tim8_pwm.set_dead_time_value(0x08);  //0 to 15875ns by 125ns step  0x04 -> 500ns, wenn 8MHZ hier 16MHz also 0x08 -> 500ns
    tim8_pwm.set_duty(Channel::Ch2, 2);
    tim8_pwm.set_invert_channel(Channel::Ch2, true);
    tim8_pwm.set_duty(Channel::Ch3, 2);
    tim8_pwm.set_invert_channel(Channel::Ch3, true);

    tim1_pwm.enable(Channel::Ch2);
    tim1_pwm.enable(Channel::Ch3);
    tim8_pwm.enable(Channel::Ch2);
    tim8_pwm.enable(Channel::Ch3);

    // TIM3

    let ch1 = PwmPin::new_ch1(p.PA6);
    let ch2 = PwmPin::new_ch2(p.PA7);
    let mut tim3_pwm: SimplePwm<_> = SimplePwm::new(p.TIM3, Some(ch1), Some(ch2), None, None, khz(60));

    set_phase_shift(&mut tim3_pwm, 0.0);
    tim3_pwm.stop();
    tim1_pwm.stop();
    tim8_pwm.stop();

    //Connecting
    // PA6 --> CH1_TIM1  PA8
    // PA7 --> CH1_TIM8  PC6

    tim1_pwm.set_cnt(3);
    tim8_pwm.set_cnt(3);
    tim3_pwm.reset_cnt();

    info!("TIM1 CNT: {}", tim1_pwm.read_cnt());

    info!("TIM3 CNT: {}", tim3_pwm.read_cnt());
    info!("TIM8 CNT: {}", tim8_pwm.read_cnt());

    tim3_pwm.enable(Channel::Ch1);
    tim3_pwm.enable(Channel::Ch2);

    info!("TIM1 CNT: {}", tim1_pwm.read_cnt());

    info!("TIM3 CNT: {}", tim3_pwm.read_cnt());
    info!("TIM8 CNT: {}", tim8_pwm.read_cnt());

    Timer::after(Duration::from_millis(300)).await;

    tim1_pwm.start();
    tim8_pwm.start();
    tim3_pwm.start();

    //Poti
    let mut delay = Delay;
    let mut adc = Adc::new(p.ADC1, &mut delay);
    let mut pin = p.PC1;

    info!("AAR TIM3 {}", tim3_pwm.get_max_duty());
    info!("TIM1 CR1: {}", tim1_pwm.read_cr1());
    info!("TIM1 CR2: {}", tim1_pwm.read_cr2());
    info!("TIM1 SMCR: {}", tim1_pwm.read_smcr());
    let mut angle = 0.0;
    // let mut data = Data { data: &0.0 };
    // unwrap!(spawner.spawn(ausgabe(&data)));

    loop {
        let a0 = adc.read(&mut pin);
        let mut angle = (a0 as i32 - 2048) as f32 * 45.0 / 2047.0;
        if (angle > 44.9) {
            angle = 45.0;
        } else if (angle < -44.9) {
            angle = -45.0;
        }

        // data.set_data(&angle);

        //set_phase_shift(&mut tim3_pwm, angle);
        //
        // info!("TIM1 CNT: {}", tim1_pwm.read_cnt());

        Timer::after(Duration::from_millis(300)).await;
    }
}

// struct Data<'l, T> {
//     data: &'l T,
// }
// impl<'l, T> Data<'l, T> {
//     pub fn set_data(&self, val: &T) {
//         self.data = val;
//     }
// }

// #[embassy_executor::task(pool_size = 4)]
// async fn ausgabe(angle: &Data<'static, f32>) {
//     let mut ticker = Ticker::every(Duration::from_secs(1));
//     loop {
//         info!(
//             "Angle: {} TIM3: CCR1 , CCR2 ",
//             angle.data,
//             // tim3_pwm.read_ccr(Channel::Ch1),
//             // tim3_pwm.read_ccr(Channel::Ch2)
//         );
//         ticker.next().await;
//     }
// }
