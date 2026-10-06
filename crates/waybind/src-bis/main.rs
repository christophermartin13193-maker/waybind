//use waybind::Interface;
//use zbus;
//
//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let interface = Interface {};
//    let _connection = zbus::connection::Builder::session()?
//        .name("org.waybind.Daemon")?
//        .serve_at("/org/waybind/Daemon", interface)?
//        .build()
//        .await?;
//
//    std::future::pending::<()>().await;
//
//    Ok(())
//}

use waybind::{Automate, Output, STANDARD};

#[tokio::main]
async fn main() {
    let mut automate = Automate::default();
    loop {
        match automate.run().await {
            Ok(Output::Continue) => continue,
            Ok(Output::Exit) => std::process::exit(STANDARD),

            Err(error) => std::process::exit(Automate::handle_error(error)),
        }
    }
}
