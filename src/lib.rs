#![no_std]

use core::{panic::PanicInfo, ptr};

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

    // let count = &__edata as *const u32 as usize - &__sdata as *const u32 as usize;
    // ptr::copy_nonoverlapping(&__sidata as *const u32, &mut __sdata as *mut u32, count);


    extern "Rust" {
        fn main() -> !;
    }

    main();
}


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
