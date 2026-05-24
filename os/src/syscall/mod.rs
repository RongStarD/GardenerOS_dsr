pub fn syscall(syscall_id: usize, args: [usize; 3], _cx: &mut crate::trap::TrapContext) -> isize {
    match syscall_id {
        64 => {
            let buf = args[1] as *const u8;
            let len = args[2];
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            0
        }
        93 => {
            println!("[kernel] Application exited with code {}", args[0]);
            crate::batch::run_next_app();
        }
        _ => panic!("Unsupported syscall: {}", syscall_id),
    }
}
