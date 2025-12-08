mod day06;

fn main() {
    if let Err(e) = day06::run() {
        eprintln!("run() failed: {e}");
    }
}
