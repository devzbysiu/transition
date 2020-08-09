use crate::color::Led;
use crate::error::TransitionErr;
use blinkrs::Blinkers;
use blinkrs::Message as BlinkMsg;
use std::fmt::Debug;
use std::time::Duration;

pub(crate) trait Task: Send + Sync {
    fn execute(&self) -> Result<(), TransitionErr>;
    fn get(&self) -> &[BlinkMsg];
}

impl Debug for dyn Task {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "task colors: {:#?}", self.get())
    }
}

pub(crate) struct BlinkTask {
    blinkers: Blinkers,
    transition: Vec<BlinkMsg>,
}

impl BlinkTask {
    #[must_use]
    pub fn new(colors: &[Led]) -> Self {
        let mut transition = Vec::new();
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        for color in colors {
            transition.push(BlinkMsg::Fade(color.into(), Duration::from_millis(500)));
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

impl Task for BlinkTask {
    fn execute(&self) -> Result<(), TransitionErr> {
        self.play_transition();
        Ok(())
    }

    fn get(&self) -> &[BlinkMsg] {
        &self.transition
    }
}
