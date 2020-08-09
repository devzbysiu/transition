use crate::color::Led;
use crate::error::TransitionErr;
use blinkrs::Blinkers;
use blinkrs::Message as BlinkMsg;
use core::fmt::Debug;
use std::clone::Clone;
use std::time::Duration;

pub(crate) trait Message: Send + Sync {
    fn send(&self) -> Result<(), TransitionErr>;
    fn get(&self) -> BlinkMsg;
}

impl Debug for dyn Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "color of msg: {:#?}", self.get())
    }
}

pub(crate) struct ColorMessage {
    blinkers: Blinkers,
    color_msg: BlinkMsg,
}

impl ColorMessage {
    pub(crate) fn new(color: &Led) -> Self {
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        Self {
            color_msg: color_msg(color),
            blinkers,
        }
    }
}

impl Message for ColorMessage {
    fn send(&self) -> Result<(), TransitionErr> {
        self.blinkers.send(self.color_msg)?;
        Ok(())
    }

    fn get(&self) -> BlinkMsg {
        self.color_msg
    }
}

impl Clone for ColorMessage {
    fn clone(&self) -> Self {
        let blinkers: Blinkers =
            Blinkers::new().unwrap_or_else(|_| panic!("Could not find device"));
        Self {
            blinkers,
            color_msg: self.color_msg.clone(),
        }
    }
}

fn color_msg(color: &Led) -> BlinkMsg {
    BlinkMsg::Fade(color.into(), Duration::from_millis(500))
}
