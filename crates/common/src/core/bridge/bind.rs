use crate::{SendMessage, Shortcut};
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

        context.f.dbg("STEP: WAYBIND BIND ADD");

        let action = action.action();
        let id = context.stack.pop().unwrap_or(context.map.len() as ID);
        let shortcut = Shortcut {bind, action, exec};
        let answer = format!("{id} : shortcut");

        context.map.insert(id, shortcut);

        Ok(answer)
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

        context.f.dbg("STEP : WAYBIND BIND MODIFY");

        if !context.map.contains_key(&id) {
            return Err(zbus::fdo::Error::Failed(WaybindError::UnknownID(id).to_string()))
        }

        let mut answer = format!("modified {id} : ");
        context.map.entry(id).and_modify(|s|{
            s.action = action.action();
            s.exec = exec;
            answer.push_str(&s.to_string());
        });

        Ok(answer)
    }

    async fn del(&self, id: ID) -> Result<String, zbus::fdo::Error> {
        let mut context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;

        context.f.dbg("STEP : WAYBIND BIND DEL");

        match context.map.remove(&id) {
            None => Err(zbus::fdo::Error::Failed(WaybindError::UnknownID(id).to_string())),
            Some(shortcut) => Ok(format!("removed {id} : {shortcut}")),
        }
    }

    async fn trigger(&self, id: ID) -> Result<String, zbus::fdo::Error> {
        let context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;

        context.f.dbg("STEP : WAYBIND BIND TRIGGER");

        Ok(format!("{id} placeholder"))
    }
}
