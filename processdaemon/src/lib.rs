//! process daemon
//! Stable operation on systems without nohup

/// process_daemon
/// # Examples
/// ```
/// use std::thread;
/// use processdaemon::process_daemon;
/// thread::spawn(process_daemon);
/// ```
use signal_hook::{iterator::Signals, SIGHUP,SIGINT,SIGQUIT,SIGTERM};
use std::{thread, time::Duration};
pub fn process_daemon() {
    let signals = match Signals::new(&[SIGHUP,SIGINT,SIGQUIT,SIGTERM]) {
        Ok(t) => t,
        Err(e) => panic!(e),
    };

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    // Following code does the actual work, and can be interrupted by pressing
    // Ctrl-C. As an example: Let's wait a few seconds.
    thread::sleep(Duration::from_secs(2));
}