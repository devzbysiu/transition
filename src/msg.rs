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

#[derive(Debug)]
pub(crate) struct ColorMessage {
    blinkers: Blinkers,
    color_msg: BlinkMsg,
}

impl ColorMessage {
    pub(crate) fn new(color: &Led) -> Self {
        let blinkers: Blinkers = Blinkers::new().expect("could not find device");
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
        let blinkers: Blinkers = Blinkers::new().expect("could not find device");
        Self {
            blinkers,
            color_msg: self.color_msg,
        }
    }
}

fn color_msg(color: &Led) -> BlinkMsg {
    BlinkMsg::Fade(color.into(), Duration::from_millis(500), None)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_debug_formatting() {
        let msg: Box<dyn Message> = Box::new(ColorMessage::new(&Led::White));
        let result = format!("{:?}", msg);
        assert_eq!(result, "color of msg: Fade(\n    Three(\n        255,\n        255,\n        255,\n    ),\n    500ms,\n    None,\n)");
    }
}
