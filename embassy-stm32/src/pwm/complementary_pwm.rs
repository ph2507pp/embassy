use core::marker::PhantomData;

use embassy_hal_common::{into_ref, PeripheralRef};
use stm32_metapac::timer::vals;
pub use stm32_metapac::timer::vals::Ckd;

use super::simple_pwm::*;
use super::*;
#[allow(unused_imports)]
use crate::gpio::sealed::{AFType, Pin};
use crate::gpio::AnyPin;
use crate::time::Hertz;
use crate::Peripheral;

pub struct ComplementaryPwmPin<'d, Perip, Channel> {
    _pin: PeripheralRef<'d, AnyPin>,
    phantom: PhantomData<(Perip, Channel)>,
}

macro_rules! complementary_channel_impl {
    ($new_chx:ident, $channel:ident, $pin_trait:ident, $complementary_pin_trait:ident) => {
        impl<'d, Perip: CaptureCompare16bitInstance> ComplementaryPwmPin<'d, Perip, $channel> {
            pub fn $new_chx(pin: impl Peripheral<P = impl $complementary_pin_trait<Perip>> + 'd) -> Self {
                into_ref!(pin);
                critical_section::with(|_| unsafe {
                    pin.set_low();
                    pin.set_as_af(pin.af_num(), AFType::OutputPushPull);
                    #[cfg(gpio_v2)]
                    pin.set_speed(crate::gpio::Speed::VeryHigh);
                });
                ComplementaryPwmPin {
                    _pin: pin.map_into(),
                    phantom: PhantomData,
                }
            }
        }
    };
}

complementary_channel_impl!(new_ch1, Ch1, Channel1Pin, Channel1ComplementaryPin);
complementary_channel_impl!(new_ch2, Ch2, Channel2Pin, Channel2ComplementaryPin);
complementary_channel_impl!(new_ch3, Ch3, Channel3Pin, Channel3ComplementaryPin);
complementary_channel_impl!(new_ch4, Ch4, Channel4Pin, Channel4ComplementaryPin);

pub struct ComplementaryPwm<'d, T> {
    inner: PeripheralRef<'d, T>,
}

impl<'d, T: ComplementaryCaptureCompare16bitInstance> ComplementaryPwm<'d, T> {
    pub fn new(
        tim: impl Peripheral<P = T> + 'd,
        _ch1: Option<PwmPin<'d, T, Ch1>>,
        _ch1n: Option<ComplementaryPwmPin<'d, T, Ch1>>,
        _ch2: Option<PwmPin<'d, T, Ch2>>,
        _ch2n: Option<ComplementaryPwmPin<'d, T, Ch2>>,
        _ch3: Option<PwmPin<'d, T, Ch3>>,
        _ch3n: Option<ComplementaryPwmPin<'d, T, Ch3>>,
        _ch4: Option<PwmPin<'d, T, Ch4>>,
        _ch4n: Option<ComplementaryPwmPin<'d, T, Ch4>>,
        freq: Hertz,
    ) -> Self {
        Self::new_inner(tim, freq)
    }

    fn new_inner(tim: impl Peripheral<P = T> + 'd, freq: Hertz) -> Self {
        into_ref!(tim);

        T::enable();
        <T as crate::rcc::sealed::RccPeripheral>::reset();

        let mut this = Self { inner: tim };

        this.inner.set_frequency(freq);
        this.inner.start();

        unsafe {
            this.inner.enable_outputs(true);

            this.inner
                .set_output_compare_mode(Channel::Ch1, OutputCompareMode::PwmMode1);
            this.inner
                .set_output_compare_mode(Channel::Ch2, OutputCompareMode::PwmMode1);
            this.inner
                .set_output_compare_mode(Channel::Ch3, OutputCompareMode::PwmMode1);
            this.inner
                .set_output_compare_mode(Channel::Ch4, OutputCompareMode::PwmMode1);
        }
        this
    }

    pub fn enable(&mut self, channel: Channel) {
        unsafe {
            self.inner.enable_channel(channel, true);
            self.inner.enable_complementary_channel(channel, true);
        }
    }

    pub fn disable(&mut self, channel: Channel) {
        unsafe {
            self.inner.enable_complementary_channel(channel, false);
            self.inner.enable_channel(channel, false);
        }
    }

    pub fn set_freq(&mut self, freq: Hertz) {
        self.inner.set_frequency(freq);
    }

    pub fn get_max_duty(&self) -> u16 {
        unsafe { self.inner.get_max_compare_value() }
    }

    pub fn set_duty(&mut self, channel: Channel, duty: u16) {
        assert!(duty < self.get_max_duty());
        unsafe { self.inner.set_compare_value(channel, duty) }
    }

    pub fn set_dead_time_clock_division(&mut self, value: Ckd) {
        unsafe { self.inner.set_dead_time_clock_division(value) }
    }

    pub fn set_dead_time_value(&mut self, value: u8) {
        unsafe { self.inner.set_dead_time_value(value) }
    }
}

pub struct InputPin<'d, Perip, Channel> {
    _pin: PeripheralRef<'d, AnyPin>,
    phantom: PhantomData<(Perip, Channel)>,
}

impl<'d, Perip: CaptureCompare16bitInstance> InputPin<'d, Perip, Ch1> {
    pub fn new_ch1(pin: impl Peripheral<P = impl Channel1Pin<Perip>> + 'd) -> Self {
        into_ref!(pin);
        critical_section::with(|_| unsafe {
            pin.set_low();
            pin.set_as_af(pin.af_num(), AFType::Input);
            #[cfg(gpio_v2)]
            pin.set_speed(crate::gpio::Speed::VeryHigh);
        });
        InputPin {
            _pin: pin.map_into(),
            phantom: PhantomData,
        }
    }
}

pub struct ComplementaryPwmExtTriggerCH1<'d, T> {
    inner: PeripheralRef<'d, T>,
}

impl<'d, T: ComplementaryCaptureCompare16bitInstance> ComplementaryPwmExtTriggerCH1<'d, T> {
    pub fn new(
        tim: impl Peripheral<P = T> + 'd,
        _ch1: Option<InputPin<'d, T, Ch1>>,
        _ch2: Option<PwmPin<'d, T, Ch2>>,
        _ch2n: Option<ComplementaryPwmPin<'d, T, Ch2>>,
        _ch3: Option<PwmPin<'d, T, Ch3>>,
        _ch3n: Option<ComplementaryPwmPin<'d, T, Ch3>>,
        _ch4: Option<PwmPin<'d, T, Ch4>>,
        _ch4n: Option<ComplementaryPwmPin<'d, T, Ch4>>,
        aar: u16,
    ) -> Self {
        Self::new_inner(tim, aar)
    }

    fn new_inner(tim: impl Peripheral<P = T> + 'd, aar: u16) -> Self {
        into_ref!(tim);

        T::enable();
        <T as crate::rcc::sealed::RccPeripheral>::reset();

        unsafe {
            T::regs().arr().modify(|w| w.set_arr(aar));

            T::regs_advanced().cr2().modify(|w| w.set_ti1s(vals::Tis::NORMAL));

            T::regs_advanced().smcr().modify(|w| {
                w.set_ts(vals::Ts::TI1F_ED); //TI1F_ED
                w.set_sms(vals::Sms::EXT_CLOCK_MODE);
            });
        }

        let mut this = Self { inner: tim };

        //this.inner.set_frequency(freq);
        this.inner.start();

        unsafe {
            this.inner.enable_outputs(true);

            this.inner
                .set_output_compare_mode(Channel::Ch1, OutputCompareMode::PwmMode1);
            this.inner
                .set_output_compare_mode(Channel::Ch2, OutputCompareMode::PwmMode1);
            this.inner
                .set_output_compare_mode(Channel::Ch3, OutputCompareMode::PwmMode1);
            this.inner
                .set_output_compare_mode(Channel::Ch4, OutputCompareMode::PwmMode1);
        }
        this
    }

    pub fn enable(&mut self, channel: Channel) {
        unsafe {
            self.inner.enable_channel(channel, true);
            self.inner.enable_complementary_channel(channel, true);
        }
    }

    pub fn disable(&mut self, channel: Channel) {
        unsafe {
            self.inner.enable_complementary_channel(channel, false);
            self.inner.enable_channel(channel, false);
        }
    }

    pub fn read_cr1(&mut self) -> u32 {
        unsafe { T::regs_gp16().cr1().read().0 }
    }

    pub fn read_cr2(&mut self) -> u32 {
        unsafe { T::regs_gp16().cr2().read().0 }
    }

    pub fn read_smcr(&mut self) -> u32 {
        unsafe { T::regs_gp16().smcr().read().0 }
    }
    pub fn read_cnt(&mut self) -> u32 {
        unsafe { T::regs_gp16().cnt().read().0 }
    }
    pub fn reset_cnt(&mut self) {
        unsafe { T::regs_gp16().cnt().modify(|w| w.set_cnt(0)) }
    }

    pub fn set_freq(&mut self, freq: Hertz) {
        self.inner.set_frequency(freq);
    }

    pub fn get_max_duty(&self) -> u16 {
        unsafe { self.inner.get_max_compare_value() }
    }

    pub fn set_duty(&mut self, channel: Channel, duty: u16) {
        assert!(duty < self.get_max_duty());
        unsafe { self.inner.set_compare_value(channel, duty) }
    }

    pub fn set_dead_time_clock_division(&mut self, value: Ckd) {
        unsafe { self.inner.set_dead_time_clock_division(value) }
    }

    pub fn set_dead_time_value(&mut self, value: u8) {
        unsafe { self.inner.set_dead_time_value(value) }
    }
}
