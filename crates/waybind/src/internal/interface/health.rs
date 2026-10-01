use zbus::interface;

use std::collections::HashMap;
use super::Health;

#[interface(name = "org.waybind.Health")]
impl Health {
    async fn status(&self) -> String {
        format!("Context placeholder is equal to {}", self.context.placeholder)
    }

    async fn list(&self) -> String {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("Super+A"), String::from("/usr/bin/foo"));
        map.insert(String::from("Super+Ctrl+B"), String::from("/usr/bin/bar"));

        let mut answer = String::new();

        for (bind, command) in &map {
            answer.push_str(&format!("bind: {bind} -> command: {command}"));
        }

        answer
    }

    async fn get(&self, bind: &str) -> String {
        format!("You asked for the bind: {bind}. This is a placeholder.")
    }
}
