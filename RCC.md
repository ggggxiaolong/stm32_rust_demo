## RCC

Rest && Clock Control 时钟控制

[**以下内容来自**](https://www.cnblogs.com/zc110747/p/4692379.html)

 **(1)时钟源（4个晶振源，1个中介源）**

| 时钟源   | 说明                                                         |
| -------- | ------------------------------------------------------------ |
| HSI(RC)  | 内部高速晶振                                                 |
| HSE(Osc) | 外部高速晶振                                                 |
| LSE(Osc) | 外部低速晶振                                                 |
| LSI(RC)  | 内部低速晶振                                                 |
| PLLCLK   | 锁向环倍输出,最大频率小于72MHz,注：PLLCLK来源HSE，HSE/2，HSI/2 |

**(2)系统时钟源**

| 时钟源 | 说明                                                         |
| ------ | ------------------------------------------------------------ |
| SYSCLK | 系统时钟;来源HSI，PLLCLK，HSE，若CSS（时钟监视系统）检测到HSE失效，SYSCLK = HSI； |

![img](https://res.cloudinary.com/xiaolong/image/upload/v1612709847/rust_stm32/041024497369672_epir5e.png)

**(3)主要输出时钟源**

| 输出时钟源 | 说明                                                         |
| ---------- | ------------------------------------------------------------ |
| HCLK       | 高性能总线时钟(SYSCLK通过AHB Prescaler，最高72MHZ)           |
| PCCLK1     | 外设1区域时钟(通过APB1 Prescaler，最高36MHZ)                 |
| PCCLK2     | 外设2区域时钟(通过APB2 Prescaler，最高72MHZ)                 |
|            | APB1，APB2外设时钟除了给对应外设区域提供时钟外，还可通过TIMERX Prescaler分配不同的定时器时钟 |
| ADCCLK     | ADC外设时钟（PCLK2通过ADC Prescaler）                        |
| USBCLK     | 通用串行接口时钟(PLLCLK通过USB Prescaler，等于48MHZ)         |
| RTCCLK     | 实时时钟，来源LSI，LSE，HSE/128                              |
| IWDGCLK    | 独立看门狗时钟，来源LSI                                      |
| MCO        | 输出内部时钟                                                 |

 从上面看，我们前面学到的GPIOD外设还有后面的USART等的时钟都没有提到，为什么，其实它们包含在PCLK1，PCLK2这两个外设区域时钟里，也就是说他们的外设时钟来源于该区域的时钟。下面是STM32Fxxx固件函数库中15.2.22以及15.2.23所提到的图，包含所有外设对应的区域：

PCLK1时钟区域:

![PLCLK1](https://res.cloudinary.com/xiaolong/image/upload/v1612709912/rust_stm32/041114211422793_znbnzr.jpg)

 PCLK2时钟区域:

![PCLK2](https://res.cloudinary.com/xiaolong/image/upload/v1612709954/rust_stm32/041114465014445_hjonok.jpg)