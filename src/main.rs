#![no_std]
#![no_main]
#![feature(llvm_asm)]

mod lang_items;

fn syscall(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"
            : "volatile"
        );
    }
    ret
}


const SYS_EXIT: usize = 93;
pub fn sys_exit(xstate: usize) -> usize {
    syscall(SYS_EXIT, xstate, 0 , 0)
} 

const SBI_SHUTDOWN: usize = 8;
pub fn shutdown() -> ! {
    syscall(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}


const SYSCALL_WRITE: usize = 64;
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
  syscall(SYSCALL_WRITE, fd, buffer.as_ptr() as usize, buffer.len())
}


struct Stdout;
impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}
pub fn print(args: core::fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}


#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[no_mangle]
extern "C" fn _start() {
    shutdown()
}
