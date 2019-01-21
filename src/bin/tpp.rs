extern crate tpp;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage:\n\ttpp file");
        return;
    }
    let r = tpp::parse_file(&args[1]);
    if let Ok(v) = r {
        tpp::start(v);
        return;
    }
}
