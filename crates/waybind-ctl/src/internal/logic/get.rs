use super::{HealthInterfaceProxy, Step, Verbosity, WaybindError};

pub async fn step(v: Verbosity, bind: String) -> Result<Step, WaybindError> {
    let connection = zbus::Connection::session().await?;
    let proxy = HealthInterfaceProxy::new(
        &connection,
        "org.waybind.Health",
        "org/waybind/Daemon/Health",
    )
    .await?;

    match proxy.get(bind).await {
        Err(err) => Err(WaybindError::ZBusError(err)),
        Ok(answer) => {
            if v != Verbosity::Quiet {
                println!("{answer}")
            }
            Ok(Step::Exit)
        }
    }
}
