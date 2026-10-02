use zbus::interface;
use super::{Health, STANDARD};

#[interface(name = "org.waybind.Health")]
impl Health {
    async fn status(&self) -> String {
        if let Ok(context) = self.context.lock() {
            format!("Context placeholder is equal to {}", context.placeholder)
        }
        else {
            String::from("Couldn't lock placeholder")
        }
    }

    async fn list(&self) -> String {
        if let Ok(context) = self.context.lock() {
            format!("Context placeholder is equal to {}", context.placeholder)
        }
        else {
            String::from("Couldn't lock placeholder")
        }
    }

    async fn get(&self, bind: &str) -> String {
        format!("You asked for the bind: {bind}. This is a placeholder.")
    }

    async fn end(&self) {
        std::process::exit(STANDARD)
    }
}
