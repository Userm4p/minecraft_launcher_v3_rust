use crate::app::models::unified_version::{
    Action, GameElement, GameLaunchContext, GameRule, UnifiedVersionManifest, Value,
};

pub fn build_game_arguments(
    manifest: &UnifiedVersionManifest,
    context: &GameLaunchContext,
) -> Vec<String> {
    let mut args = Vec::new();

    for arg in &manifest.arguments.game {
        match arg {
            GameElement::String(value) => {
                let replaced = replace_game_placeholders(value, context);

                if !contains_unresolved_placeholder(&replaced) {
                    args.push(replaced);
                }
            }

            GameElement::GameClass(game_class) => {
                if !check_game_rules(&game_class.rules) {
                    continue;
                }

                match &game_class.value {
                    Value::String(value) => {
                        let replaced = replace_game_placeholders(value, context);

                        if !contains_unresolved_placeholder(&replaced) {
                            args.push(replaced);
                        }
                    }

                    Value::StringArray(values) => {
                        for value in values {
                            let replaced = replace_game_placeholders(value, context);

                            if !contains_unresolved_placeholder(&replaced) {
                                args.push(replaced);
                            }
                        }
                    }
                }
            }
        }
    }

    args
}

fn replace_game_placeholders(value: &str, context: &GameLaunchContext) -> String {
    value
        .replace("${auth_player_name}", &context.username)
        .replace("${version_name}", &context.version_name)
        .replace("${game_directory}", &context.game_directory)
        .replace("${assets_root}", &context.assets_root)
        .replace("${assets_index_name}", &context.assets_index_name)
        .replace("${auth_uuid}", "offline-uuid")
        .replace("${auth_access_token}", "offline-token")
        .replace("${user_type}", "offline")
        .replace("${version_type}", "release")
}

fn check_game_rules(rules: &[GameRule]) -> bool {
    if rules.is_empty() {
        return true;
    }

    let mut allowed = false;

    for rule in rules {
        match rule.action {
            Action::Allow => {
                allowed = true;
            }
        }

        if let Some(is_demo_user) = rule.features.is_demo_user {
            if is_demo_user {
                return false;
            }
        }
    }

    allowed
}

fn contains_unresolved_placeholder(value: &str) -> bool {
    value.contains("${")
}
