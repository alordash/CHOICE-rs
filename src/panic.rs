use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // dos::print("\nPanic!$".as_ptr());
    // dos::exit();
    loop {}
}
