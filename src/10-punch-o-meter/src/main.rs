#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
//use nrf52832_hal::Timer;
use panic_rtt_target as _;

mod bsp;
use crate::bsp::Board;
use nb::block;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    //let mut timer = Timer::new(board.TIMER0);
    let mut led_is_on = false;
    loop {
        if led_is_on {
            board.leds.led_2.disable();
        } else {
            board.leds.led_2.enable();
        }
        //block!(timer.wait()).unwrap();
        led_is_on = !led_is_on;
    }
}
