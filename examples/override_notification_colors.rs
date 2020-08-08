use std::error::Error;
use std::thread;
use std::time::Duration;
use transition::{Led, Transition};

fn main() -> Result<(), Box<dyn Error>> {
    let notification = Transition::default().on_success(&Led::Orange).start()?;
    thread::sleep(Duration::from_secs(5));
    notification.notify_success()?;

    thread::sleep(Duration::from_secs(5));

    let notification = Transition::default().on_failure(&Led::Cyan).start()?;
    thread::sleep(Duration::from_secs(5));
    notification.notify_failure()?;

    Ok(())
}
