use std::env;
use std::process::Command;

pub fn run(args: &[&str]) -> bool {
    let program = env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    Command::new(program)
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
