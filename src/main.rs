#![no_main]
#![no_std]

use panic_halt as _;

mod assets;
use assets::numbers::NUMS;

use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin};
use microbit::Board;
use microbit::hal::Timer;
use microbit::display::blocking::Display;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    let mut index_num: i8 = 0;

loop {
        
        index_num += 1;
        // loop indexing when overflow
        if index_num > 25 { index_num = 0; }
        if index_num < 0 { index_num = 25; }

        display.show(&mut timer, NUMS[index_num as usize], 150);
        display.clear()
    }
}