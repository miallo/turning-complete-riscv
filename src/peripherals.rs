use core::ops::{Deref, DerefMut};
use core::arch::asm;

use bitfield::bitfield;

#[allow(dead_code)]
pub fn ecall() {
    unsafe { asm!("ecall"); }
}

#[allow(dead_code)]
pub fn ebreak() {
    unsafe { asm!("ebreak"); }
}

// 80x24 Console
pub type Console = [u8; 80 * 24];

pub struct CON {}
unsafe impl Send for CON {}
impl CON {
    pub const fn ptr() -> *mut Console {
        0x400 as *mut _
    }
}
impl Deref for CON {
    type Target = Console;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CON::ptr() }
    }
}
impl DerefMut for CON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *CON::ptr() }
    }
}

// Keyboard Input
bitfield!{
    #[derive(Clone)]
    pub struct KeyboardInput(u16);
    u8;
    pub keyval, _: 7, 0;
    pub keyup, _: 8;
    pub has_more, _: 9;
}

pub struct KBD {}
unsafe impl Send for KBD {}
impl KBD {
    pub const fn ptr() -> *mut KeyboardInput {
        0xff000100 as *mut _
    }

    pub fn pop_input(&mut self) -> Option<KeyboardInput> {
        let input = unsafe { core::ptr::read_volatile(&mut *Self::ptr()) }.clone();

        if input.keyval() == 0 {
            None
        } else {
            Some(input)
        }
    }
}

// Unix timestamp
pub type HardwareTimestamp = u64;

pub struct TIM {}
unsafe impl Send for TIM {}
impl TIM {
    pub const fn ptr() -> *mut HardwareTimestamp {
        0xff000200 as *mut _
    }

    pub fn read(&mut self) -> HardwareTimestamp {
        unsafe { core::ptr::read_volatile(&mut *Self::ptr()) }
    }
}

// 96x64 Display
pub struct Display {
    dsp: DSP,
    pixels: [u32; 96*64],
    flush: u32,
}

pub struct DSP {}
unsafe impl Send for DSP {}
impl DSP {
    const fn ptr() -> *mut Display {
        0xff000300 as *mut _
    }

    pub fn write_pixel(&mut self, offset: usize, rgb: u32) {
        unsafe { core::ptr::write_volatile(&mut (*Self::ptr()).pixels[offset] as *mut u32, rgb) };
    }

    pub fn flush(&mut self) {
        unsafe { core::ptr::write_volatile(&mut (*Self::ptr()).flush as *mut u32, 1) };
    }
}

impl Display {
    pub fn new(dsp: DSP) -> Self {
        Self {
            dsp,
            pixels: [0; 96*64],
            flush: 0
        }
    }

    pub fn write_pixel(&mut self, offset: usize, rgb: u32) {
        self.dsp.write_pixel(offset, rgb)
    }

    pub fn clear(&mut self) {
        for i in 0..self.pixels.len(){
            self.dsp.write_pixel(i, 0)
        }
    }

    pub fn flush(&mut self) {
        self.dsp.flush();
    }
}

static mut DEVICE_PERIPHERALS: bool = false;

pub struct Peripherals {
    pub con: CON,
    pub kbd: KBD,
    pub tim: TIM,
    pub dsp: DSP,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        if unsafe { DEVICE_PERIPHERALS } { None } else { Some(unsafe { Peripherals::steal() }) }
    }

    unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;

        Peripherals {
            con: CON {},
            kbd: KBD {},
            tim: TIM {},
            dsp: DSP {},
        }
    }
}
