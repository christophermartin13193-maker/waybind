use std::path::PathBuf;

use super::{Step, check};
use common::{HealthClientProxy, SendMessage, WaybindError};

async fn proxy() -> Result<(zbus::Connection, HealthClientProxy<'static>), WaybindError> {
    let c = zbus::Connection::session().await?;
    let proxy =
        HealthClientProxy::new(&c, "org.waybind.Health", "/org/waybind/Daemon/Health").await?;
    Ok((c, proxy))
}

pub async fn status<M: SendMessage + Clone + Copy>(m: M) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : STATUS");
    let (_c, proxy) = proxy().await?;
    m.msg(&format!("{}", proxy.status().await?));

    Ok(Step::Exit)
}

pub async fn list<M: SendMessage + Clone + Copy>(m: M) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : LIST");
    let (_c, proxy) = proxy().await?;
    m.msg(&format!("{}", proxy.list().await?));

    Ok(Step::Exit)
}

pub async fn reload<M: SendMessage + Clone + Copy>(
    m: M,
    config: Option<PathBuf>,
) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : RELOAD");

    let mut cmd = std::process::Command::new("waybind");

    match &config {
        None => { common::get_default()?;},
        Some(path) => {
            common::get(path)?;
            cmd.arg("--config").arg(path);
        },
    }

    let (_c, p) = proxy().await?;
    match p.kill().await {
        _ => (),
    }

    cmd.output()?;
    Ok(Step::Exit)
}

pub async fn get<M: SendMessage + Clone + Copy>(
    m: M,
    bind: String,
) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : GET");
    let (_c, p) = proxy().await?;
    m.msg(&format!("{}", p.get(bind).await?));
    Ok(Step::Exit)
}

pub async fn monitor<M: SendMessage + Clone + Copy>(m: M) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : MONITOR");
    m.msg("This function isn't built yet.");
    Ok(Step::Exit)
}
