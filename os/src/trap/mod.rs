pub mod context;
pub use context::TrapContext;
use core::arch::global_asm;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" { fn __alltraps(); }
    unsafe {
        riscv::register::stvec::write(__alltraps as usize, riscv::register::stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    println!("[kernel] Trap detected!");
    cx
}
