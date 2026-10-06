use common::WaybindError;

async fn notification_open() -> Result<bool, WaybindError> {
    let c = zbus::Connection::session().await?;
    let bus = zbus::fdo::DBusProxy::new(&c).await?;

    let has_owner = bus
        .name_has_owner("org.freedesktop.Notifications".try_into()?)
        .await?;
    Ok(has_owner)
}

pub enum Feedback {
    On,
    Off,
}

impl Feedback {
    pub async fn on() -> Self {
        match notification_open().await {
            Err(_) | Ok(false) => Self::Off,
            _ => Self::On,
        }
    }
    pub fn off() -> Self {
        Self::Off
    }

    pub fn send(&self, content: &str) {
        match self {
            Feedback::Off => (),
            Feedback::On => {
                match std::process::Command::new("notify-send")
                    .arg("waybind")
                    .arg(content)
                    .spawn()
                {
                    Ok(_) => (),
                    Err(err) => eprintln!("{err}"),
                }
            }
        }
    }
}
