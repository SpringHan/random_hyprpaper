// Random hyprpaper

use std::fs::{read_dir, OpenOptions};
use std::error::Error;
use std::io::Write;

use rand::{rng, Rng};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let wallpapers = wallpaper_files()?;
    let random_nr = rng().random_range(0..wallpapers.len());
    
    modify_conf(wallpapers[random_nr].to_owned())?;

    Ok(())
}

fn get_path(dir: &str) -> Result<String> {
    let user_name = std::env::var("USER")?;
    let path_str = format!(
        "/{}/.config/hypr/{}",
        if &user_name == "root" {
            String::from("root")
        } else {
            format!("home/{}", user_name)
        },
        dir
    );

    Ok(path_str)
}

fn wallpaper_files() -> Result<Vec<String>> {
    let wallpaper_dir = read_dir(get_path("wallpapers/")?)?;
    let wallpapers = wallpaper_dir
        .filter_map(|entry| {
            let file = entry.unwrap()
                .file_name()
                .into_string()
                .unwrap();

            if &file != "." && &file != ".." {
                Some(file)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(wallpapers)
}

fn modify_conf(wallpaper: String) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(get_path("hyprpaper.conf")?)?;

    let full_path = get_path(&format!("wallpapers/{}", wallpaper))?;

    let new_conf = format!(
        "preload = {}\nwallpaper = ,{}\nsplash = false",
        full_path,
        full_path
    );

    file.write(new_conf.as_bytes())?;

    Ok(())
}
