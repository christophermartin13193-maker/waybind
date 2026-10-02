use zbus::proxy;

#[proxy(
    interface = "org.waybind.Bind",
    default_service = "org.waybind.Daemon",
    default_path = "/org/waybind/Daemon",
)]
pub trait BindInterface {
    async fn add(&self) -> zbus::Result<String>;
    async fn modify(&self) -> zbus::Result<String>;
    async fn del(&self, bind: String) -> zbus::Result<String>;
}