use super::{MinimizedArg, Step};
use common::{BindClientProxy, SendMessage, WaybindError};

async fn proxy() -> Result<(zbus::Connection, BindClientProxy<'static>), WaybindError> {
    let c = zbus::Connection::session().await?;
    let proxy = BindClientProxy::new(&c, "org.waybind.Bind", "/org/waybind/Daemon/Bind").await?;
    Ok((c, proxy))
}

pub async fn add<M: SendMessage + Clone + Copy>(
    m: M,
    args: MinimizedArg,
) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : ADD");
    let (_c, p) = proxy().await?;
    m.msg(&format!(
        "{}",
        p.add(args.bind, args.exec, args.action).await?
    ));
    Ok(Step::Exit)
}

pub async fn modify<M: SendMessage + Clone + Copy>(
    m: M,
    args: MinimizedArg,
) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : MOD");
    let (_c, p) = proxy().await?;
    m.msg(&format!(
        "{}",
        p.modify(args.bind, args.exec, args.action).await?
    ));
    Ok(Step::Exit)
}

pub async fn del<M: SendMessage + Clone + Copy>(
    m: M,
    bind: String,
) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : DEL");
    let (_c, p) = proxy().await?;
    m.msg(&format!("{}", p.del(bind).await?));
    Ok(Step::Exit)
}

pub async fn trigger<M: SendMessage + Clone + Copy>(
    m: M,
    bind: String,
) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : TRIGGER");
    let (_c, p) = proxy().await?;
    m.msg(&format!("{}", p.trigger(bind).await?));
    Ok(Step::Exit)
}
