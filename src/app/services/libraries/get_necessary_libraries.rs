use crate::app::models::unified_version::{UnifiedLibrary, UnifiedVersionManifest};

pub fn get_necessary_libraries(manifest: &UnifiedVersionManifest) -> Vec<UnifiedLibrary> {
    let necessary_libraries: Vec<UnifiedLibrary> = manifest
        .libraries
        .iter()
        .filter(|lib| {
            if let Some(rules) = &lib.rules {
                rules.iter().all(|rule| rule.applies_to_current_os())
            } else {
                true
            }
        })
        .cloned()
        .collect();
    necessary_libraries
}
