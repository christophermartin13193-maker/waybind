use super::{MinimizedAdd, MinimizedMod, Step};
use common::{BindClientProxy, ID, SendMessage, WaybindError};

async fn proxy() -> Result<(zbus::Connection, BindClientProxy<'static>), WaybindError> {
    let c = zbus::Connection::session().await?;
    let proxy = BindClientProxy::new(&c, "org.waybind.Bind", "/org/waybind/Daemon/Bind").await?;
    Ok((c, proxy))
}

pub async fn add<M: SendMessage + Clone + Copy>(
    m: M,
    args: MinimizedAdd,
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
    args: MinimizedMod,
) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : MOD");
    let (_c, p) = proxy().await?;
    m.msg(&format!(
        "{}",
        p.modify(args.id, args.exec, args.action).await?
    ));
    Ok(Step::Exit)
}

pub async fn del<M: SendMessage + Clone + Copy>(m: M, id: ID) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : DEL");
    let (_c, p) = proxy().await?;
    m.msg(&format!("{}", p.del(id).await?));
    Ok(Step::Exit)
}

pub async fn trigger<M: SendMessage + Clone + Copy>(m: M, id: ID) -> Result<Step<M>, WaybindError> {
    m.dbg("STEP : TRIGGER");
    let (_c, p) = proxy().await?;
    m.msg(&format!("{}", p.trigger(id).await?));
    Ok(Step::Exit)
}
