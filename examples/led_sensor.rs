#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::adc;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let mut adc = adc::Adc::new(dp.ADC, Default::default());
    let mut sensor = pins.a0.into_analog_input(&mut adc);
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );


    loop {
        let value: u16 = nb::block!(adc.read(&mut sensor)).void_unwrap();
        ufmt::uwriteln!(&mut serial, "sensor reading: {}\r", value).void_unwrap();
        arduino_uno::delay_ms(1000);

    }


}
