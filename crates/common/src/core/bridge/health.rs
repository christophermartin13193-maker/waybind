use crate::{Feedback, SendMessage, SendNotification};

use super::{ID, STANDARD, WaybindError, entity::HealthServer};
use zbus::{interface, proxy};

#[proxy(interface = "org.waybind.Health")]
pub trait HealthClient {
    async fn status(&self) -> zbus::Result<String>;
    async fn list(&self) -> zbus::Result<String>;
    async fn get(&self, id: ID) -> zbus::Result<String>;
    async fn kill(&self) -> zbus::Result<()>;
}

#[interface(name = "org.waybind.Health")]
impl HealthServer {
    async fn status(&self) -> Result<String, zbus::fdo::Error> {
        let context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;

        context.f.dbg("STEP : WAYBIND HEALTH STATUS");

        let mut number = 0;
        for _ in context.map.keys() {
            number += 1
        }
        let config = context.config.to_str().unwrap_or("Unknown");

        Ok(String::from(format!(
            "Status:\tActive\nConfig:\t{config}\nShortcuts:\t{number} loaded\nWayland:\tConnected (placeholder)\nUptime:\tplaceholder\n"
        )))
    }

    async fn list(&self) -> Result<String, zbus::fdo::Error> {
        let context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;

        context.f.dbg("STEP : WAYBIND HEALTH LIST");

        let mut answer = String::new();
        for (id, shortcut) in context.map.iter() {
            answer.push_str(&format!("{id} : {shortcut}\n"));
        }
        Ok(answer)
    }

    async fn get(&self, id: ID) -> Result<String, zbus::fdo::Error> {
        let context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;

        context.f.dbg("STEP : WAYBIND HEALTH GET");

        match context.map.get(&id) {
            None => Err(zbus::fdo::Error::Failed(
                WaybindError::UnknownID(id).to_string(),
            )),
            Some(shortcut) => Ok(String::from(format!("{id} : {shortcut}"))),
        }
    }

    async fn kill(&self) -> Result<(), zbus::fdo::Error> {
        let context = self
            .context
            .lock()
            .map_err(|e| WaybindError::LockPoisoned(e.to_string()))?;

        context.f.dbg("STEP : WAYBIND HEALTH KILL");
        match context.f.send("Shutdown due to signal sent by waybind-ctl") {
            Ok(_) => (),
            Err(err) => context.f.err(&err.to_string()),
        }

        std::process::exit(STANDARD)
    }
}
