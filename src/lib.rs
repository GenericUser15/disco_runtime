#![no_std]

use core::{panic::PanicInfo, ptr};

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn NMI();
    fn HardFaultTrampoline();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
    fn WWDG();
    fn PVD();
    fn TAMPER_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2_TS();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CHANNEL1();
    fn DMA1_CHANNEL2();
    fn DMA1_CHANNEL3();
    fn DMA1_CHANNEL4();
    fn DMA1_CHANNEL5();
    fn DMA1_CHANNEL6();
    fn DMA1_CHANNEL7();
    fn ADC1_2();
    fn USB_HP_CAN_TX();
    fn USB_LP_CAN_RX0();
    fn CAN_RX1();
    fn CAN_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM15();
    fn TIM1_UP_TIM16();
    fn TIM1_RTG_COM_TIM17();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_Alarm();
    fn USBWakeUp();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRIG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn FMC();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DAC();
    fn TIM7();
    fn DMA2_CHANNEL1();
    fn DMA2_CHANNEL2();
    fn DMA2_CHANNEL3();
    fn DMA2_CHANNEL4();
    fn DMA2_CHANNEL5();
    fn ADC4();
    fn COMP1_2_3();
    fn COMP4_5_6();
    fn COMP7();
    fn I2C3_EV();
    fn I2C3_ER();
    fn USB_HP();
    fn USB_LP();
    fn USB_WakeUp_RMP();
    fn TIM20_BRK();
    fn TIM20_UP();
    fn TIM20_TRG_COM();
    fn TIM20_CC();
    fn FPU();
    fn SPI4();
}


// Init ram to  0.
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut __sbss: u32;
        static mut __ebss: u32;

        static mut __sdata: u32;
        static mut __edata: u32;

        static __sidata: u32;
    }

    // Initialise BSS and DATA sections as 0.
    let count = &__ebss as *const u32 as usize - &__sbss as *const u32 as usize;
    ptr::write_bytes(&mut __sbss as *mut u32, 0, count);

    let count = &__edata as *const u32 as usize - &__sdata as *const u32 as usize;
    ptr::copy_nonoverlapping(&__sidata as *const u32, &mut __sdata as *mut u32, count);


    extern "Rust" {
        fn main() -> !;
    }

    main();
}


#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {};
}

#[no_mangle]
pub extern  "C" fn DefaultInterruptHandler() {
    loop {};
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { reserved: 0 },
    Vector { handler: NMI },
    Vector { handler: HardFaultTrampoline },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector { handler: UsageFault },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 85] = [
    Vector { handler: WWDG },
    Vector { handler: PVD },
    Vector { handler: TAMPER_STAMP },
    Vector { handler: RTC_WKUP },
    Vector { handler: FLASH },
    Vector { handler: RCC },
    Vector { handler: EXTI0 },
    Vector { handler: EXTI1 },
    Vector { handler: EXTI2_TS },
    Vector { handler: EXTI3 },
    Vector { handler: EXTI4 },
    Vector { handler: DMA1_CHANNEL1 },
    Vector { handler: DMA1_CHANNEL2 },
    Vector { handler: DMA1_CHANNEL3 },
    Vector { handler: DMA1_CHANNEL4 },
    Vector { handler: DMA1_CHANNEL5 },
    Vector { handler: DMA1_CHANNEL6 },
    Vector { handler: DMA1_CHANNEL7 },
    Vector { handler: ADC1_2 },
    Vector { handler: USB_HP_CAN_TX },
    Vector { handler: USB_LP_CAN_RX0 },
    Vector { handler: CAN_RX1 },
    Vector { handler: CAN_SCE },
    Vector { handler: EXTI9_5 },
    Vector { handler: TIM1_BRK_TIM15 },
    Vector { handler: TIM1_UP_TIM16 },
    Vector { handler: TIM1_RTG_COM_TIM17 },
    Vector { handler: TIM1_CC },
    Vector { handler: TIM2 },
    Vector { handler: TIM3 },
    Vector { handler: TIM4 },
    Vector { handler: I2C1_EV },
    Vector { handler: I2C1_ER },
    Vector { handler: I2C2_EV },
    Vector { handler: I2C2_ER },
    Vector { handler: SPI1 },
    Vector { handler: SPI2 },
    Vector { handler: USART1 },
    Vector { handler: USART2 },
    Vector { handler: USART3 },
    Vector { handler: EXTI15_10 },
    Vector { handler: RTC_Alarm },
    Vector { handler: USBWakeUp },
    Vector { handler: TIM8_BRK },
    Vector { handler: TIM8_UP },
    Vector { handler: TIM8_TRIG_COM },
    Vector { handler: TIM8_CC },
    Vector { handler: ADC3 },
    Vector { handler: FMC },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SPI3 },
    Vector { handler: UART4 },
    Vector { handler: UART5 },
    Vector { handler: TIM6_DAC },
    Vector { handler: TIM7 },
    Vector { handler: DMA2_CHANNEL1 },
    Vector { handler: DMA2_CHANNEL2 },
    Vector { handler: DMA2_CHANNEL3 },
    Vector { handler: DMA2_CHANNEL4 },
    Vector { handler: DMA2_CHANNEL5 },
    Vector { handler: ADC4 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: COMP1_2_3 },
    Vector { handler: COMP4_5_6 },
    Vector { handler: COMP7 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: I2C3_EV },
    Vector { handler: I2C3_ER },
    Vector { handler: USB_HP },
    Vector { handler: USB_LP },
    Vector { handler: USB_WakeUp_RMP },
    Vector { handler: TIM20_BRK },
    Vector { handler: TIM20_UP },
    Vector { handler: TIM20_TRG_COM },
    Vector { handler: TIM20_CC },
    Vector { handler: FPU },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SPI4 },
];


#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn () -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;

            f();
        }
    };
}
