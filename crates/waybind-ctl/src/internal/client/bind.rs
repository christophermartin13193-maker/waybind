use common::ActionType;
use zbus::proxy;

#[proxy(interface = "org.waybind.Bind")]
pub trait BindInterface {
    async fn add(&self, action: ActionType, bind: String, exec: String) -> zbus::Result<String>;
    async fn modify(&self, action: ActionType, bind: String, exec: String) -> zbus::Result<String>;
    async fn del(&self, bind: String) -> zbus::Result<String>;
    async fn trigger(&self, bind: String) -> zbus::Result<String>;
}
