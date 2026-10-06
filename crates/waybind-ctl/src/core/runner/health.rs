use std::{path::PathBuf, process::ExitStatus};

use super::{Step, check};
use common::{HealthClientProxy, Notifier, WaybindError};

async fn proxy() -> Result<(zbus::Connection, HealthClientProxy<'static>), WaybindError> {
    let c = zbus::Connection::session().await?;
    let proxy =
        HealthClientProxy::new(&c, "org.waybind.Health", "/org/waybind/Daemon/Health").await?;
    Ok((c, proxy))
}

pub async fn status(n: Notifier) -> Result<Step, WaybindError> {
    n.dbg("STEP : STATUS");
    let (_c, proxy) = proxy().await?;
    n.msg(&format!("{}", proxy.status().await?));

    Ok(Step::Exit)
}

pub async fn list(n: Notifier) -> Result<Step, WaybindError> {
    n.dbg("STEP : LIST");
    let (_c, proxy) = proxy().await?;
    n.msg(&format!("{}", proxy.list().await?));

    Ok(Step::Exit)
}

pub async fn reload(n: Notifier, config: Option<PathBuf>) -> Result<Step, WaybindError> {
    n.dbg("STEP : RELOAD");
    check(n, config.clone())?;

    let (_c, p) = proxy().await?;
    p.kill().await?;

    match config {
        None => std::process::Command::new("waybind").output()?,
        Some(path) => std::process::Command::new("waybind").arg("--config").arg(path).output()?,
    };

    Ok(Step::Exit)

}

pub async fn get(n: Notifier, bind: String) -> Result<Step, WaybindError> {
    n.dbg("STEP : GET");
    let (_c, p) = proxy().await?;
    n.msg(&format!("{}", p.get(bind).await?));
    Ok(Step::Exit)
}

pub async fn monitor(n: Notifier) -> Result<Step, WaybindError> {
    n.dbg("STEP : MONITOR");
    n.msg("This function isn't built yet.");
    Ok(Step::Exit)
}
