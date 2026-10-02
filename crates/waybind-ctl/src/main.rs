//use waybind_ctl::{Automate, Output, STANDARD};
//
//fn main() {
//    let mut automate = Automate::default();
//    loop {
//        match automate.run() {
//            Ok(Output::Continue) => continue,
//            Ok(Output::Exit) => std::process::exit(STANDARD),
//
//            Err(error) => std::process::exit(automate.handle_error(error)),
//        }
//    }
//}

//use zbus::proxy;
//
//#[proxy(
//    interface = "org.waybind.Daemon1",
//    default_service = "org.waybind.Daemon",
//    default_path = "/org/waybind/Daemon"
//)]
//trait MyGreeter {
//    async fn say_hello(&self, name: &str) -> zbus::Result<String>;
//}
//
//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    println!("Connexion au bus...");
//    let connection = zbus::Connection::session().await?;
//
//    println!("Création du proxy...");
//    let proxy = MyGreeterProxy::new(&connection).await?;
//
//    println!("Envoi de la requête...");
//    let reply = proxy.say_hello("Chris").await?;
//
//    println!("Réponse reçue : {reply}");
//
//    Ok(())
//}
//
//use zbus::proxy;
//
//#[proxy(
//    interface = "org.waybind.Health",
//    default_service = "org.waybind.Daemon",
//    default_path = "/org/waybind/Daemon"
//)]
//trait HealthInterface {
//    async fn status(&self) -> zbus::Result<String>;
//    async fn list(&self) -> zbus::Result<String>;
//    async fn get(&self, bind: &str) -> zbus::Result<String>;
//}
//
//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let connection = zbus::Connection::session().await?;
//    let proxy = HealthInterfaceProxy::new(&connection).await?;
//
//    println!("status result : {}", proxy.status().await?);
//    println!("list result : {}", proxy.list().await?);
//    println!("get result : {}", proxy.get("Ctrl+C").await?);
//
//    Ok(())
//}

use waybind_ctl::{Automate, STANDARD, Output};

#[tokio::main]
async fn main() {
    let mut automate = Automate::default();
    loop {
        match automate.run().await {
            Ok(Output::Continue) => continue,
            Ok(Output::Exit) => std::process::exit(STANDARD),

            Err(error) => std::process::exit(automate.handle_error(error))
        }
    }
}
