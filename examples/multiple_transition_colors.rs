use std::error::Error;
use std::thread;
use std::time::Duration;
use transition::{Led, Transition};

fn main() -> Result<(), Box<dyn Error>> {
    let notification = Transition::new(&[
        Led::Cyan,
        Led::Blank,
        Led::Yellow,
        Led::Blank,
        Led::White,
        Led::Blank,
        Led::Orange,
        Led::Blank,
    ])?
    .start()?;

    thread::sleep(Duration::from_secs(5));

    notification.notify_success()?;

    Ok(())
}
