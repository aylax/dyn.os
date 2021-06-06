#![no_std]
#![no_main]
#![feature(llvm_asm, global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));


fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}


#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("\u{1b}[31m[kernel] Hello world!\u{1b}[0m");
    panic!("Unreachable in rust main!");
}
 

