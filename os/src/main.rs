#![no_std]
#![no_main]

use core::arch::{asm, global_asm};

global_asm!(include_str!("entry.asm"));

fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

pub fn console_putchar(c: usize) {
    sbi_call(1, c, 0, 0);
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    for c in b"Hello Kernel From Rust!\n" {
        console_putchar(*c as usize);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
