use crate::app::models::unified_version::LaunchContext;

pub fn replace_jvm_placeholders(value: &str, context: &LaunchContext) -> String {
    value
        .replace("${natives_directory}", &context.natives_directory)
        .replace("${launcher_name}", &context.launcher_name)
        .replace("${launcher_version}", &context.launcher_version)
        .replace("${classpath}", &context.classpath)
}
