use tokio::{sync::mpsc, task::JoinHandle};

use crate::installer::ui::{
    commands::Command,
    errors::{TuiError, TuiResult},
};

/// Owns the TUI terminal and drives the event loop. Call `run()` to launch
/// the task, then use `sender()` to build a `UiHandle`. Call `wait()` to
/// block the current thread until the TUI exits.
pub struct Tui {
    tx: mpsc::Sender<Command>,
    rx: Option<mpsc::Receiver<Command>>,
    task: Option<JoinHandle<()>>,
}

impl Tui {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);
        Self {
            tx,
            rx: Some(rx),
            task: None,
        }
    }

    /// Returns a sender that a `UiHandle` can use to send commands.
    pub fn sender(&self) -> mpsc::Sender<Command> {
        self.tx.clone()
    }

    /// Spawns the TUI event loop. Must be called before sending any commands.
    pub fn run(&mut self) -> TuiResult<()> {
        let rx = self.rx.take().ok_or(TuiError::AlreadyRunning)?;
        self.task = Some(tokio::spawn(event_loop(rx)));
        Ok(())
    }

    /// Blocks the current thread until the TUI task completes.
    pub fn wait(self) -> TuiResult<()> {
        let task = self.task.ok_or(TuiError::NotStarted)?;
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(task)
                .map_err(|e| TuiError::TaskFailed(e.to_string()))
        })
    }
}

async fn event_loop(mut rx: mpsc::Receiver<Command>) {
    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::InstallForm {
                boot_options,
                reply,
            } => {
                // TODO: render install form (hostname, username, password,
                //       boot mode selection, cloud client)
                let _ = reply.send(todo!("install form not yet implemented"));
            }
            Command::PackageListForm { packages, reply } => {
                // TODO: render interactive package selection list
                let _ = reply.send(todo!("package list form not yet implemented"));
            }
        }
    }
}
