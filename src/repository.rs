pub mod repository {
    use std::fs;

    pub fn create(loc: &str) -> Result<bool, String> {
        if loc.is_empty() {
            return Err(String::from("localtion of repository cannot be empty"));
        }
        let r = fs::create_dir_all(loc);
        match r {
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string()),
        }
    }
}
