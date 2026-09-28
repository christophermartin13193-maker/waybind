use waybind_ctl::{Automate, Output, STANDARD};

fn main() {
    let mut automate = Automate::default();
    loop {
        match automate.run() {
            Ok(Output::Continue) => continue,
            Ok(Output::Exit) => std::process::exit(STANDARD),

            Err(error) => std::process::exit(automate.handle_error(error)),
        }
    }
}
