mod day05;

fn main() {
    if let Err(e) = day05::run() {
        eprintln!("run() failed: {e}");
    }
}
