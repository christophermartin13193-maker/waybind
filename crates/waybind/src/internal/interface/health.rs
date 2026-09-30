use super::Interface;
use zbus::interface;

use std::collections::HashMap;

#[interface(name = "org.waybind.Health")]
impl Interface {
    async fn status(&self) -> String {
        String::from("You asked for a status")
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
