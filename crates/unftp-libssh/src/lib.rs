#![allow(dead_code)]
#![allow(unused_variables)]
use std::sync::Once;

use libssh_rs_sys as sys;

pub mod bind;
pub mod channel;
pub mod error;
pub mod session;

pub use bind::{Bind, BindBuilder};
pub use error::Error;
pub use session::Session;

struct LibraryState {}
impl LibraryState {
    pub fn new() -> Option<Self> {
        let res = unsafe { sys::ssh_init() };
        if res != sys::SSH_OK as i32 {
            None
        } else {
            Some(Self {})
        }
    }
}
impl Drop for LibraryState {
    fn drop(&mut self) {
        unsafe { sys::ssh_finalize() };
    }
}

static INIT: Once = Once::new();
static mut LIB: Option<LibraryState> = None;
