use crate::app::{
    models::unified_version::{JvmElement, LaunchContext, UnifiedVersionManifest, Value},
    services::java::{
        check_jvm_rules::check_jvm_rules, placeholder_replacement::replace_jvm_placeholders,
    },
};

pub fn build_jvm_arguments(
    manifest: &UnifiedVersionManifest,
    context: &LaunchContext,
) -> Vec<String> {
    let mut args = Vec::new();

    args.push(format!("-Xmx{}M", context.ram_mb));

    for arg in &manifest.arguments.jvm {
        match arg {
            JvmElement::String(value) => {
                args.push(replace_jvm_placeholders(value, context));
            }

            JvmElement::JvmClass(rule_arg) => {
                if !check_jvm_rules(&rule_arg.rules) {
                    continue;
                }

                match &rule_arg.value {
                    Value::String(value) => {
                        args.push(replace_jvm_placeholders(value, context));
                    }

                    Value::StringArray(values) => {
                        for value in values {
                            args.push(replace_jvm_placeholders(value, context));
                        }
                    }
                }
            }
        }
    }

    args
}
