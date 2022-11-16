fn main() {
    if let Err(e) = rust_uniq::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
