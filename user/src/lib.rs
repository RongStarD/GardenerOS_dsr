#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#[macro_use] pub mod console;
mod syscall; mod lang_items;
use syscall::*;
#[unsafe(no_mangle)] #[link_section = ".text.entry"] pub extern "C" fn _start() -> ! { clear_bss(); exit(main()); panic!("unreachable!"); }
#[linkage = "weak"] #[unsafe(no_mangle)] fn main() -> i32 { panic!("Cannot find main!"); }
fn clear_bss() { extern "C" { fn sbss(); fn ebss(); } (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) }); }
pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }
