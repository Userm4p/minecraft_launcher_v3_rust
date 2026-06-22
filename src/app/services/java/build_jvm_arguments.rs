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
                let arg = replace_jvm_placeholders(value, context);
                if is_runtime_compatible(&arg, context.java_version) {
                    args.push(arg);
                }
            }

            JvmElement::JvmClass(rule_arg) => {
                if !check_jvm_rules(&rule_arg.rules) {
                    continue;
                }

                match &rule_arg.value {
                    Value::String(value) => {
                        let arg = replace_jvm_placeholders(value, context);
                        if is_runtime_compatible(&arg, context.java_version) {
                            args.push(arg);
                        }
                    }

                    Value::StringArray(values) => {
                        for value in values {
                            let arg = replace_jvm_placeholders(value, context);
                            if is_runtime_compatible(&arg, context.java_version) {
                                args.push(arg);
                            }
                        }
                    }
                }
            }
        }
    }

    args
}

fn is_runtime_compatible(arg: &str, java_major: u32) -> bool {
    match arg {
        "--sun-misc-unsafe-memory-access=allow" => false,

        x if x.starts_with("-XX:+UseConcMarkSweepGC") && java_major >= 14 => false,

        _ => true,
    }
}
