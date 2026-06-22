use crate::{MyApp, app::app_state::LauncherEvent};

pub fn launch_events_manager(app: &mut MyApp) {
    while let Ok(event) = app.launcher_rx.try_recv() {
        match event {
            LauncherEvent::Started => {
                app.app_state.ui_state.launch_in_progress = true;
                app.app_state.ui_state.logs.push("Launch started".into());
            }

            LauncherEvent::Log(log) => {
                app.app_state.ui_state.logs.push(log);
            }

            LauncherEvent::Progress(progress) => {
                app.app_state.ui_state.download_progress =
                    app.app_state.ui_state.download_progress + progress;
            }

            LauncherEvent::Error(err) => {
                app.app_state.ui_state.error_message = Some(err);
                app.app_state.ui_state.launch_in_progress = false;
            }

            LauncherEvent::Finished => {
                app.app_state.ui_state.launch_in_progress = false;
                app.app_state.ui_state.logs.push("Launch completed".into());
            }
        }
    }
}
