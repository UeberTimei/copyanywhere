mod cargo;

use std::process::ExitCode;

const CRATES: [&str; 5] = [
    "domain",
    "application",
    "infrastructure",
    "ui",
    "copyanywhere",
];
const USAGE: &str = "usage: cargo xtask <build|test|check|lint|fmt|run|ci> [crate]\ncrates: domain application infrastructure ui copyanywhere (omit for all)";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let task = args.first().map(String::as_str);
    let package = args.get(1).map(String::as_str);
    if package.is_some_and(|name| !CRATES.contains(&name)) {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    }
    let passed = match task {
        Some("build") => cargo::run(&cargo::scoped(&["build"], package)),
        Some("test") => cargo::run(&cargo::scoped(&["test"], package)),
        Some("check") => cargo::run(&cargo::scoped(&["check"], package)),
        Some("lint") => lint(package),
        Some("fmt") => cargo::run(&["fmt", "--all"]),
        Some("run") => cargo::run(&["run", "-p", "copyanywhere"]),
        Some("ci") => ci(),
        _ => {
            eprintln!("{USAGE}");
            false
        }
    };
    if passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn ci() -> bool {
    cargo::run(&["fmt", "--all", "--check"])
        && cargo::run(&[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ])
        && cargo::run(&["test", "--workspace"])
}

fn lint(package: Option<&str>) -> bool {
    let mut args = cargo::scoped(&["clippy", "--all-targets"], package);
    args.extend(["--", "-D", "warnings"]);
    cargo::run(&args)
}
