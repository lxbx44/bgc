use std::{
    io::{
        stdin, 
        Write
    },
    path::PathBuf,
    process::{
        exit,
        Command
    },
    fs::{
        self, 
        File
    },
};

use terminal_menu::{
    menu, 
    button, 
    run, 
    mut_menu, 
    label
};

use clap::Parser;

use dirs::config_dir;
use core::panic;


/// A program written in Rust to change wallpapers in wayland using swww
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Flag to execute to get last wallpaper set
    #[arg(short, long, default_value_t = false)]
    set: bool
}


fn is_img(file_path: &PathBuf) -> bool {
    let img_files: Vec<&str> = vec!["jpeg", "jpg", "png", "gif", "pnm", "tga", "ttf", "webp", "bmp", "farb", "farbfeld"];

    if let Some(extension) = file_path.extension() {
        if let Some(ext_str) = extension.to_str() {
            return img_files.contains(&ext_str);
        }
    }
    false
}

fn main() {
    let args = Args::parse();

    let mut conf_path: PathBuf = PathBuf::new();

    if let Some(pth) = config_dir() {
        conf_path = pth
            .join("bgc/config.conf");
    }

    if args.set {
        let cfc: String = fs::read_to_string(conf_path.clone())
            .expect("Error reading to string");
        let mut wall_path: PathBuf = PathBuf::new();

        cfc.lines()
            .filter(|x| x.trim().starts_with("prev_wallpaper"))
            .for_each(|line| {
                match line.split('=').last() {
                    Some(wallpaper_path_s) => {
                        wall_path = PathBuf::from(wallpaper_path_s.trim());
                    },
                    None => panic!("No wallpaper_path on config file"),
                };
            });


        match Command::new("swww")
                .arg("init")
                .spawn() {
                    Ok(child) => child,
                    Err(err) => panic!("Swww not installed\n{}", err),
                };

        match Command::new("swww")
            .arg("img")
            .arg(&wall_path)
            .arg("--transition-step")
            .arg("30")
            .arg("--transition-fps")
            .arg("60")
            .spawn() {
                Ok(child) => child,
                Err(err) => panic!("Swww not installed\n{}", err),
            };

        exit(0);
    }

    if !conf_path.exists() {
        let mut w_input: String = String::new();

        println!("Enter the absolute path of the wallpapers");

        stdin().read_line(&mut w_input)
            .expect("Error getting input.");

        let w_path: PathBuf = PathBuf::from(w_input.trim());
        
        if !w_path.exists() || !w_path.is_dir() {
             panic!("The path doesnt exist or its not a directory");
        }

        match fs::DirBuilder::new()
            .recursive(true)
            .create(config_dir().expect("Error getting config directory").join("bgc/")) {
                Ok(()) => (),
                Err(error) => panic!("Error creating directory: {}", error),
            };


        let mut c_file: File = match File::create(&conf_path) {
            Ok(file) => file,
            Err(err) => panic!("Error creating config file\n{}", err),
        };

        let text: String = String::from("wallpaper_path = ") + w_path.to_str()
                                                                .expect("Error converting to str");

        match c_file.write_all(text.as_bytes()) {
            Ok(()) => (),
            Err(err) => panic!("Error writing to file\n{}", err),
        };

        match c_file.flush() {
            Ok(()) => (),
            Err(err) => panic!("Error flushing file\n{}", err),
        };

        match Command::new("swww")
            .arg("init")
            .spawn() {
                Ok(child) => child,
                Err(err) => panic!("Swww not installed\n{}", err),
            };


        match clearscreen::clear() {
            Ok(()) => (),
            Err(e) => eprintln!("Error cleaning screen\n{}", e),
        };
    } 

    let config_file_contents: String = fs::read_to_string(conf_path.clone())
                                            .unwrap();
    
    let mut wallpapers_path: PathBuf = PathBuf::new();
    let mut other_wall: bool = false;

    for line in config_file_contents.lines() {
        if line.trim().starts_with("wallpaper_path") {
            let wallpapers_path_s: &str = line
                .split('=')
                .nth(1)
                .expect("No wallpaper_path on config file");
            wallpapers_path = PathBuf::from(wallpapers_path_s.trim());
        }
        if line.trim().starts_with("prev_wallpaper") {
            other_wall = true;
        }
    }

    if !wallpapers_path.is_dir() {
        println!("The path stored in the config file is not a path");
        println!("Config file: ~/.config/bgc/config.conf");
        println!("Wallpapers directory provided: {}", wallpapers_path.to_str().expect("No file provided"));
        exit(1);
    }

    let mut images: Vec<String> = Vec::new();
    let mut images_p: Vec<PathBuf> = Vec::new();

    fs::read_dir(&wallpapers_path)
        .expect("Couldn't read directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| is_img(&entry.path()))
        .for_each(|entry| {
            let entry_p: PathBuf = entry.path();
            if let Some(entry_string) = entry_p.file_name().and_then(|x| x.to_str()) {
                images.push(entry_string.to_string());
                images_p.push(entry_p);
            }
        });
    
    let vstring: String = "v".to_string() + env!("CARGO_PKG_VERSION") + "\n";

    let buttons: Vec<_> = images
                    .iter()
                    .map(|image| button(image))
                    .collect();

    let mut menu_v = vec![
        label("-----------------------------------------------------------------"),
        label("Welcome to Background Changer (bgc)"),
        label(vstring),
        label("You can move up and down using 'j' and 'k'"),
        label("Press 'enter' to select a wallpaper or 'i' to preview the image"),
        label("-----------------------------------------------------------------\n"),
    ];

    menu_v.extend(buttons.into_iter());

    menu_v.push(label(""));
    
    menu_v.push(button("Online wallpaper"));

    menu_v.push(label(""));

    menu_v.push(button("Quit"));

    let menu = menu(menu_v);
    run(&menu);

    let menu_result = mut_menu(&menu);
    let selected: &str = menu_result.selected_item_name();

    if selected == "Quit" {
        exit(1);
    } else if selected == "Online wallpaper" {
        println!("Online wallpaper not aviable yet");
        exit(0);
    }

    let selected_path: Option<PathBuf> = images_p
        .into_iter()
        .find(|x| {
            x.to_str()
                .unwrap_or_default()
                .contains(selected)
        });

    let image_path: String = match selected_path {
        Some(path) => path.to_string_lossy().to_string(),
        None => {
            panic!("Selected path not found");
        }
    };

    match Command::new("swww")
            .arg("img")
            .arg(&image_path)
            .arg("--transition-step")
            .arg("30")
            .arg("--transition-fps")
            .arg("60")
            .spawn() {
                Ok(child) => child,
                Err(err) => panic!("Swww not installed\n{}", err),
            };

    if other_wall {
        let contents = fs::read_to_string(conf_path.clone())
            .expect("Couldn't read config path");

        let repl = contents
            .trim()
            .split('=')
            .last()
            .expect("There was no =");
        
        let new_data_file = contents.replace(repl, &image_path);

        let mut w_file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(conf_path)
            .expect("Couldn't replace text from older file");
        
        w_file.write(new_data_file.as_bytes())
            .expect("Error writing data to file");
    } else {
        let new_data_file: String = "prev_wallpaper = ".to_string() + &image_path;
        
        fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(conf_path)
            .expect("Couldn't replace text from older file")
            .write(new_data_file.as_bytes())
            .expect("Error writing data to file");
    }
}
