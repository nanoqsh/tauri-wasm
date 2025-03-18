use std::{
    error,
    process::{Command, ExitCode},
};

type Error = Box<dyn error::Error>;

fn main() -> ExitCode {
    match build() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn build() -> Result<(), Error> {
    run(Command::new("cargo").args([
        "b",
        "-p",
        "app",
        "-r",
        "--target",
        "wasm32-unknown-unknown",
    ]))?;

    run(Command::new("wasm-bindgen").args([
        "../target/wasm32-unknown-unknown/release/app.wasm",
        "--out-dir",
        "app-tauri/dist",
        "--target",
        "web",
        "--no-typescript",
    ]))?;

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
        Ok(_) => Err(Error::from(format!("execution of {name} failed"))),
        Err(e) => Err(Error::from(format!("failed to run {name}: {e}"))),
    }
}
