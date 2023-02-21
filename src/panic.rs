use core::panic::PanicInfo;

use crate::io::print_str;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // unsafe { print_str(_info.payload().downcast_ref::<&str>().unwrap()) };
    unsafe { print_str("PANIC!!!$") };
    loop {}
}
