use embedded_hal::digital::v2::{OutputPin, InputPin};
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

pub fn read_gpio() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // 关于RCC https://www.cnblogs.com/zc110747/p/4692379.html
    let mut rcc = dp.RCC.constrain();

    // 控制GPIO 需要使用 PCLK2：外设2区域时钟(通过APB2 Prescaler，最高72MHZ)
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // crl 用于控制GPIO低8位(0~7)
    // crh 用于控制GPIO高8位(8~15)
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