use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn run(args: &[&str]) -> bool {
    Command::new(program())
        .args(args)
        .status()
        .is_ok_and(|status| status.success())
}

pub fn scoped<'a>(task: &[&'a str], package: Option<&'a str>) -> Vec<&'a str> {
    let mut args = task.to_vec();
    match package {
        Some(name) => args.extend(["-p", name]),
        None => args.push("--workspace"),
    }
    args
}

// rustup's proxy keeps the MSVC toolchain even when another cargo (e.g. a GNU one) leads PATH
fn program() -> PathBuf {
    let home = env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("USERPROFILE").map(|dir| PathBuf::from(dir).join(".cargo")));
    home.map(|dir| {
        dir.join("bin")
            .join(format!("cargo{}", env::consts::EXE_SUFFIX))
    })
    .filter(|path| path.exists())
    .unwrap_or_else(|| env::var_os("CARGO").map_or("cargo".into(), PathBuf::from))
}
