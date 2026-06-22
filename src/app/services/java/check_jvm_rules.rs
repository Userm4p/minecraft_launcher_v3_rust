use crate::app::models::unified_version::{JvmRule, Name};

pub fn check_jvm_rules(rules: &[JvmRule]) -> bool {
    for rule in rules {
        if let Some(os) = &rule.os.name {
            match os {
                Name::Windows if cfg!(windows) => {
                    return true;
                }

                Name::Linux if cfg!(target_os = "linux") => {
                    return true;
                }

                Name::Osx if cfg!(target_os = "macos") => {
                    return true;
                }

                _ => {}
            }
        }
    }

    false
}
