use super::{BindInterfaceProxy, Step, Verbosity, WaybindError};

pub async fn step(v: Verbosity, bind: String) -> Result<Step, WaybindError> {
    if v == Verbosity::Debug {
        println!("STEP : TRIGGER")
    }

    let connection = zbus::Connection::session().await?;
    let proxy =
        BindInterfaceProxy::new(&connection, "org.waybind.Bind", "org/waybind/Daemon/Bind").await?;

    match proxy.trigger(bind).await {
        Err(err) => Err(WaybindError::ZBusError(err)),
        Ok(answer) => {
            if v != Verbosity::Quiet {
                println!("{answer}")
            }
            Ok(Step::Exit)
        }
    }
}
