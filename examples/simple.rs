use std::error::Error;
use std::thread;
use std::time::Duration;
use transition::Transition;

fn main() -> Result<(), Box<dyn Error>> {
    // transition finished with success
    let notification = Transition::default().start()?;
    thread::sleep(Duration::from_secs(5));
    notification.notify_success()?;

    thread::sleep(Duration::from_secs(5));

    // transition finished with failure
    let notification = Transition::default().start()?;
    thread::sleep(Duration::from_secs(5));
    notification.notify_failure()?;

    Ok(())
}
