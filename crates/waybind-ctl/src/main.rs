use common::Outcome;
use waybind_ctl::Automaton;

#[tokio::main]
async fn main() {
    let mut automaton = Automaton::default();
    loop {
        match automaton.run().await {
            Ok(Outcome::Continue) => continue,
            Ok(Outcome::Exit(code)) => std::process::exit(code),
            Err(err) => std::process::exit(automaton.error(err)),
        }
    }
}
