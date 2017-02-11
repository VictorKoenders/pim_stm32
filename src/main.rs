#![feature(asm)]
#![feature(lang_items)]

#![no_main]
#![no_std]

use core::ptr;

#[inline(never)]
#[no_mangle]
#[export_name = "_main"]
pub fn main() -> ! {
    // Document: http://www.st.com/content/ccc/resource/technical/document/reference_manual/59/b9/ba/7f/11/af/43/d5/CD00171190.pdf/files/CD00171190.pdf/jcr:content/translations/en.CD00171190.pdf
    // SVD2Rust: https://docs.rs/svd2rust/0.2.1/svd2rust/
    // SVD: https://raw.githubusercontent.com/posborne/cmsis-svd/master/data/STMicro/STM32F103xx.svd
    
    // configure PC13 as output
    unsafe { ptr::write_volatile::<u32>((0x40021000 + 0x18) as *mut u32, 1 << 4) };

    const GPIO_C_ADDRESS: u32 = 0x40011000;
    const ODR_OFFSET: u32 = 0x10;
    const PIN13_HIGH: u32 = 1 << 13;
    const PIN13_LOW: u32 = 1 << (13 + 16);

    loop {
        for _ in 0..10000 {
            unsafe { asm!("nop") };
        }

        unsafe { ptr::write_volatile::<u32>((GPIO_C_ADDRESS + ODR_OFFSET) as *mut u32, PIN13_HIGH) };

        for _ in 0..10000 {
            unsafe { asm!("nop") };
        }

        unsafe { ptr::write_volatile::<u32>((GPIO_C_ADDRESS + ODR_OFFSET) as *mut u32, PIN13_LOW) };
    }
}

#[export_name = "_default_exception_handler"]
pub unsafe extern "C" fn exception_handler() {
    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32
) -> ! {
    loop {}
}

pub type Handler = fn();

#[export_name = "_EXCEPTIONS"]
pub static EXCEPTIONS: [Option<Handler>; 14] = [
    None, // Some(_nmi),
    None, // Some(_hard_fault),
    None, // Some(_memmanage_fault),
    None, // Some(_bus_fault),
    None, // Some(_usage_fault),
    None,
    None,
    None,
    None,
    None, // Some(_svcall),
    None,
    None,
    None, // Some(_pendsv),
    None, // Some(_systick)
];

// For more info, see f3/src/interrupts.rs
// TODO: Find documentation on interrupts
// TODO: Add information on the interrupts
// TODO: Figure out how to enable interrupts
#[doc(hidden)]
#[export_name = "_INTERRUPTS"]
pub static INTERRUPTS: [Option<Handler>; 85] = [
    None, // Some(_wwdg),
    None, // Some(_pvd),
    None, // Some(_tamper_stamp),
    None, // Some(_rtc_wkup),
    None, // Some(_flash),
    None, // Some(_rcc),
    None, // Some(_exti0),
    None, // Some(_exti1),
    None, // Some(_exti2_ts),
    None, // Some(_exti3),
    None, // Some(_exti4),
    None, // Some(_dma1_channel1),
    None, // Some(_dma1_channel2),
    None, // Some(_dma1_channel3),
    None, // Some(_dma1_channel4),
    None, // Some(_dma1_channel5),
    None, // Some(_dma1_channel6),
    None, // Some(_dma1_channel7),
    None, // Some(_adc1_2),
    None, // Some(_usb_hp_can_tx),
    None, // Some(_usb_lp_can_rx0),
    None, // Some(_can_rx1),
    None, // Some(_can_sce),
    None, // Some(_exti9_5),
    None, // Some(_tim1_brk_tim15),
    None, // Some(_tim1_up_tim16),
    None, // Some(_tim1_trg_com_tim17),
    None, // Some(_tim1_cc),
    None, // Some(_tim2),
    None, // Some(_tim3),
    None, // Some(_tim4),
    None, // Some(_i2c1_ev),
    None, // Some(_i2c1_er),
    None, // Some(_i2c2_ev),
    None, // Some(_i2c2_er),
    None, // Some(_spi1),
    None, // Some(_spi2),
    None, // Some(_usart1),
    None, // Some(_usart2),
    None, // Some(_usart3),
    None, // Some(_exti15_10),
    None, // Some(_rtc_alarm),
    None, // Some(_usb_wake_up),
    None, // Some(_tim8_brk),
    None, // Some(_tim8_up),
    None, // Some(_tim8_trg_com),
    None, // Some(_tim8_cc),
    None, // Some(_adc3),
    None, // Some(_fmc),
    None,
    None,
    None, // Some(_spi3),
    None, // Some(_uart4),
    None, // Some(_uart5),
    None, // Some(_tim6_dac),
    None, // Some(_tim7),
    None, // Some(_dma2_channel1),
    None, // Some(_dma2_channel2),
    None, // Some(_dma2_channel3),
    None, // Some(_dma2_channel4),
    None, // Some(_dma2_channel5),
    None, // Some(_adc4),
    None,
    None,
    None, // Some(_comp1_2_3),
    None, // Some(_comp4_5_6),
    None, // Some(_comp7),
    None,
    None,
    None,
    None,
    None,
    None, // Some(_i2c3_ev),
    None, // Some(_i2c3_er),
    None, // Some(_usb_hp),
    None, // Some(_usb_lp),
    None, // Some(_usb_wake_up_rmp),
    None, // Some(_tim20_brk),
    None, // Some(_tim20_up),
    None, // Some(_tim20_trg_com),
    None, // Some(_tim20_cc),
    None, // Some(_fpu),
    None,
    None,
    None // Some(_spi4)
];
