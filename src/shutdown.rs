use std::io;
use tokio::{select, signal};

#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

pub struct Shutdown {
    #[cfg(unix)]
    sigterm: tokio::signal::unix::Signal,
}
impl Shutdown {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            #[cfg(unix)]
            sigterm: signal(SignalKind::terminate())?,
        })
    }

    pub async fn wait_for_shutdown(&mut self) {
        #[cfg(unix)]
        let unix_signal = self.sigterm.recv();
        #[cfg(not(unix))]
        let unix_signal = std::future::pending::<()>();

        select! {
            _ = signal::ctrl_c() => {},
            _ = unix_signal => {},
        }
    }
}
