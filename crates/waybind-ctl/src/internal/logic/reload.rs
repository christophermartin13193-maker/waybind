use super::{HealthInterfaceProxy, Step, Verbosity, WaybindError};
use std::path::PathBuf;

pub async fn step(v: Verbosity, path: Option<PathBuf>) -> Result<Step, WaybindError> {
    if v == Verbosity::Debug {
        println!("STEP : RELOAD")
    }

    let connection = zbus::Connection::session().await?;
    let proxy = HealthInterfaceProxy::new(
        &connection,
        "org.waybind.Health",
        "/org/waybind/Daemon/Health",
    )
    .await?;

    match path {
        None => {
            proxy.kill().await?;
            std::process::Command::new("waybind").output()?;
        }
        Some(path) => {
            common::get(&path)?;
            std::process::Command::new("waybind")
                .arg("--config")
                .arg(path)
                .output()?;
        }
    }

    if v != Verbosity::Quiet {
        println!("Done !")
    }

    Ok(Step::Exit)
}
