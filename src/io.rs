use std::time::Duration;

use dns;
use rpc;
use tokio_timer;

#[derive(Clone)]
pub struct Io {
    pub messenger: rpc::Messenger,
    pub resolver: dns::Resolver,
    pub timer: tokio_timer::Timer,
}

impl Io {

    pub fn new(messenger: rpc::Messenger) -> Io {
        Io {
            messenger: messenger,
            resolver: dns::Resolver::new(),
            timer: tokio_timer::wheel().tick_duration(Duration::from_millis(10))
                                       .num_slots(8192)
                                       .build(),
        }
    }

    #[inline]
    pub fn messenger(&mut self) -> &mut rpc::Messenger {
        &mut self.messenger
    }

    #[inline]
    pub fn resolver(&mut self) -> &mut dns::Resolver {
        &mut self.resolver
    }

    #[inline]
    pub fn timer(&mut self) -> &mut tokio_timer::Timer {
        &mut self.timer
    }
}
