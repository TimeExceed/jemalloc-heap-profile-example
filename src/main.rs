use std::path::*;
use std::ffi::{CString, c_char};

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn dump_heap(i: usize) {
    const PROF_DUMP: &[u8] = b"prof.dump\0";

    let path_buf = PathBuf::from(format!("./test.{}.hprof", i));
    let path = path_buf.to_str().unwrap().to_string();
    let mut bytes = CString::new(path.as_str()).unwrap().into_bytes_with_nul();
    let ptr = bytes.as_mut_ptr() as *mut c_char;
    unsafe {
        jemalloc_ctl::raw::write(PROF_DUMP, ptr).unwrap();
    };
    println!("{} dumped.", path);
}

fn main() {
    let mut res = vec![];
    let mut i = 0;
    loop {
        if i % 10 == 0 {
            dump_heap(i);
        }
        let mut x = Vec::<u8>::new();
        x.resize(1024 * 1024, 1u8);
        res.push(x);
        std::thread::sleep(std::time::Duration::from_secs(1));
        i += 1;
    }
}
