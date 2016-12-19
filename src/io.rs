use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std;

use futures::sync::oneshot;
use tokio::reactor::{Core, Remote};
use tokio_timer;

use rpc::Messenger;
use dns;
use rpc;
use rpc::ConnectionOptions;



#[derive(Clone)]
pub struct Io {
    pub reactors: Vec<Remote>,
    pub resolver: dns::Resolver,
    pub timer: tokio_timer::Timer,
    pub messenger: rpc::Messenger,
    _reactor_shutdown: Arc<Vec<oneshot::Sender<()>>>,
}

fn spawn_reactor() -> (Remote, oneshot::Sender<()>) {
    let (remote_send, remote_recv) = std::sync::mpsc::channel();
    let (send, recv) = oneshot::channel();

    thread::spawn(move || {
        let mut core = Core::new().unwrap();
        remote_send.send(core.remote()).unwrap();
        let _ = core.run(recv);
    });

    (remote_recv.recv().unwrap(), send)
}

impl Io {

    pub fn new(num_reactors: usize) -> Io {
        let mut reactors = Vec::new();
        let mut shutdowns = Vec::new();

        for _ in 0..num_reactors {
            let (remote, shutdown) = spawn_reactor();
            reactors.push(remote);
            shutdowns.push(shutdown);
        }

        let resolver = dns::Resolver::new();
        let timer = tokio_timer::wheel().tick_duration(Duration::from_millis(10))
                                        .num_slots(8192)
                                        .build();
        let messenger = Messenger::new(&reactors, ConnectionOptions::default());

        Io {
            reactors: reactors,
            resolver: resolver,
            timer: timer,
            messenger: messenger,
            _reactor_shutdown: Arc::new(shutdowns),
        }
    }

    pub fn with_reactors(reactors: Vec<Remote>) -> Io {
        let resolver = dns::Resolver::new();
        let timer = tokio_timer::wheel().tick_duration(Duration::from_millis(10))
                                        .num_slots(8192)
                                        .build();
        let messenger = Messenger::new(&reactors, ConnectionOptions::default());

        Io {
            reactors: reactors,
            resolver: resolver,
            timer: timer,
            messenger: messenger,
            _reactor_shutdown: Arc::new(Vec::new()),
        }
    }
}
