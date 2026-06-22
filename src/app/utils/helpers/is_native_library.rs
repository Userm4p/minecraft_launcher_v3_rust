use crate::app::models::unified_version::UnifiedLibrary;

pub fn is_native_library(library: &UnifiedLibrary) -> bool {
    library.name.contains("natives")
}
