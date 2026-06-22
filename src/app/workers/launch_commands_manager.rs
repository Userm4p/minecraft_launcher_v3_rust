use tokio::sync::mpsc;

use crate::app::{
    app_state::{LauncherCommand, LauncherEvent},
    services::handle_launch,
};

pub async fn launch_commands_manager(
    launcher_command_rx: &mut mpsc::UnboundedReceiver<LauncherCommand>,
    launcher_event_tx: mpsc::UnboundedSender<LauncherEvent>,
) {
    while let Some(command) = launcher_command_rx.recv().await {
        match command {
            LauncherCommand::Launch {
                version_url,
                username,
                ram_allocation,
                installation_path,
            } => {
                let _ = launcher_event_tx.send(LauncherEvent::Started);
                let _ = launcher_event_tx.send(LauncherEvent::Log(format!(
                    "Launching {} version...",
                    version_url
                )));
                match handle_launch(
                    &installation_path,
                    &version_url,
                    &launcher_event_tx,
                    &username,
                    ram_allocation,
                )
                .await
                {
                    Ok(_) => {
                        let _ = launcher_event_tx.send(LauncherEvent::Log(
                            "Launch process completed successfully.".into(),
                        ));
                    }

                    Err(err) => {
                        print!("Error during launch process: {}", err);
                        let _ = launcher_event_tx.send(LauncherEvent::Error(err.to_string()));
                    }
                }
            }
        }
    }
}
