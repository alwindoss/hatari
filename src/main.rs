fn main() {
    match hatari::run() {
        Ok(_) => println!("Hatari success!!!"),
        Err(_) => println!("Hatari failed :("),
    }
}
