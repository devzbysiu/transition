use anyhow::Result;
use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message as BlinkMsg;
use std::time::Duration;

pub trait Task: Send + Sync {
    fn execute(&self) -> Result<()>;
}

pub(crate) struct Simple {
    blinkers: Blinkers,
    transition: Vec<BlinkMsg>,
}

impl Simple {
    #[must_use]
    pub fn new<A: AsRef<str>>(colors: &[A]) -> Self {
        let mut transition = Vec::new();
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        for color_name in colors {
            transition.push(BlinkMsg::Fade(
                Color::from(color_name.as_ref()),
                Duration::from_millis(500),
            ));
        }
        Self {
            blinkers,
            transition,
        }
    }

    fn play_transition(&self) {
        for &message in &self.transition {
            self.blinkers.send(message).unwrap();
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}

impl Task for Simple {
    fn execute(&self) -> Result<()> {
        self.play_transition();
        Ok(())
    }
}
