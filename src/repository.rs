use std::{fs, path::PathBuf};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let loc = &PathBuf::from("./tmp/repo1");
        let _ = create(loc);
        if loc.is_dir() && loc.exists() && loc.ends_with("repo1") {
            println!("Successfully created repository");
        } else {
            panic!("expected a directory to be created but was not")
        }
    }
}

pub fn create(loc: &PathBuf) -> Result<bool, String> {
    let r = fs::create_dir_all(loc);
    match r {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}
