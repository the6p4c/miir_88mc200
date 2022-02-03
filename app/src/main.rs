#![no_main]
#![no_std]
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;
use cortex_m_rt::entry;
use marvell_88mc200;

extern crate panic_halt;

fn bad_delay(syst: &mut SYST, t_ms: usize) {
    syst.clear_current();

    for _ in 0..t_ms {
        while !syst.has_wrapped() {}
    }
}

#[entry]
fn main() -> ! {
    const GPIO_28: u32 = 1 << 28;
    const GPIO_29: u32 = 1 << 29;

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = marvell_88mc200::Peripherals::take().unwrap();

    let mut syst = cp.SYST;
    let gpio = dp.GPIO;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(200_000);
    syst.enable_counter();

    unsafe {
        gpio.gsdr0.write(|w| w.bits(GPIO_28 | GPIO_29));

        loop {
            gpio.gpsr0.write(|w| w.bits(GPIO_28));
            gpio.gpcr0.write(|w| w.bits(GPIO_29));
            bad_delay(&mut syst, 1000);

            gpio.gpcr0.write(|w| w.bits(GPIO_28));
            gpio.gpsr0.write(|w| w.bits(GPIO_29));
            bad_delay(&mut syst, 1000);
        }
    }
}
