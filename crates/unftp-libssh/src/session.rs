#![allow(unused_imports)]
use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use libssh_rs_sys as sys;

use crate::error::{self, Result, *};

/// A Rust wrapper around the ssh_session type, ensuring proper allocation and deallocation
pub(crate) struct SessionStore {
    pub(crate) session: sys::ssh_session,
}

impl SessionStore {
    pub(crate) fn new() -> Result<Self> {
        let sess = unsafe { sys::ssh_new() };
        if sess.is_null() {
            return Err(Error::fatal("ssh_new() failed"));
        }
        Ok(SessionStore { session: sess })
    }

    /// Return the last error ocurred, if any
    fn get_error(&self) -> Option<Error> {
        get_error(self.session as *mut c_void)
    }
}

impl Drop for SessionStore {
    fn drop(&mut self) {
        unsafe { sys::ssh_free(self.session) }
    }
}

impl std::ops::Deref for SessionStore {
    type Target = sys::ssh_session;

    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

impl From<SessionStore> for Session {
    fn from(s: SessionStore) -> Self {
        Session(Arc::new(Mutex::new(s)))
    }
}

unsafe impl Send for SessionStore {}

/// A thread safe implementation of an `ssh_session`
pub struct Session(pub(crate) Arc<Mutex<SessionStore>>);

impl Session {
    pub(crate) fn new() -> Result<Self> {
        let sess = SessionStore::new()?;
        Ok(Session(Arc::new(Mutex::new(sess))))
    }
}
