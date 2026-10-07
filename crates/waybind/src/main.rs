#[tokio::main]
async fn main() {
    let mut automaton = waybind::Automaton::new().await;
    loop {
        match automaton.run().await {
            Ok(common::Outcome::Continue) => continue,
            Ok(common::Outcome::Exit(code)) => std::process::exit(code),
            Err(err) => std::process::exit(automaton.error(err)),
        }
    }
}
