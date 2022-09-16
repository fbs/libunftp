#![allow(dead_code)]
#![allow(unused_variables)]

use std::path::PathBuf;

use libunftp::SftpServer;
use unftp_libssh::*;
use unftp_sbe_fs::Filesystem;

use tokio::runtime::Runtime;

fn main() -> std::result::Result<(), String> {
    let builder = BindBuilder::new("127.0.0.1", 2222);
    let bind = builder.build().expect("failed to build server");

    let mut rt = Runtime::new().unwrap();

    let p = PathBuf::from("/tmp/sftp");
    let storage = Box::new(move || {
        let p = &p.clone();
        Filesystem::new(p)
    });

    let server = SftpServer::new(storage, bind);

    let handle = rt.spawn(bind.listen());

    Ok(())
}
