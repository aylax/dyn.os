use core::panic::PanicInfo;
use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        println!(
            "[kernel] panicked at '{}', {}:{}:{}",
            info.message.unwrap(), 
            loc.file(), 
            loc.line(), 
            loc.column()
        );
    } else {
        println!(
            "[kernel] panicked at '{}'",
            info.message().unwrap()
        );
    }
    shutdown()
}

