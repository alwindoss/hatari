fn main() {
    match hatari::run() {
        Ok(_) => {
            println!("exiting successfully...");
        }
        Err(e) => {
            println!("Found an error: {}", e);
        }
    }
}
