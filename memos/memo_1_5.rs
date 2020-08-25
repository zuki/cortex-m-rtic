Breakpoint 1, types::init (cx=...) at examples/types.rs:27
27	        debug::exit(debug::EXIT_SUCCESS);
(gdb) p cx
$1 = types::init::Context {
  start: rtic::cyccnt::Instant {
    inner: 0
  },
  core: rtic::Peripherals {
    CBP: cortex_m::peripheral::CBP {
      _marker: core::marker::PhantomData<*const ()>
    },
    CPUID: cortex_m::peripheral::CPUID {
      _marker: core::marker::PhantomData<*const ()>
    },
    DCB: cortex_m::peripheral::DCB {
      _marker: core::marker::PhantomData<*const ()>
    },
    DWT: cortex_m::peripheral::DWT {
      _marker: core::marker::PhantomData<*const ()>
    },
    FPB: cortex_m::peripheral::FPB {
      _marker: core::marker::PhantomData<*const ()>
    },
    FPU: cortex_m::peripheral::FPU {
      _marker: core::marker::PhantomData<*const ()>
    },
    ITM: cortex_m::peripheral::ITM {
      _marker: core::marker::PhantomData<*const ()>
    },
    MPU: cortex_m::peripheral::MPU {
      _marker: core::marker::PhantomData<*const ()>
    },
    NVIC: cortex_m::peripheral::NVIC {
      _marker: core::marker::PhantomData<*const ()>
    },
    SCB: cortex_m::peripheral::SCB {
      _marker: core::marker::PhantomData<*const ()>
    },
    TPIU: cortex_m::peripheral::TPIU {
      _marker: core::marker::PhantomData<*const ()>
    }
  },
  device: stm32f4::stm32f407::Peripherals {
    RNG: stm32f4::stm32f407::RNG {
      _marker: core::marker::PhantomData<*const ()>
    },
    DCMI: stm32f4::stm32f407::DCMI {
      _marker: core::marker::PhantomData<*const ()>
    },
    FSMC: stm32f4::stm32f407::FSMC {
      _marker: core::marker::PhantomData<*const ()>
    },
    DBGMCU: stm32f4::stm32f407::DBGMCU {
      _marker: core::marker::PhantomData<*const ()>
    },
    DMA2: stm32f4::stm32f407::DMA2 {
      _marker: core::marker::PhantomData<*const ()>
    },
    DMA1: stm32f4::stm32f407::DMA1 {
      _marker: core::marker::PhantomData<*const ()>
    },
    RCC: stm32f4::stm32f407::RCC {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOI: stm32f4::stm32f407::GPIOI {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOH: stm32f4::stm32f407::GPIOH {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOG: stm32f4::stm32f407::GPIOG {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOF: stm32f4::stm32f407::GPIOF {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOE: stm32f4::stm32f407::GPIOE {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOD: stm32f4::stm32f407::GPIOD {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOC: stm32f4::stm32f407::GPIOC {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOJ: stm32f4::stm32f407::GPIOJ {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOK: stm32f4::stm32f407::GPIOK {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOB: stm32f4::stm32f407::GPIOB {
      _marker: core::marker::PhantomData<*const ()>
    },
    GPIOA: stm32f4::stm32f407::GPIOA {
      _marker: core::marker::PhantomData<*const ()>
    },
    SYSCFG: stm32f4::stm32f407::SYSCFG {
      _marker: core::marker::PhantomData<*const ()>
    },
    SPI1: stm32f4::stm32f407::SPI1 {
      _marker: core::marker::PhantomData<*const ()>
    },
    SPI2: stm32f4::stm32f407::SPI2 {
      _marker: core::marker::PhantomData<*const ()>
    },
    SPI3: stm32f4::stm32f407::SPI3 {
      _marker: core::marker::PhantomData<*const ()>
    },
    I2S2EXT: stm32f4::stm32f407::I2S2EXT {
      _marker: core::marker::PhantomData<*const ()>
    },
    I2S3EXT: stm32f4::stm32f407::I2S3EXT {
      _marker: core::marker::PhantomData<*const ()>
    },
    SPI4: stm32f4::stm32f407::SPI4 {
      _marker: core::marker::PhantomData<*const ()>
    },
    SPI5: stm32f4::stm32f407::SPI5 {
      _marker: core::marker::PhantomData<*const ()>
    },
    SPI6: stm32f4::stm32f407::SPI6 {
      _marker: core::marker::PhantomData<*const ()>
    },
    SDIO: stm32f4::stm32f407::SDIO {
      _marker: core::marker::PhantomData<*const ()>
    },
    ADC1: stm32f4::stm32f407::ADC1 {
      _marker: core::marker::PhantomData<*const ()>
    },
    ADC2: stm32f4::stm32f407::ADC2 {
      _marker: core::marker::PhantomData<*const ()>
    },
    ADC3: stm32f4::stm32f407::ADC3 {
      _marker: core::marker::PhantomData<*const ()>
    },
    USART6: stm32f4::stm32f407::USART6 {
      _marker: core::marker::PhantomData<*const ()>
    },
    USART1: stm32f4::stm32f407::USART1 {
      _marker: core::marker::PhantomData<*const ()>
    },
    USART2: stm32f4::stm32f407::USART2 {
      _marker: core::marker::PhantomData<*const ()>
    },
    USART3: stm32f4::stm32f407::USART3 {
      _marker: core::marker::PhantomData<*const ()>
    },
    DAC: stm32f4::stm32f407::DAC {
      _marker: core::marker::PhantomData<*const ()>
    },
    PWR: stm32f4::stm32f407::PWR {
      _marker: core::marker::PhantomData<*const ()>
    },
    I2C3: stm32f4::stm32f407::I2C3 {
      _marker: core::marker::PhantomData<*const ()>
    },
    I2C2: stm32f4::stm32f407::I2C2 {
      _marker: core::marker::PhantomData<*const ()>
    },
    I2C1: stm32f4::stm32f407::I2C1 {
      _marker: core::marker::PhantomData<*const ()>
    },
    IWDG: stm32f4::stm32f407::IWDG {
      _marker: core::marker::PhantomData<*const ()>
    },
    WWDG: stm32f4::stm32f407::WWDG {
      _marker: core::marker::PhantomData<*const ()>
    },
    RTC: stm32f4::stm32f407::RTC {
      _marker: core::marker::PhantomData<*const ()>
    },
    UART4: stm32f4::stm32f407::UART4 {
      _marker: core::marker::PhantomData<*const ()>
    },
    UART5: stm32f4::stm32f407::UART5 {
      _marker: core::marker::PhantomData<*const ()>
    },
    UART7: stm32f4::stm32f407::UART7 {
      _marker: core::marker::PhantomData<*const ()>
    },
    UART8: stm32f4::stm32f407::UART8 {
      _marker: core::marker::PhantomData<*const ()>
    },
    ADC_COMMON: stm32f4::stm32f407::ADC_COMMON {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM1: stm32f4::stm32f407::TIM1 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM8: stm32f4::stm32f407::TIM8 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM2: stm32f4::stm32f407::TIM2 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM3: stm32f4::stm32f407::TIM3 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM4: stm32f4::stm32f407::TIM4 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM5: stm32f4::stm32f407::TIM5 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM9: stm32f4::stm32f407::TIM9 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM12: stm32f4::stm32f407::TIM12 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM10: stm32f4::stm32f407::TIM10 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM13: stm32f4::stm32f407::TIM13 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM14: stm32f4::stm32f407::TIM14 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM11: stm32f4::stm32f407::TIM11 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM6: stm32f4::stm32f407::TIM6 {
      _marker: core::marker::PhantomData<*const ()>
    },
    TIM7: stm32f4::stm32f407::TIM7 {
      _marker: core::marker::PhantomData<*const ()>
    },
    ETHERNET_MAC: stm32f4::stm32f407::ETHERNET_MAC {
      _marker: core::marker::PhantomData<*const ()>
    },
    ETHERNET_MMC: stm32f4::stm32f407::ETHERNET_MMC {
      _marker: core::marker::PhantomData<*const ()>
    },
    ETHERNET_PTP: stm32f4::stm32f407::ETHERNET_PTP {
      _marker: core::marker::PhantomData<*const ()>
    },
    ETHERNET_DMA: stm32f4::stm32f407::ETHERNET_DMA {
      _marker: core::marker::PhantomData<*const ()>
    },
    CRC: stm32f4::stm32f407::CRC {
      _marker: core::marker::PhantomData<*const ()>
    },
    OTG_FS_GLOBAL: stm32f4::stm32f407::OTG_FS_GLOBAL {
      _marker: core::marker::PhantomData<*const ()>
    },
    OTG_FS_HOST: stm32f4::stm32f407::OTG_FS_HOST {
      _marker: core::marker::PhantomData<*const ()>
    },
    OTG_FS_DEVICE: stm32f4::stm32f407::OTG_FS_DEVICE {
      _marker: core::marker::PhantomData<*const ()>
    },
    OTG_FS_PWRCLK: stm32f4::stm32f407::OTG_FS_PWRCLK {
      _marker: core::marker::PhantomData<*const ()>
    },
    CAN1: stm32f4::stm32f407::CAN1 {
      _marker: core::marker::PhantomData<*const ()>
    },
    CAN2: stm32f4::stm32f407::CAN2 {
      _marker: core::marker::PhantomData<*const ()>
    },
    FLASH: stm32f4::stm32f407::FLASH {
      _marker: core::marker::PhantomData<*const ()>
    },
    EXTI: stm32f4::stm32f407::EXTI {
      _marker: core::marker::PhantomData<*const ()>
    },
    OTG_HS_GLOBAL: stm32f4::stm32f407::OTG_HS_GLOBAL {
      _marker: core::marker::PhantomData<*const ()>
    },
    OTG_HS_HOST: stm32f4::stm32f407::OTG_HS_HOST {
      _marker: core::marker::PhantomData<*const ()>
    },
    OTG_HS_DEVICE: stm32f4::stm32f407::OTG_HS_DEVICE {
      _marker: core::marker::PhantomData<*const ()>
    },
    OTG_HS_PWRCLK: stm32f4::stm32f407::OTG_HS_PWRCLK {
      _marker: core::marker::PhantomData<*const ()>
    },
    SAI1: stm32f4::stm32f407::SAI1 {
      _marker: core::marker::PhantomData<*const ()>
    },
    LTDC: stm32f4::stm32f407::LTDC {
      _marker: core::marker::PhantomData<*const ()>
    },
    HASH: stm32f4::stm32f407::HASH {
      _marker: core::marker::PhantomData<*const ()>
    },
    CRYP: stm32f4::stm32f407::CRYP {
      _marker: core::marker::PhantomData<*const ()>
    },
    FPU: stm32f4::stm32f407::FPU {
      _marker: core::marker::PhantomData<*const ()>
    },
    STK: stm32f4::stm32f407::STK {
      _marker: core::marker::PhantomData<*const ()>
    },
    NVIC_STIR: stm32f4::stm32f407::NVIC_STIR {
      _marker: core::marker::PhantomData<*const ()>
    },
    FPU_CPACR: stm32f4::stm32f407::FPU_CPACR {
      _marker: core::marker::PhantomData<*const ()>
    },
    SCB_ACTRL: stm32f4::stm32f407::SCB_ACTRL {
      _marker: core::marker::PhantomData<*const ()>
    }
  },
  schedule: types::init::Schedule {
    _not_send: core::marker::PhantomData<*mut ()>
  },
  spawn: types::init::Spawn {
    _not_send: core::marker::PhantomData<*mut ()>
  }
}
