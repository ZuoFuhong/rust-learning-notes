use log::{debug, error, info, log_enabled, Level};

fn main() {
    env_logger::init();
    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4;
        info!("the answer was: {}", x);
    }
}
