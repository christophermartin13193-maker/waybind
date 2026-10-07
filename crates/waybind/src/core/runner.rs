use super::{Args, Step};
use common::{BindServer, Context, HealthServer};
use common::{SendMessage, SendNotification, WaybindError};
use std::sync::{Arc, Mutex};

pub fn init<F>(f: F, args: Args) -> Result<Step<F>, WaybindError>
where
    F: SendMessage + SendNotification + Clone + Copy,
{
    f.dbg("STEP : INIT");
    // TODO get the key_map
    // TODO create list of shortcuts

    Ok(Step::Connection(f))
}

pub async fn connection<F>(f: F) -> Result<Step<F>, WaybindError>
where
    F: SendMessage + SendNotification + Clone + Copy,
{
    f.dbg("STEP : CONNECTION");

    let context = Context { placeholder: 10 };
    let sc = Arc::new(Mutex::new(context));

    let health = HealthServer {
        context: Arc::clone(&sc),
    };
    let bind = BindServer {
        context: Arc::clone(&sc),
    };

    let _c1 = zbus::connection::Builder::session()?
        .name("org.waybind.Health")?
        .serve_at("/org/waybind/Daemon/Health", health)?
        .build()
        .await?;

    let _c2 = zbus::connection::Builder::session()?
        .name("org.waybind.Bind")?
        .serve_at("/org/waybind/Daemon/Bind", bind)?
        .build()
        .await?;

    std::future::pending::<()>().await;

    Ok(Step::Exit)
}
