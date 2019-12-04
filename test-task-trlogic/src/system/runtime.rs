use failure::{Error, err_msg};
use futures_old::Future;
use signal_hook::iterator::Signals;

pub struct Runtime {
    runtime: tokio::runtime::Runtime,
}

impl Runtime {
    const GRACEFUL_SHUTDOWN: [i32; 1] = [signal_hook::SIGINT];
    const FORCE_SHUTDOWN: [i32; 1] = [signal_hook::SIGTERM];
    pub fn new() -> Result<Runtime, Error> {
        let runtime = tokio::runtime::Runtime::new().map_err(err_msg)?;
        return Ok(Runtime {
            runtime,
        });
    }

    pub fn spawn(&mut self, future: impl Future<Item=(), Error=()> + Send + 'static) {
        self.runtime.spawn(future);
    }

    pub fn execute(self) -> Result<(), Error> {
        let mut signals = Vec::new();
        signals.extend_from_slice(&Self::FORCE_SHUTDOWN);
        signals.extend_from_slice(&Self::GRACEFUL_SHUTDOWN);
        let signals = Signals::new(signals).map_err(err_msg)?;
        let signals = signals.forever();
        for signal in signals {
            if Self::GRACEFUL_SHUTDOWN.contains(&signal) {
                let _ = self.runtime.shutdown_on_idle();
                return Ok(());
            }
            if Self::FORCE_SHUTDOWN.contains(&signal) {
                let _ = self.runtime.shutdown_now();
                return Ok(());
            }
        }
        return Ok(());
    }
}