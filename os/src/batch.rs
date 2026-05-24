use core::cell::RefCell;
use lazy_static::*;
const APP_BASE: usize = 0x80400000;

struct AppManager { inner: RefCell<AppManagerInner> }
struct AppManagerInner { num_app: usize, current_app: usize, app_start: [usize; 17] }
unsafe impl Sync for AppManager {}

lazy_static! {
    static ref APP_MANAGER: AppManager = AppManager {
        inner: RefCell::new({
            extern "C" { fn _num_app(); }
            let ptr = _num_app as *const usize;
            let num = unsafe { ptr.read_volatile() };
            let mut start = [0; 17];
            let raw = unsafe { core::slice::from_raw_parts(ptr.add(1), num + 1) };
            start[..=num].copy_from_slice(raw);
            AppManagerInner { num_app: num, current_app: 0, app_start: start }
        }),
    };
}

pub fn run_next_app() -> ! {
    let mut m = APP_MANAGER.inner.borrow_mut();
    let id = m.current_app;
    if id >= m.num_app { panic!("All apps finished!"); }
    unsafe {
        let src = core::slice::from_raw_parts(m.app_start[id] as *const u8, m.app_start[id+1] - m.app_start[id]);
        core::slice::from_raw_parts_mut(APP_BASE as *mut u8, src.len()).copy_from_slice(src);
    }
    m.current_app += 1;
    panic!("Need trap implementation");
}
