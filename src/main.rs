#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate lpc43xx;

use cortex_m::asm;
use cortex_m_rt::entry;

// {2, 10, (SCU_MODE_FUNC0)}, // GPIO0[14]
// {2, 11, (SCU_MODE_FUNC0)}, // GPIO1[11]
// {2, 12, (SCU_MODE_FUNC0)}, // GPIO1[12]
// {2, 13, (SCU_MODE_FUNC0)}, // GPIO1[13]

#[entry]
fn main() -> ! {

    let peripherals = lpc43xx::Peripherals::take().unwrap();
    let gpio = &peripherals.GPIO_PORT;

    gpio.dir[0].write(|w| w.dirp14().set_bit());
    gpio.set[0].write(|w| w.setp14().set_bit());

    gpio.dir[1].write(|w| w.dirp11().set_bit());
    gpio.set[1].write(|w| w.setp11().set_bit());

    loop {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
        for _x in 0..20_000 {
            asm::nop();
        }

        gpio.not[1].write(|w| w.notp11().set_bit());
    }
}