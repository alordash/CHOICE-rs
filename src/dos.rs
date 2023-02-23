use core::arch::asm;

pub fn wait(millis: u32) {
    unsafe {
        let real_millis = millis * 1000;
        let cx = ((real_millis & 0xFFFF0000) >> 16) as u16;
        let dx = (real_millis & 0x0000FFFF) as u16;
        asm!(
            "int 15h",
            in("ax") 0x8600 as u16,
            in("cx") cx,
            in("dx") dx,
        );
    }
}

pub fn set_wait_interval(millis: u32, byte_ptr: *mut u8) {
    unsafe {
        *byte_ptr = 0;
        let real_millis = millis * 1000;
        let cx = ((real_millis & 0xFFFF0000) >> 16) as u16;
        let dx = (real_millis & 0x0000FFFF) as u16;
        asm!(
            "int 15h",
            in("ax") 0x8300 as u16,
            in("cx") cx,
            in("dx") dx,
            in("bx") byte_ptr as u16,
        );
    }
}

pub fn stop_wait_interval(byte_ptr: *mut u8) {
    unsafe {
        *byte_ptr = 0;
        asm!(
            "int 15h",
            in("ax") 0x8301 as u16
        );
    }
}
