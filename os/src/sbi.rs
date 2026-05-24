use core::arch::asm;

pub fn console_putchar(c: usize) {
    unsafe {
        asm!(
            "li x10, {0}",
            "li x17, 1",
            "ecall",
            const 1, // SBI_CONSOLE_PUTCHAR
            in("x10") c,
            in("x17") 1,
        );
    }
}

pub fn shutdown() -> ! {
    unsafe {
        asm!("li x17, 8", "ecall"); // SBI_SHUTDOWN
    }
    loop {}
}
