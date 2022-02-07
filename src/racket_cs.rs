use std::ffi::{CString, NulError};

use crate::bindings::{self, racket_boot_arguments_t};

pub struct RacketBootArguments<'a> {
    pub boot1_path: &'a str,
    pub boot1_offset: i64,
    pub boot1_len: i64,
    pub boot2_path: &'a str,
    pub boot2_offset: i64,
    pub boot2_len: i64,
    pub boot3_path: &'a str,
    pub boot3_offset: i64,
    pub boot3_len: i64,
    // pub argc: i32,
    // pub argv: Vec<&'a str>,
    pub exec_file: &'a str,
    // pub run_file: &'a str,
    pub collects_dir: Vec<String>,
    pub config_dir: &'a str,
    // pub dll_dir: *mut ::std::os::raw::c_void,
    // pub k_file: &'a str,
    // pub cs_compiled_subdir: bool,
    // pub segment_offset: u64,
    // pub dll_open: *mut ::std::os::raw::c_void,
    // pub dll_find_object: *mut ::std::os::raw::c_void,
    // pub dll_close: *mut ::std::os::raw::c_void,
    // pub exit_after: bool,
    // pub is_gui: bool,
    // pub wm_is_gracket_or_x11_arg_count: bool,
    // pub gracket_guid_or_x11_args: *mut ::std::os::raw::c_char,
}

pub fn boot(r: &RacketBootArguments) -> Result<(), NulError> {
    let boot1_path = CString::new(r.boot1_path)?;
    let boot2_path = CString::new(r.boot2_path)?;
    let boot3_path = CString::new(r.boot3_path)?;
    let exec_file = CString::new(r.exec_file)?;
    // let run_file = CString::new(r.run_file)?;
    let null_ptr = std::ptr::null();
    // let null_mut_ptr = std::ptr::null_mut();
    let config_dir = CString::new(r.config_dir)?;
    // let argv_cstr: Result<Vec<CString>, NulError> = r.argv.iter().map(|&arg| CString::new(arg.to_string())).collect();
    // let mut argv_pstr: Vec<_> = argv_cstr?.iter().map(|arg| arg.into_raw()).collect();
    // argv_pstr.push(std::ptr::null_mut());
    let ba = racket_boot_arguments_t {
        boot1_path: boot1_path.as_ptr(),
        boot1_offset: r.boot1_offset,
        boot1_len: r.boot1_len,
        boot2_path: boot2_path.as_ptr(),
        boot2_offset: r.boot2_offset,
        boot2_len: r.boot2_len,
        boot3_path: boot3_path.as_ptr(),
        boot3_offset: r.boot3_offset,
        boot3_len: r.boot3_len,
        argc: 0,
        argv: std::ptr::null_mut(),
        exec_file: exec_file.as_ptr(),
        run_file: null_ptr,
        collects_dir: null_ptr,
        config_dir: null_ptr,
        dll_dir: std::ptr::null_mut(),
        k_file: null_ptr,
        cs_compiled_subdir: 1,
        segment_offset: 0,
        dll_open: std::ptr::null_mut(),
        dll_find_object: std::ptr::null_mut(),
        dll_close: std::ptr::null_mut(),
        exit_after: 0,
        is_gui: 0,
        wm_is_gracket_or_x11_arg_count: 0,
        gracket_guid_or_x11_args: std::ptr::null_mut(),
    };
    let ba = Box::new(ba);
    let load_file = CString::new("./smol.bin").unwrap();
    let racket = CString::new("racket").unwrap();
    unsafe {
        bindings::racket_boot(Box::into_raw(ba));
        bindings::racket_embedded_load_file(load_file.as_ptr(), 1);
        bindings::racket_namespace_require(bindings::Sstring_to_symbol(racket.as_ptr()));

        // let racket_module = bindings::Scons(bindings::Sstring_to_symbol("quote"),
        //                        bindings::Scons(bindings::Sstring_to_symbol("evaluator"),
        //                         bindings::Snil));
    }
    Ok(())
}

// fn normalize_collects_dir(paths: Vec<&'_ str>) -> Vec<> {
//     let mut s = paths.join("\0");
//     s.push('\0');
// }
