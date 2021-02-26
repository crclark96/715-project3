#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;
use arduino_uno::prelude::*;
use core::cell;

static COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

#[avr_device::interrupt(atmega328p)]
fn INT0() {
    // external interrupt 0 (falling edge triggered)
    avr_device::interrupt::free(|cs| {
        let counter = COUNTER.borrow(cs).get();
        COUNTER.borrow(cs).set(counter + 1);
    });
}

fn int0_init(exint: arduino_uno::pac::EXINT) {
    exint.eimsk.write(|w| w.int0().set_bit()); // enable int0
    exint.eicra.write(|w| w.isc0().val_0x02()); // falling edge interrupts
}

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let mut ll_counter = 0;

    int0_init(dp.EXINT);

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    unsafe { avr_device::interrupt::enable() };

    loop {
        avr_device::interrupt::free(|cs| {
            let gl_counter = COUNTER.borrow(cs).get();
            if gl_counter > ll_counter {
                ufmt::uwriteln!(&mut serial, "interrupts: {}\r", gl_counter).void_unwrap();
                ll_counter = gl_counter;
            }
        });
        arduino_uno::delay_ms(10);

    }


}
