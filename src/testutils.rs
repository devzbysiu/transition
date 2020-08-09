#[cfg(test)]
pub(crate) mod utils {
    use crate::error::TransitionErr;
    use crate::msg::Message;
    use crate::task::Task;
    use blinkrs::Message as BlinkMsg;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    pub(crate) struct TaskSpy {
        task_executed: AtomicBool,
    }

    impl TaskSpy {
        pub(crate) fn new() -> Self {
            Self {
                task_executed: AtomicBool::new(false),
            }
        }

        pub(crate) fn executed(&self) -> bool {
            self.task_executed.load(Ordering::SeqCst)
        }
    }

    impl Task for TaskSpy {
        fn execute(&self) -> Result<(), TransitionErr> {
            self.task_executed.store(true, Ordering::SeqCst);
            Ok(())
        }

        fn get(&self) -> &[BlinkMsg] {
            unimplemented!("not needed here")
        }
    }

    pub(crate) struct MessageSpy {
        message_sent: AtomicBool,
    }

    impl MessageSpy {
        pub(crate) fn new() -> Self {
            Self {
                message_sent: AtomicBool::new(false),
            }
        }

        pub(crate) fn msg_sent(&self) -> bool {
            self.message_sent.load(Ordering::SeqCst)
        }
    }

    impl Message for MessageSpy {
        fn send(&self) -> Result<(), TransitionErr> {
            self.message_sent.store(true, Ordering::SeqCst);
            Ok(())
        }

        fn get(&self) -> BlinkMsg {
            unimplemented!("not needed here")
        }
    }

    pub(crate) fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
}
