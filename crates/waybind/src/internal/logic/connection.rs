use super::{Step, WaybindError, Context, Bind, Health};
use std::sync::{Arc, Mutex};
use common::Config;

pub async fn step(_config: Config) -> Result<Step, WaybindError> {
    println!("Step : Connection");

    let context = Context { placeholder: 10 };
    let sc = Arc::new(Mutex::new(context));

    let health = Health { context: Arc::clone(&sc) };
    let bind = Bind { context: Arc::clone(&sc) };

    let _cnt1 = zbus::connection::Builder::session()?.name("org.waybind.Health")?.serve_at("/org/waybind/Daemon/Health", health)?.build().await?;
    let _cnt1 = zbus::connection::Builder::session()?.name("org.waybind.Bind")?.serve_at("/org/waybind/Daemon/Bind", bind)?.build().await?;

    std::future::pending::<()>().await;

    Ok(Step::Exit)
}