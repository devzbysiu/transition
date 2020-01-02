use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message;
use std::time::Duration;

pub trait Messg: Send + Sync {
    fn send(&self) -> Result<(), failure::Error>;
}

pub(crate) struct SimpleMessg {
    blinkers: Blinkers,
    color_msg: Message,
}

impl SimpleMessg {
    #[allow(dead_code)]
    pub(crate) fn new<I: Into<String>>(color: I) -> Self {
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        Self {
            color_msg: color_msg(color),
            blinkers,
        }
    }
}

impl Messg for SimpleMessg {
    fn send(&self) -> Result<(), failure::Error> {
        self.blinkers.send(self.color_msg)?;
        Ok(())
    }
}

fn color_msg<I: Into<String>>(color_name: I) -> Message {
    Message::Fade(
        Color::from(color_name.into().as_str()),
        Duration::from_millis(500),
    )
}
