use std::{
    io::{self, Write},
    path::PathBuf,
    process::exit,
    fs::File
};
use dirs::home_dir;

#[allow(dead_code)]
fn print_title() {
    println!("Welcome to Background Changer (bgc)");
    println!("v{}", env!("CARGO_PKG_VERSION"));

    println!("\nYou can move up and down using 'j' and 'k'");
    println!("Press 'enter' to select a wallpaper or 'i' to preview the image\n");
}

fn get_img(file_path: PathBuf) -> bool {
    let img_files = ["jpeg", "png", "gif", "pnm", "tga", "ttf", "webp", "bmp", "farb", "farbfeld"];

    let a = file_path.extension();

    if img_files.contains(&a.unwrap().to_str().unwrap()) {
        return true; 
    }

    false
}

fn main() {
    let conf_path: PathBuf = home_dir().expect("Error getting home dir")
                                .join(".config/bgc/config.conf");

    if !conf_path.exists() {
        let mut w_input: String = String::new();

        println!("Enter the path of the wallpapers");

        io::stdin().read_line(&mut w_input)
            .expect("Error reading path.");

        let w_path: PathBuf = PathBuf::from(w_input.trim());
        
        if !w_path.exists() || !w_path.is_dir() {
             println!("The path doesnt exist or its not a directory");
             exit(1);
        }

        let mut c_file = File::create(&conf_path)
            .expect("Error creating file");

        let text: String = String::from("wallpaper_path = ") + w_path.to_str().unwrap();

        c_file.write_all(text.as_bytes())
            .expect("Error writing to file");

        c_file.flush()
            .expect("Error flushing file");
    } 
}
