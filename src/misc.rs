use serde::de::DeserializeOwned;
use std::fs;

pub fn load_from_file<T>(file_name: &str) -> T
where
    T: DeserializeOwned + Default,
{
    match fs::read_to_string(file_name) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(data) => {
                println!("[info]: Loaded {} successfully.", file_name);
                data
            }
            Err(e) => {
                eprintln!("[err]: {} -> {}. Using default value.", file_name, e);
                T::default()
            }
        },
        Err(_) => {
            println!(
                "[warn]: File could not be found: {}. Starting with empty data.",
                file_name
            );
            T::default()
        }
    }
}
