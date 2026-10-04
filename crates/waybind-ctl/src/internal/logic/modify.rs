use super::{BindAction, BindInterfaceProxy, Step, Verbosity, WaybindError};
use common::ActionType;

pub async fn step(v: Verbosity, action: BindAction) -> Result<Step, WaybindError> {
    if v == Verbosity::Debug {
        println!("STEP : MOD")
    }

    let bind = action.bind;
    let exec = action.exec;

    let action = match (action.hold, action.released) {
        (Some(ms), _) => ActionType::Hold(ms),
        (_, Some(ms)) => ActionType::Released(ms),
        _ => ActionType::Pressed(0),
    };

    let connection = zbus::Connection::session().await?;
    let proxy =
        BindInterfaceProxy::new(&connection, "org.waybind.Bind", "/org/waybind/Daemon/Bind")
            .await?;

    if v != Verbosity::Quiet {
        println!("{}", proxy.modify(action, bind, exec).await?)
    } else {
        proxy.modify(action, bind, exec).await?;
    }

    Ok(Step::Exit)
}
