use core::arch::asm;
use core::panic::PanicInfo;

use crate::io::print_str;

pub unsafe fn exit_with_code(exit_code: u8) -> ! {
    asm!(
        "int 21h",
        in("ah") 0x4C_u8,
        in("al") exit_code,
    );
    loop {}
}

pub unsafe fn panic_exit(err_msg: &str, exit_code: u8) -> ! {
    print_str(err_msg);
    exit_with_code(exit_code)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_str("PANIC!!!$");
    loop {}
}
