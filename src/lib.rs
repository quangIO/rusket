pub mod bindings;
pub mod racket_cs;

#[cfg(test)]
mod tests {
    use std::{env, ffi::CString, os::unix::prelude::OsStrExt, path::PathBuf};

    use crate::bindings::default_boot;

    #[test]
    fn it_works() {}
}
