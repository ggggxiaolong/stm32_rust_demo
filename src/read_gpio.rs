use embedded_hal::digital::v2::{OutputPin, InputPin};
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

pub fn read_gpio() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let pin = gpioa.pa0.into_floating_input(&mut gpioa.crl);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
        if pin.is_high().unwrap() {
            led.set_low().unwrap();
        } else {
            led.set_high().unwrap();
        }
    }
}