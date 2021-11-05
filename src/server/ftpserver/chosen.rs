//! Represents the chosen options that the libunftp user opted for.

use crate::storage::Metadata;
use crate::{
    auth::Authenticator,
    auth::UserDetail,
    options::{FtpsRequired, PassiveHost, SiteMd5},
    server::controlchan,
    server::tls::FtpsConfig,
    storage::StorageBackend,
};
use std::{ops::Range, sync::Arc, time::Duration};

pub struct OptionsHolder<Storage, User>
where
    Storage: StorageBackend<User>,
    User: UserDetail,
{
    pub storage: Arc<dyn (Fn() -> Storage) + Send + Sync>,
    pub greeting: &'static str,
    pub authenticator: Arc<dyn Authenticator<User>>,
    pub passive_ports: Range<u16>,
    pub passive_host: PassiveHost,
    pub ftps_config: FtpsConfig,
    pub collect_metrics: bool,
    pub idle_session_timeout: Duration,
    pub logger: slog::Logger,
    pub ftps_required_control_chan: FtpsRequired,
    pub ftps_required_data_chan: FtpsRequired,
    pub site_md5: SiteMd5,
}

impl<Storage, User> From<&OptionsHolder<Storage, User>> for controlchan::LoopConfig<Storage, User>
where
    User: UserDetail + 'static,
    Storage: StorageBackend<User> + 'static,
    Storage::Metadata: Metadata,
{
    fn from(server: &OptionsHolder<Storage, User>) -> Self {
        controlchan::LoopConfig {
            authenticator: server.authenticator.clone(),
            storage: (server.storage)(),
            ftps_config: server.ftps_config.clone(),
            collect_metrics: server.collect_metrics,
            greeting: server.greeting,
            idle_session_timeout: server.idle_session_timeout,
            passive_ports: server.passive_ports.clone(),
            passive_host: server.passive_host.clone(),
            logger: server.logger.new(slog::o!()),
            ftps_required_control_chan: server.ftps_required_control_chan,
            ftps_required_data_chan: server.ftps_required_data_chan,
            site_md5: server.site_md5,
        }
    }
}
