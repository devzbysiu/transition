#[cfg(test)]
pub(crate) mod utils {
    use crate::msg::Message;
    use crate::task::Task;
    use anyhow::Result;
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
        fn execute(&self) -> Result<()> {
            self.task_executed.store(true, Ordering::SeqCst);
            Ok(())
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
        fn send(&self) -> Result<()> {
            self.message_sent.store(true, Ordering::SeqCst);
            Ok(())
        }
    }
}
