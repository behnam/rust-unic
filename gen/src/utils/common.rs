use std::process::Command;
use std::path::Path;
use std::{fs, io};

pub fn cleanup_data<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    fs::remove_dir_all(&path)?;
    fs::create_dir(&path)?;
    Ok(())
}

pub fn cleanup_rsv<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if path.as_ref().exists() {
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy() == "rsv" {
                    fs::remove_file(&path)?;
                }
            }
        }
    }

    Ok(())
}

pub fn fetch<S, P>(url: S, destination: P)
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    let (url, destination) = (url.as_ref(), destination.as_ref());
    let curl_exit = Command::new("curl")
        .arg("-o")
        .arg(url)
        .arg(destination.to_str().unwrap())
        .spawn()
        .expect("Failed to launch curl")
        .wait()
        .expect("Failed to await curl");
    if !curl_exit.success() {
        panic!("curl failed with exit code {} for url {}", curl_exit, url)
    }
}

fn capitalize(str: &str) -> String {
    let mut chars = str.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn title_case(str: &str) -> String {
    str.split_whitespace()
        .map(capitalize)
        .fold(String::new(), |acc, ref word| acc + word)
}
