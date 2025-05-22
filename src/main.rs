// Random hyprpaper

use std::fs::read_dir;
use std::error::Error;
use std::io::{self, Write};

use rand::{rng, Rng};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut stdio = io::stdout();

    // Change to specific wallpaper
    if args.len() == 2 {
        let current_path = std::env::current_dir()?;
        stdio.write(
            current_path
                .join(&args[1])
                .to_string_lossy()
                .as_bytes()
        )?;

        return Ok(())
    }

    let wallpapers = wallpaper_files()?;
    let random_nr = rng().random_range(0..wallpapers.len());
    let wallpaper_path = get_path(
        &format!("wallpapers/{}", wallpapers[random_nr])
    )?;

    stdio.write(wallpaper_path.as_bytes())?;

    Ok(())
}

fn get_path(dir: &str) -> Result<String> {
    let user_name = std::env::var("USER")?;
    let path_str = format!(
        "/{}/.config/niri/{}",
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
