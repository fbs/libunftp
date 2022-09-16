#![allow(unused_imports)]
#![allow(dead_code)]
use std::{pin::Pin, sync::Arc};

use crate::{
    auth::{anonymous::AnonymousAuthenticator, Authenticator, UserDetail},
    storage::{Metadata, StorageBackend},
};

use unftp_libssh::Bind;

/// a thing that does things
pub struct SftpServer<Storage, User>
where
    Storage: StorageBackend<User>,
    User: UserDetail,
{
    storage: Arc<dyn (Fn() -> Storage) + Send + Sync>,
    authenticator: Arc<dyn Authenticator<User>>,
    bind: Bind,
}

impl<Storage, User> SftpServer<Storage, User>
where
    Storage: StorageBackend<User> + 'static,
    Storage::Metadata: Metadata,
    User: UserDetail + 'static,
{
    /// Construct a new [`Server`] with the given [`StorageBackend`] generator and an [`AnonymousAuthenticator`]
    ///
    /// [`Server`]: struct.Server.html
    /// [`StorageBackend`]: ../storage/trait.StorageBackend.html
    /// [`AnonymousAuthenticator`]: ../auth/struct.AnonymousAuthenticator.html
    pub fn new(sbe_generator: Box<dyn (Fn() -> Storage) + Send + Sync>, bind: Bind) -> Self
    where
        AnonymousAuthenticator: Authenticator<User>,
    {
        SftpServer {
            storage: Arc::from(sbe_generator),
            authenticator: Arc::new(AnonymousAuthenticator {}),
            bind,
        }
    }

    async fn listen() {}
}
