use std::process::Command;

pub fn get_java_version() -> Option<String> {
    let output = Command::new("java").arg("-version").output().ok()?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    stderr.lines().next().map(|line| line.to_string())
}

fn parse_java_version(output: &str) -> Option<String> {
    output
        .lines()
        .find(|l| l.contains("version"))
        .and_then(|l| l.split('"').nth(1).map(|v| v.to_string()))
}

fn get_major_version(version: &str) -> Option<u32> {
    version.split('.').next()?.parse::<u32>().ok()
}

pub fn get_java_version_number() -> Option<u32> {
    let java_version = get_java_version()?;
    let parsed_java_version = parse_java_version(&java_version)?;
    let java_version_number = get_major_version(&parsed_java_version)?;
    Some(java_version_number)
}
