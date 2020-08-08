use std::error::Error;
use std::thread;
use std::time::Duration;
use transition::Transition;

fn main() -> Result<(), Box<dyn Error>> {
    // start notification
    let notification = Transition::default().start()?;

    // our example code
    thread::sleep(Duration::from_secs(5));

    // task finished with success
    notification.notify_success()?;

    Ok(())
}
