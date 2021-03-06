// start a new day
extern crate chrono;

mod helper;

mod newday {
    use std::env;
    use std::process::Command;
    use helper;

    /// Append the required text to the log file
    fn exec() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("evertils generate daily")
            .output()
            .expect("failed to execute");

        helper::rolling_log::new_day();

        helper::output::print(&output.stdout);
    }

    pub fn new() {
        let args: Vec<String> = env::args().collect();

        if args.len() > 0 {
            exec();
        } else {
            println!("This command takes no arguments");
        }
    }
}

fn main() {
    newday::new();
}