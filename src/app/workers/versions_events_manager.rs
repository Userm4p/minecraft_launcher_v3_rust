use crate::MyApp;

pub fn manage_versions_event(app: &mut MyApp) {
    while let Ok(result) = app.versions_rx.try_recv() {
        match result {
            Ok(minecraft_manifest) => {
                let latest_release = minecraft_manifest.latest.release.clone();
                app.app_state.version_state.minecraft_manifest = minecraft_manifest;
                app.app_state.launcher_settings.selected_version = latest_release;
                app.app_state.version_state.loading = false;
            }

            Err(err) => {
                app.app_state.version_state.loading = false;

                app.app_state.version_state.error =
                    Some(format!("Error al cargar versiones: {}", err));
            }
        }
    }
}
