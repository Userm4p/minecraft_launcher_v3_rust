use tokio::sync::mpsc;

use crate::app::app_state::LauncherEvent;

pub fn send_log(tx: &mpsc::UnboundedSender<LauncherEvent>, message: impl Into<String>) {
    let _ = tx.send(LauncherEvent::Log(message.into()));
}

pub fn send_finished(tx: &mpsc::UnboundedSender<LauncherEvent>) {
    let _ = tx.send(LauncherEvent::Finished);
}
