#![allow(non_upper_case_globals, non_camel_case_types, non_camel_case_types, non_snake_case)]

extern crate libc;
#[cfg(any(target_os = "linux",
          target_os = "freebsd",
          target_os = "openbsd",
          target_os = "netbsd"))]
extern crate x11;



pub mod vdpau;
#[cfg(any(target_os = "linux",
          target_os = "freebsd",
          target_os = "openbsd",
          target_os = "netbsd"))]
pub mod vdpau_x11;
