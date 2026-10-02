use super::{WaybindError, Step, HealthInterfaceProxy };

pub async fn step() -> Result<Step, WaybindError> {
    let connection = zbus::Connection::session().await?;
    let proxy = HealthInterfaceProxy::new(&connection).await?;

    println!("{}", proxy.status().await?);

    Ok(Step::Exit)
}