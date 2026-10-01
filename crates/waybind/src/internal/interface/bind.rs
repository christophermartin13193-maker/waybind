use zbus::interface;
use common::ActionType;
use super::Bind;

#[interface(name = "org.waybind.Bind")]
impl Bind {
    async fn add(&self, bind: String, action_type: ActionType, exec: String) -> String {
        format!("bind: {bind}\taction type: {action_type}\texec: {exec}")
    }

    async fn delete(&self, bind: String) -> String {
        format!("bind: {bind}")
    }

    async fn modify(&self, bind: String, action_type: ActionType, exec: String) -> String {
        format!("bind: {bind}\taction type: {action_type}\texec: {exec}")
    }
}