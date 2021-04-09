#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let n = leds.len();
    loop {
        for i in 0..n {
            let j = (i + 1) % n;

            leds[j].on().ok();
            delay.delay_ms(50u16);

            leds[i].off().ok();
            delay.delay_ms(50u16);
        }
    }
}
