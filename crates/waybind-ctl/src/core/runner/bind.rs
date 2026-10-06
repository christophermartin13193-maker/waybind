use super::{MinimizedArg, Step};
use common::{BindClientProxy, Notifier, SharedAction, WaybindError};

async fn proxy() -> Result<(zbus::Connection, BindClientProxy<'static>), WaybindError> {
    let c = zbus::Connection::session().await?;
    let proxy = BindClientProxy::new(&c, "org.waybind.Bind", "/org/waybind/Daemon/Bind").await?;
    Ok((c, proxy))
}

pub async fn add(n: Notifier, args: MinimizedArg) -> Result<Step, WaybindError> {
    n.dbg("STEP : ADD");
    let (_c, p) = proxy().await?;
    n.msg(&format!(
        "{}",
        p.add(args.bind, args.exec, args.action).await?
    ));
    Ok(Step::Exit)
}

pub async fn modify(n: Notifier, args: MinimizedArg) -> Result<Step, WaybindError> {
    n.dbg("STEP : MOD");
    let (_c, p) = proxy().await?;
    n.msg(&format!(
        "{}",
        p.modify(args.bind, args.exec, args.action).await?
    ));
    Ok(Step::Exit)
}

pub async fn del(n: Notifier, bind: String) -> Result<Step, WaybindError> {
    n.dbg("STEP : DEL");
    let (_c, p) = proxy().await?;
    n.msg(&format!("{}", p.del(bind).await?));
    Ok(Step::Exit)
}

pub async fn trigger(n: Notifier, bind: String) -> Result<Step, WaybindError> {
    n.dbg("STEP : TRIGGER");
    let (_c, p) = proxy().await?;
    n.msg(&format!("{}", p.trigger(bind).await?));
    Ok(Step::Exit)
}
