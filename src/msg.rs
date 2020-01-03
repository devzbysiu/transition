use anyhow::Result;
use blinkrs::Blinkers;
use blinkrs::Color;
use blinkrs::Message as BlinkMsg;
use std::time::Duration;

pub trait Message: Send + Sync {
    fn send(&self) -> Result<()>;
}

pub(crate) struct Simple {
    blinkers: Blinkers,
    color_msg: BlinkMsg,
}

impl Simple {
    pub(crate) fn new<I: Into<String>>(color: I) -> Self {
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        Self {
            color_msg: color_msg(color),
            blinkers,
        }
    }
}

impl Message for Simple {
    fn send(&self) -> Result<()> {
        self.blinkers.send(self.color_msg)?;
        Ok(())
    }
}

fn color_msg<I: Into<String>>(color_name: I) -> BlinkMsg {
    BlinkMsg::Fade(
        Color::from(color_name.into().as_str()),
        Duration::from_millis(500),
    )
}
