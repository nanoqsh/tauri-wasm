use std::{
    env, error,
    path::PathBuf,
    process::{Command, ExitCode},
};

type Error = Box<dyn error::Error>;

fn main() -> ExitCode {
    if let Err(e) = build() {
        eprintln!("error: {e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn build() -> Result<(), Error> {
    let root = workspace_root().ok_or("workspace root not found")?;
    let root = root.to_string_lossy();

    run(Command::new("cargo").args([
        "b",
        "-p",
        "app-ui",
        "-r",
        "--target",
        "wasm32-unknown-unknown",
    ]))?;

    let mut wasm_bindgen = Command::new("wasm-bindgen");

    wasm_bindgen.args([
        &format!("{root}/target/wasm32-unknown-unknown/release/app_ui.wasm"),
        "--out-dir",
        &format!("{root}/examples/app-tauri/dist"),
        "--target",
        "web",
        "--no-typescript",
    ]);

    run(&mut wasm_bindgen)?;

    Ok(())
}

fn run(cmd: &mut Command) -> Result<(), Error> {
    let name = cmd
        .get_program()
        .to_str()
        .map(str::to_owned)
        .unwrap_or_default();

    match cmd.status() {
        Ok(s) if s.success() => Ok(()),
        Ok(_) => Err(format!("execution of {name} failed").into()),
        Err(e) => Err(format!("failed to run {name}: {e}").into()),
    }
}

fn workspace_root() -> Option<PathBuf> {
    env::current_dir()
        .ok()?
        .ancestors()
        .find(|p| p.join("Cargo.lock").is_file())
        .map(PathBuf::from)
}
