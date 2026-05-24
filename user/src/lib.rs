#![no_std]
#![feature(linkage)]

#[macro_use]
pub mod console;
pub mod syscall;
pub mod lang_items;

fn clear_bss() {
    unsafe extern "C" {
        fn start_bss();
        fn end_bss();
    }
    let start_bss_ptr = unsafe { start_bss as *const () as usize };
    let end_bss_ptr = unsafe { end_bss as *const () as usize };
    (start_bss_ptr..end_bss_ptr).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}

pub fn write(fd: usize, buf: &[u8]) -> isize { syscall::sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> isize { syscall::sys_exit(exit_code) }
