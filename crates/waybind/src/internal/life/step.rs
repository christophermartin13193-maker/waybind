use common::Config;

pub enum Step {
    GetConfig,
    GetKeyMap(Config),
    Connection(Config),
    Exit,
}