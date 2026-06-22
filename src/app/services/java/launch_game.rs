use anyhow::Result;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::app::{app_state::LauncherEvent, utils::helpers::send_log};

pub fn launch_minecraft(
    java_path: &str,
    jvm_args: Vec<String>,
    main_class: &str,
    game_args: Vec<String>,
    tx: &UnboundedSender<LauncherEvent>,
) -> Result<()> {
    let mut child = Command::new(java_path)
        .args(&jvm_args)
        .arg(main_class)
        .args(&game_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stderr) = child.stderr.take() {
        let tx_copy = tx.clone();
        std::thread::spawn(move || {
            let reader = BufReader::new(stderr);

            for line in reader.lines().flatten() {
                send_log(&tx_copy, format!("[MC ERROR] {}", line));
                let _ = tx_copy.send(LauncherEvent::Finished);
            }
        });
    }

    if let Some(stdout) = child.stdout.take() {
        let tx_copy = tx.clone();
        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);

            for line in reader.lines().flatten() {
                send_log(&tx_copy, format!("[MC] {}", line));
                let _ = tx_copy.send(LauncherEvent::Finished);
            }
        });
    }

    Ok(())
}
