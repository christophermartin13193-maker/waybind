use super::{BindServer, SharedAction, WaybindError};

use zbus::{interface, proxy};

#[proxy(interface = "org.waybind.Bind")]
pub trait BindClient {
    async fn add(&self, bind: String, exec: String, action: SharedAction) -> zbus::Result<String>;
    async fn modify(
        &self,
        bind: String,
        exec: String,
        action: SharedAction,
    ) -> zbus::Result<String>;
    async fn del(&self, bind: String) -> zbus::Result<String>;
    async fn trigger(&self, bind: String) -> zbus::Result<String>;
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
        context.placeholder += 1;

        Ok(format!(
            "bind: {bind}\texec: {exec} : action: {action}\nplaceholder: {}",
            context.placeholder
        ))
    }

    async fn modify(
        &self,
        bind: String,
        exec: String,
        action: SharedAction,
    ) -> Result<String, zbus::fdo::Error> {
        let mut context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;
        context.placeholder += 1;

        Ok(format!(
            "bind: {bind}\texec: {exec} : action: {action}\nplaceholder: {}",
            context.placeholder
        ))
    }

    async fn del(&self, bind: String) -> Result<String, zbus::fdo::Error> {
        let mut context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;
        context.placeholder += 1;

        Ok(format!(
            "bind: {bind}\nplaceholder: {}",
            context.placeholder
        ))
    }

    async fn trigger(&self, bind: String) -> Result<String, zbus::fdo::Error> {
        let mut context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;
        context.placeholder += 1;

        Ok(format!(
            "bind: {bind}\nplaceholder: {}",
            context.placeholder
        ))
    }
}
