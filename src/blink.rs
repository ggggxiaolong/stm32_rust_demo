use nb::block;

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

pub fn blink() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut timber = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

    let mut leds = [
        gpioc.pc13.into_push_pull_output(&mut gpioc.crh).downgrade(),
        gpioc.pc14.into_push_pull_output(&mut gpioc.crh).downgrade(),
        gpioc.pc15.into_push_pull_output(&mut gpioc.crh).downgrade(),
    ];

    loop {
        block!(timber.wait()).unwrap();
        for led in leds.iter_mut() {
            block!(timber.wait()).unwrap();
            led.set_high().unwrap();
        }
        // block!(timber.wait()).unwrap();
        for led in leds.iter_mut() {
            block!(timber.wait()).unwrap();
            led.set_low().unwrap();
        }
    }
}