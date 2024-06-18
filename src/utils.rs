use std::path::Path;
use std::fs::File;
use std::io::Write;
use anyhow::{Result, Error};
use color_print::cprintln;
use reqwest::get;
use uuid::Uuid;
use log::*;

pub fn get_cache_dir_path() -> String { 
    let cache_dir = dirs::cache_dir().unwrap();
    Path::new(&cache_dir).join(".rekognition").to_str().unwrap().to_string()
}

// TODO: For now, I'll roughly implement it. clean it up later
pub async fn retrive_file_from_cache(url: &str) -> Result<String, Error> {
    let cache_dir_path = get_cache_dir_path();
    let allow_extensions = vec!["jpg", "jpeg", "png"];

    let file_extension = Path::new(url);

    if !Path::new(&cache_dir_path).exists() {
        std::fs::create_dir_all(&cache_dir_path).unwrap();
    }

    match file_extension.extension() {
        Some(extension) => {
            if !allow_extensions.contains(&extension.to_str().unwrap()) {
                error!("The file extension is not allowed.");
                return Err(anyhow::anyhow!("The file extension is not allowed."));
            }
        },
        None => {
            error!("The file extension is not allowed.");
            return Err(anyhow::anyhow!("The file extension is not allowed."));
        }
    }

    let unique_id = Uuid::new_v4();
    let file_name = format!("{}.{}", unique_id, file_extension.extension().unwrap().to_str().unwrap());
    
    let file_path = Path::new(&cache_dir_path).join(file_name);
    
    let response = get(url).await.unwrap();
    let content = response.bytes().await.unwrap();
    let mut file = File::create(&file_path).unwrap();
    file.write_all(&content).unwrap();


    Ok(file_path.to_str().unwrap().to_string())

}

pub fn clean_cache_dir() -> Result<(), Error> {
    let cache_dir_path = get_cache_dir_path();
    if Path::new(&cache_dir_path).exists() {
        for file in std::fs::read_dir(&cache_dir_path)? {
            let file_path = file?.path();
            std::fs::remove_file(file_path)?;
        }
        cprintln!("Cache directory path: {}", cache_dir_path);
        cprintln!("✅<green>Cache directory cleaned successfully.</green>");
    }

    let cache_dir_files = std::fs::read_dir(&cache_dir_path)?.count();
    cprintln!("✅<green>Cache directory size: {} files</green>", cache_dir_files);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cache_dir_path() {
        let cache_dir_path = get_cache_dir_path();
        assert!(!cache_dir_path.is_empty());
    }

    #[test]
    fn test_get_cache_dir_path_returns_same_path() {
        let cache_dir_path_1 = get_cache_dir_path();
        let cache_dir_path_2 = get_cache_dir_path();
        assert_eq!(cache_dir_path_1, cache_dir_path_2);
    }

    #[test]
    fn test_get_cache_dir_path_contains_rekognition() {
        let cache_dir_path = get_cache_dir_path();
        assert!(cache_dir_path.contains(".rekognition"));
    }
}