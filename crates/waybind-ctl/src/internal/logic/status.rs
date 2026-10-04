use super::{HealthInterfaceProxy, Step, Verbosity, WaybindError};

pub async fn step(v: Verbosity) -> Result<Step, WaybindError> {
    if v == Verbosity::Debug {
        println!("STEP : STATUS")
    }

    let connection = zbus::Connection::session().await?;
    let proxy = HealthInterfaceProxy::new(
        &connection,
        "org.waybind.Health",
        "/org/waybind/Daemon/Health",
    )
    .await?;

    if v != Verbosity::Quiet {
        println!("{}", proxy.status().await?);
    } else {
        proxy.status().await?;
    }

    Ok(Step::Exit)
}
