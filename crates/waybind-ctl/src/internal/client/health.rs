use zbus::proxy;

#[proxy(
    interface = "org.waybind.Health",
    //default_service = "org.waybind.Daemon",
    //default_path = "/org/waybind/Daemon",
)]
pub trait HealthInterface {
    async fn status(&self) -> zbus::Result<String>;
    async fn list(&self) -> zbus::Result<String>;
    async fn get(&self, bind: String) -> zbus::Result<String>;
    async fn kill(&self) -> zbus::Result<()>;
}
