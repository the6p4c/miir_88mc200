#![no_main]
#![no_std]
use cortex_m_rt::entry;
use marvell_88mc200 as _;

extern crate panic_halt;

macro_rules! ptr {
    ($name:ident = $value:expr) => {
        const $name: *mut u32 = $value as *mut u32;
    }
}

ptr!(GPSR0 = 0x46060018);
ptr!(GPCR0 = 0x46060024);
ptr!(GSDR0 = 0x46060054);

#[entry]
fn main() -> ! {
    unsafe {
        GSDR0.write_volatile(1 << 28);
        GSDR0.write_volatile(1 << 29);

        loop {
            GPSR0.write_volatile(1 << 29);
            GPCR0.write_volatile(1 << 28);
            for _ in 0..1000000 {}
            GPCR0.write_volatile(1 << 29);
            GPSR0.write_volatile(1 << 28);
            for _ in 0..1000000 {}
        }
    }
}
