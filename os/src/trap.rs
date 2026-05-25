use core::arch::global_asm;
global_asm!(".section .text\n.globl __alltraps\n.globl __restore\n.align 2\n__alltraps:\n    csrrw sp, sscratch, sp\n    sd x1, 1*8(sp)\n    sd x3, 3*8(sp)\n    sd x5, 5*8(sp)\n    sd x6, 6*8(sp)\n    sd x7, 7*8(sp)\n    sd x10, 10*8(sp)\n    sd x11, 11*8(sp)\n    sd x12, 12*8(sp)\n    sd x13, 13*8(sp)\n    sd x14, 14*8(sp)\n    sd x15, 15*8(sp)\n    sd x16, 16*8(sp)\n    sd x17, 17*8(sp)\n    sd x28, 28*8(sp)\n    sd x29, 29*8(sp)\n    sd x30, 30*8(sp)\n    sd x31, 31*8(sp)\n    csrr t0, sstatus\n    csrr t1, sepc\n    sd t0, 32*8(sp)\n    sd t1, 33*8(sp)\n    csrr t2, sscratch\n    sd t2, 2*8(sp)\n    mv a0, sp\n    call trap_handler\n__restore:\n    mv sp, a0\n    ld t0, 32*8(sp)\n    ld t1, 33*8(sp)\n    csrw sstatus, t0\n    csrw sepc, t1\n    ld x1, 1*8(sp)\n    ld x3, 3*8(sp)\n    ld x5, 5*8(sp)\n    ld x6, 6*8(sp)\n    ld x7, 7*8(sp)\n    ld x10, 10*8(sp)\n    ld x11, 11*8(sp)\n    ld x12, 12*8(sp)\n    ld x13, 13*8(sp)\n    ld x14, 14*8(sp)\n    ld x15, 15*8(sp)\n    ld x16, 16*8(sp)\n    ld x17, 17*8(sp)\n    ld x28, 28*8(sp)\n    ld x29, 29*8(sp)\n    ld x30, 30*8(sp)\n    ld x31, 31*8(sp)\n    ld sp, 2*8(sp)\n    sret:");
#[repr(C)] pub struct TrapContext { pub x: [usize; 32], pub sstatus: usize, pub sepc: usize }
#[no_mangle] pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = riscv::register::scause::read();
    let stval = riscv::register::stval::read();
    match scause.cause() {
        riscv::register::scause::Trap::Exception(riscv::register::scause::Exception::UserEnvCall) => {
            cx.sepc += 4;
            let ret = crate::syscall::syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]);
            cx.x[10] = ret as usize;
        },
        _ => { panic!("Unsupported trap: {:?}, stval: {:#x}!", scause.cause(), stval); }
    }
    cx
}
pub fn init() { unsafe { riscv::register::stvec::write(__alltraps as usize, riscv::register::stvec::TrapMode::Direct); } }
