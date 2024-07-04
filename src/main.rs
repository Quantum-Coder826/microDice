#![no_main]
#![no_std]

use core::panic::PanicInfo;
mod consts;
use consts::NUMS;

use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::{
    board::Board,
    display::blocking::Display, 
    hal::Timer};

#[panic_handler]
fn panic(_i: &PanicInfo) -> !{
    loop {}
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    let mut index_num: i8 = 0;

    loop {
        mma8x5x
    }
}