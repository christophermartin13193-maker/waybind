use super::{BindServer, ID, SharedAction, WaybindError};

use zbus::{interface, proxy};

#[proxy(interface = "org.waybind.Bind")]
pub trait BindClient {
    async fn add(&self, bind: String, exec: String, action: SharedAction) -> zbus::Result<String>;
    async fn modify(&self, id: ID, exec: String, action: SharedAction) -> zbus::Result<String>;
    async fn del(&self, id: ID) -> zbus::Result<String>;
    async fn trigger(&self, id: ID) -> zbus::Result<String>;
}

#[interface(name = "org.waybind.Bind")]
impl BindServer {
    async fn add(
        &self,
        bind: String,
        exec: String,
        action: SharedAction,
    ) -> Result<String, zbus::fdo::Error> {
        let mut context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;

        Ok(String::from(""))
    }

    async fn modify(
        &self,
        id: ID,
        exec: String,
        action: SharedAction,
    ) -> Result<String, zbus::fdo::Error> {
        let mut context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;
        Ok(String::from(""))
    }

    async fn del(&self, id: ID) -> Result<String, zbus::fdo::Error> {
        let mut context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;
        Ok(String::from(""))
    }

    async fn trigger(&self, id: ID) -> Result<String, zbus::fdo::Error> {
        let mut context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;
        Ok(String::from(""))
    }
}
