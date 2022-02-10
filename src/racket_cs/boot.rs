use std::{ffi::{CStr, CString, NulError}, os::unix::prelude::OsStrExt, path::{Path, PathBuf}, ptr};
use crate::bindings::{self, racket_boot_arguments_t};

pub struct RacketBootArguments<'a> {
    pub boot1_path: &'a Path,
    pub boot1_offset: i64,
    pub boot1_len: i64,
    pub boot2_path: &'a Path,
    pub boot2_offset: i64,
    pub boot2_len: i64,
    pub boot3_path: &'a Path,
    pub boot3_offset: i64,
    pub boot3_len: i64,
    pub argc: i32,
    pub argv: Vec<String>,
    pub exec_file: &'a Path,
    pub run_file: Option<&'a Path>,
    pub collects_dir: &'a [PathBuf],
    pub config_dir: Option<&'a Path>,
    pub dll_dir: Option<PathBuf>,
    pub k_file: Option<&'a Path>,
    pub cs_compiled_subdir: bool,
    pub segment_offset: i64,
    pub dll_open: Option<PathBuf>,
    pub dll_find_object: Option<PathBuf>,
    pub dll_close: Option<PathBuf>,
    pub exit_after: bool,
    pub is_gui: bool,
    pub wm_is_gracket_or_x11_arg_count: bool,
    pub gracket_guid_or_x11_args: Option<String>
}

fn to_cstring<P>(path: P) -> Result<CString, NulError> where P: AsRef<std::path::Path> {
    CString::new(path.as_ref().as_os_str().as_bytes())
}

pub fn boot(r: RacketBootArguments) -> Result<(), NulError> {
    fn normalize_collects_dir(paths: &[PathBuf]) -> CString {
        let mut buffer: Vec<u8> = Vec::with_capacity(128);
        for p in paths {
            let p = p.as_os_str().as_bytes();
            debug_assert!(!p.contains(&0));
            buffer.extend_from_slice(p);
            buffer.push(0);
        }
        unsafe { CString::from_vec_unchecked(buffer) }
    }

    let boot1_path = to_cstring(r.boot1_path)?;
    let boot2_path = to_cstring(r.boot2_path)?;
    let boot3_path = to_cstring(r.boot3_path)?;
    let exec_file = to_cstring(r.exec_file)?;
    // let run_file = to_cstring(r.run_file)?;
    let null_ptr = std::ptr::null();
    let collect_dir = normalize_collects_dir(r.collects_dir);
    // let null_mut_ptr = std::ptr::null_mut();
    let config_dir = r.config_dir.map(to_cstring).transpose()?;
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
        argc: r.argc,
        argv: std::ptr::null_mut(),
        exec_file: exec_file.as_ptr(),
        run_file: null_ptr,
        collects_dir: collect_dir.as_ptr(),
        config_dir: config_dir.map_or(ptr::null(), |p| p.as_ptr()),
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
    unsafe {
        bindings::racket_boot(Box::into_raw(ba));
        // bindings::racket_embedded_load_file(load_file.as_ptr(), 1);
        // bindings::racket_namespace_require(bindings::Sstring_to_symbol(racket.as_ptr()));

        // let racket_module = bindings::Scons(bindings::Sstring_to_symbol("quote"),
        //                        bindings::Scons(bindings::Sstring_to_symbol("evaluator"),
        //                         bindings::Snil));
    }
    Ok(())
}
