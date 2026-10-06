use super::{STANDARD, WaybindError, entity::HealthServer};
use zbus::{interface, proxy};

#[proxy(interface = "org.waybind.Health")]
pub trait HealthClient {
    async fn status(&self) -> zbus::Result<String>;
    async fn list(&self) -> zbus::Result<String>;
    async fn get(&self, bind: String) -> zbus::Result<String>;
    async fn kill(&self) -> zbus::Result<()>;
}

#[interface(name = "org.waybind.Health")]
impl HealthServer {
    async fn status(&self) -> Result<String, zbus::fdo::Error> {
        let context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;
        Ok(format!("status : {}", context.placeholder))
    }

    async fn list(&self) -> Result<String, zbus::fdo::Error> {
        let context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;
        Ok(format!("list : {}", context.placeholder))
    }

    async fn get(&self, bind: String) -> Result<String, zbus::fdo::Error> {
        let context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;
        Ok(format!("get {} : {}", bind, context.placeholder))
    }

    async fn kill(&self) {
        std::process::exit(STANDARD)
    }
}
