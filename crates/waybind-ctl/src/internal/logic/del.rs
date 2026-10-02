use super::{WaybindError, Step, BindInterfaceProxy};

pub async fn step(bind: String) -> Result<Step, WaybindError> {
    let connection = zbus::Connection::session().await?;
    let proxy = BindInterfaceProxy::new(&connection).await?;

    println!("{}", proxy.del(bind).await?);

    Ok(Step::Exit)
}