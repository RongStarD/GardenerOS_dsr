use core::panic::PanicInfo;
#[panic_handler] fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() { println!("Panicked at {}:{} {}", loc.file(), loc.line(), info.message()); }
    else { println!("Panicked: {}", info.message()); }
    crate::exit(-1); loop {}
}
