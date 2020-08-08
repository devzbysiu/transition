use std::error::Error;
use std::thread;
use std::time::Duration;
use transition::{Led, Transition};

fn main() -> Result<(), Box<dyn Error>> {
    let notification = Transition::new(&[Led::Cyan, Led::Blank, Led::Orange, Led::Blank])
        .on_success(&Led::Red)
        .on_failure(&Led::Green)
        .start()?;
    thread::sleep(Duration::from_secs(5));
    notification.notify_success()?;

    Ok(())
}
