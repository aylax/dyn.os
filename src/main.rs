#![no_std]
#![no_main]
#![feature(llvm_asm)]

mod lang_items;


const SYSCALL_EXIT: usize = 93;
fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (id)
            : "memory"
            : "volatile"

        );
    }
    ret
}


pub fn sys_exit(xstate: isize) -> isize {
    syscall(SYSCALL_EXIT, xstate as usize, 0, 0)
}

#[no_mangle]
extern "C" fn _start() {
    sys_exit(9);
}
