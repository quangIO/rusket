use std::{env, ffi::CString, os::unix::prelude::OsStrExt, path::PathBuf, ptr};

use rustket::bindings;

fn main() {
    let lib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("racket");
    let exec_file = CString::new(env::current_exe().unwrap().as_os_str().as_bytes()).unwrap();
    let boot1_path = CString::new(lib_path.join("petite.boot").as_os_str().as_bytes()).unwrap();
    let boot2_path = CString::new(lib_path.join("scheme.boot").as_os_str().as_bytes()).unwrap();
    let boot3_path = CString::new(lib_path.join("racket.boot").as_os_str().as_bytes()).unwrap();

    let image = CString::new("/Users/qt/racket/embed/image.bin").unwrap();

    unsafe {
        bindings::racket_easy_boot(
            exec_file.as_ptr(),
            boot1_path.as_ptr(),
            boot2_path.as_ptr(),
            boot3_path.as_ptr(),
            ptr::null(),
            ptr::null()
        );

        bindings::run_something(image.as_ptr());
    }
}
