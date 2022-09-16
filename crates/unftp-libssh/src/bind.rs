#![allow(unused_imports)]
use std::{
    ffi::{c_void, CString},
    path::{Path, PathBuf},
};

use libssh_rs_sys as sys;

use crate::{
    error::{is_error, Result},
    session::SessionStore,
    *,
};

/// A Rust wrapper around the ssh_bind type, ensuring proper allocation and deallocation
struct BindStore {
    bind: sys::ssh_bind,
}

impl BindStore {
    fn new() -> Result<Self> {
        let bind = unsafe { sys::ssh_bind_new() };
        if bind.is_null() {
            return Err(Error::fatal("ssh_bind_new() failed"));
        }
        // non-blocking to support async
        unsafe {
            sys::ssh_bind_set_blocking(bind, 0);
        };
        Ok(BindStore { bind })
    }

    /// Set option (ssh_bind_options_set)
    fn set_option_string(&mut self, option: sys::ssh_bind_options_e, value: &str) -> Result<()> {
        let s = CString::new(value).expect("create cstring");

        let rc = unsafe { sys::ssh_bind_options_set(self.bind, option, s.as_ptr() as _) };
        if is_error(rc) {
            Ok(())
        } else {
            Err(Error::fatal("error setting option"))
        }
    }

    /// Return the last error ocurred, if any
    fn get_error(&self) -> Option<Error> {
        error::get_error(self.bind as *mut c_void)
    }
}

unsafe impl Send for BindStore {}

impl std::ops::Deref for BindStore {
    type Target = sys::ssh_bind;

    fn deref(&self) -> &Self::Target {
        &self.bind
    }
}

impl Drop for BindStore {
    fn drop(&mut self) {
        unsafe {
            sys::ssh_bind_free(self.bind);
        }
    }
}

pub struct Bind {
    bind: BindStore,
    addr: String,
    port: u16,
}

impl Bind {
    ///
    pub fn listen(&mut self) -> Result<()> {
        let res = unsafe { sys::ssh_bind_listen(*self.bind) };
        Ok(())
    }

    pub fn accept(&mut self) -> Result<Session> {
        let sess = SessionStore::new()?;
        let rc = unsafe { sys::ssh_bind_accept(*self.bind, *sess) };
        if is_error(rc) {
            Ok(sess.into())
        } else {
            Err(Error::fatal(""))
        }
    }
}

/// A builder for constructing [`Bind`]s.
///
/// [`Bind`]: struct.Bind.html
#[derive(Default)]
pub struct BindBuilder {
    addr: String,
    port: u16,

    /// RSA Host key path
    rsa_key: Option<PathBuf>,
    /// ECDSA Host Key path
    ecdsa_key: Option<PathBuf>,

    /// Comma separated list of acceptable cipher
    ciphers: Option<String>,
    /// Comma separated list of acceptable key exchange methods
    key_exchange: Option<String>,
    /// Comma separated list of acceptable HMACs
    hmac: Option<String>,
    /// Comma separated list of acceptable key types
    key_types: Option<String>,
}

impl BindBuilder {
    /// Construct a new builder
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to bind to
    /// * `port` - The port to bind to
    ///
    /// # Examples
    /// ```
    /// use unftp_libssh::*;
    /// let bind = BindBuilder("127.0.0.1", 2222).build();
    /// ```
    pub fn new(addr: &str, port: u16) -> Self {
        BindBuilder {
            addr: addr.to_owned(),
            port,
            ..Default::default()
        }
    }

    pub fn rsa_key(&mut self, path: &Path) -> &mut Self {
        self.rsa_key = Some(path.to_owned());
        self
    }

    pub fn ecdsa_key(&mut self, path: &Path) -> &mut Self {
        self.ecdsa_key = Some(path.to_owned());
        self
    }

    pub fn ciphers(&mut self, ciphers: String) -> &mut Self {
        self.ciphers = Some(ciphers);
        self
    }

    pub fn key_exchange(&mut self, key_exchange: String) -> &mut Self {
        self.key_exchange = Some(key_exchange);
        self
    }

    pub fn hmac(&mut self, hmac: String) -> &mut Self {
        self.hmac = Some(hmac);
        self
    }

    pub fn key_types(&mut self, key_types: String) -> &mut Self {
        self.key_types = Some(key_types);
        self
    }

    /// Construct the [`Bind`]
    ///
    /// [`Bind`]: struct.Bind.html  
    pub fn build(&self) -> Result<Bind> {
        let mut bind = BindStore::new()?;

        bind.set_option_string(sys::ssh_bind_options_e::SSH_BIND_OPTIONS_BINDADDR, &self.addr)?;
        bind.set_option_string(sys::ssh_bind_options_e::SSH_BIND_OPTIONS_BINDPORT_STR, &format!("{}", self.port))?;

        Ok(Bind {
            bind,
            addr: self.addr.clone(),
            port: self.port,
        })
    }
}
