# CopyAnywhere

A small Windows tray app that copies text the moment you select it. Select text in any program and it lands on the clipboard, with a short toast near the cursor and an optional sound. No Ctrl+C needed.

## Download

Grab the latest `CopyAnywhere-x64.exe` (64-bit Windows, the usual choice) or `CopyAnywhere-x86.exe` (32-bit Windows) from the [Releases page](../../releases/latest) and run it. There is no installer: the app starts straight into the system tray and never opens a window on launch.

### Windows warnings

The exe is not code-signed (certificates cost money), so Windows SmartScreen may show "Windows protected your PC". Click "More info", then "Run anyway". Antivirus tools may also flag it, because a tray app with a global mouse hook looks like a keylogger. It is not one: the hook only sees mouse button presses and positions to detect a selection gesture, and it never records keys or sends data anywhere. The full source is in this repository, so you can read it or [build it yourself](#build-from-source). Each release also ships `CopyAnywhere.exe.sha256`; check your download with `certutil -hashfile CopyAnywhere.exe SHA256`.

## How it works

- A global mouse hook watches for a selection gesture: releasing the left button after a drag of at least 4 px, or a double or triple click.
- On a gesture, if auto-copy is on and the foreground program is not excluded, the app sends Ctrl+C and waits up to 300 ms for new text on the clipboard.
- On success it shows a "Copied" toast (about 1.2 s, never takes focus) and plays a sound, if enabled.

### Tray

| Action | Result |
| --- | --- |
| Left click | Toggle auto-copy (green icon = on, red = off) |
| Right click | Menu: Enable/Disable, Settings, Exit |

### Settings

Opened from the tray menu; it closes when it loses focus. Settings are saved to `%APPDATA%\copyanywhere\settings.json` and apply immediately.

- Launch with Windows
- Play sound
- Show toast
- Excluded programs: pick from running programs, or add one by name, and remove entries

`explorer.exe` is excluded by default, because Ctrl+C there copies files, not text.

### Limitations

- Windows only.
- It overwrites the clipboard by design.
- Terminals treat Ctrl+C as an interrupt when nothing is selected. Add them to the exclusion list.

## Built with

- [Rust](https://www.rust-lang.org/) (edition 2024)
- [GPUI](https://www.gpui.rs/) and [gpui-kit](https://crates.io/crates/gpui-kit) for the settings window and toast
- [tray-icon](https://crates.io/crates/tray-icon) for the tray
- [windows](https://crates.io/crates/windows) crate for the mouse hook, clipboard, keyboard input and process info
- [winreg](https://crates.io/crates/winreg) for autostart, [serde](https://serde.rs/) for settings
- [winresource](https://crates.io/crates/winresource) to embed the icon in the exe

The code is a Cargo workspace split into DDD layers:

| Crate | Role |
| --- | --- |
| `crates/domain` | Pure types and rules: settings, exclusions, gestures, copy policy |
| `crates/application` | The auto-copy use case and the ports infrastructure implements |
| `crates/infrastructure` | Windows and filesystem adapters: hook, clipboard, sound, autostart, settings file |
| `crates/ui` | GPUI toast, settings window and tray |
| `crates/app` | Wiring only; produces `copyanywhere.exe` |
| `xtask` | Dev commands (`cargo xtask ...`) |

Design notes live in [`.claude/docs`](.claude/docs): `spec.md`, `plan.md`, `design.md`. Coding rules are in [`code_standards.md`](code_standards.md).

## Build from source

You need Windows 10/11, [rustup](https://rustup.rs/) (stable toolchain with the MSVC target) and the Visual Studio Build Tools with the "Desktop development with C++" workload.

```sh
git clone https://github.com/UeberTimei/copyanywhere.git
cd copyanywhere
cargo xtask run                       # run a debug build
cargo build --release -p copyanywhere # exe at target/release/copyanywhere.exe
```

### Dev commands

```sh
cargo xtask build [crate]   # also: test, check, lint
cargo xtask fmt             # format
cargo xtask ci              # fmt check + clippy -D warnings + all tests
```

`crate` is optional and one of `domain application infrastructure ui copyanywhere`.

## Contributing

Create one branch per feature, use [Conventional Commits](https://www.conventionalcommits.org/), and make sure `cargo xtask ci` passes before opening a pull request.

## Releases

CI (`.github/workflows/ci.yml`) runs on every push to `main` and on pull requests. After the checks pass on `main`, it builds the release exe and publishes a GitHub release tagged `v<version>` if one does not exist yet. To ship a new version, run `cargo xtask bump <major.minor.patch>` (edits `crates/app/Cargo.toml` and `Cargo.lock`), commit, and merge to `main`. Nothing is released until the version is bumped.

## License

MIT, see [LICENSE](LICENSE).
