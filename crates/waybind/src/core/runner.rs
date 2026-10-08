use super::{Args, Step};
use common::{BindServer, Context, Feedback, HealthServer, ID, Shortcut};
use common::{SendMessage, SendNotification, WaybindError, get, first_path, second_path, Config};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn get_default() -> Result<(PathBuf, Config), WaybindError> {
    for path in [first_path(), second_path()] {
        if let Some(path) = path && let Ok(config) = get(&path) {
            return Ok( (path, config) )
        }
    }
    Err(WaybindError::NoDefaultFiles)
}

fn path_and_config(path: Option<PathBuf>, backup: bool) -> Result<(PathBuf, Config), WaybindError> {
    match (path, backup) {
        (None, _) => Ok(get_default()?),
        (Some(path), false) => {
            let  config = get(&path)?;
            Ok( (path, config) )
        },
        (Some(path), true) => match get(&path) {
            Ok(config) => Ok( (path, config) ),
            Err(_) => Ok(get_default()?)
        }

    }
}

pub fn init(f: Feedback, args: Args) -> Result<Step, WaybindError>
{
    f.dbg("STEP : INIT");

    let (path, config) = path_and_config(args.config, args.backup)?;
    let mut map: HashMap<ID, Shortcut> = HashMap::new();
    if let Some(shortcuts) = config.shortcuts {
        for (id, shortcut) in shortcuts.into_iter().enumerate() {
            map.insert(id as ID, shortcut);
        }
    }

    // TODO get the key_map
    // bind shortcuts
    // get compositor
    // get time

    Ok(Step::Connection(f, Context {f, map, config: path}))
}

pub async fn connection(f: Feedback, context: Context) -> Result<Step, WaybindError>
{
    f.dbg("STEP : CONNECTION");

    let sc = Arc::new(Mutex::new(context));

    let health = HealthServer {
        context: Arc::clone(&sc),
    };
    let bind = BindServer {
        context: Arc::clone(&sc),
    };

    let _c1 = zbus::connection::Builder::session()?
        .name("org.waybind.Health")?
        .serve_at("/org/waybind/Daemon/Health", health)?
        .build()
        .await?;

    let _c2 = zbus::connection::Builder::session()?
        .name("org.waybind.Bind")?
        .serve_at("/org/waybind/Daemon/Bind", bind)?
        .build()
        .await?;

    match f.send("Launched successfully") {
        Ok(_) => (),
        Err(err) => f.err(&err.to_string()),
    }
    
    std::future::pending::<()>().await;

    Ok(Step::Exit)
}
