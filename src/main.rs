#![no_std]
#![no_main]

// use arrayvec::ArrayString;

mod console;
mod examples;
mod init;
mod keyboard;
mod peripherals;
mod utils;

use arrayvec::ArrayString;

#[export_name = "main"]
fn main() -> ! {
    let io = peripherals::Peripherals::take().unwrap();
    let mut console = console::Console::new(io.con);
    let mut keyboard = keyboard::Keyboard::new(io.kbd);
    let mut display = peripherals::Display::new(io.dsp);

    // examples::image::run(&mut display);
    examples::chess::run(&mut keyboard, &mut console, &mut display);
    // let mut buf = ArrayString::<80>::new();
    // loop {
    //     utils::input_line(&mut console, &mut keyboard, &mut buf);
    //     buf.clear();
    // }
    loop {}
}
