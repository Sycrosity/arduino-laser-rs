#![no_std]
#![no_main]

use panic_halt as _;

use ag_lcd::{Blink, Cursor, Display, LcdDisplay};
use arduino_hal::prelude::*;
// use robust_arduino_serial::*;

mod servo;

use crate::servo::ServoUnit;

use arduino_hal::simple_pwm;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);
    let delay = arduino_hal::Delay::new();

    let rs = pins.d12.into_output().downgrade();
    let en = pins.d11.into_output().downgrade();
    let d4 = pins.d5.into_output().downgrade();
    let d5 = pins.d4.into_output().downgrade();
    let d6 = pins.d3.into_output().downgrade();
    let d7 = pins.d2.into_output().downgrade();

    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new(rs, en, delay)
        .with_half_bus(d4, d5, d6, d7)
        .with_display(Display::On)
        .with_blink(Blink::On)
        .with_cursor(Cursor::On)
        .build();

    lcd.set_cursor(Cursor::Off);
    lcd.set_blink(Blink::Off);

    let text = "Hello World!";
    lcd.print(text);

    ufmt::uwriteln!(&mut serial, "Current text is: {}\r", text).void_unwrap();


    let servo = pins.d8.into_output().downgrade();
    


    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).void_unwrap();

        match b {
            10u8 => (),
            47u8 => lcd.clear(),
            _ => lcd.write(b),
        }

        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
    }
}
