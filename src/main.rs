use core::panic;
use std::{
    io::{stdin, Write},
    path::PathBuf,
    process::exit,
    fs::{self, File},
    process::Command, vec
};
use dirs::home_dir;
use terminal_menu::{menu, button, run, mut_menu, label};

fn is_img(file_path: &PathBuf) -> bool {
    let img_files: Vec<&str> = vec!["jpeg", "jpg", "png", "gif", "pnm", "tga", "ttf", "webp", "bmp", "farb", "farbfeld"];

    if let Some(extension) = file_path.extension() {
        if let Some(ext_str) = extension.to_str() {
            return img_files.contains(&ext_str);
        }
    }
    false
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let conf_path: PathBuf = home_dir()
        .expect("Error getting home dir")
        .join(".config/bgc/config.conf");

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
            .create(home_dir().expect("Error getting home directory").join(".config/bgc/")) {
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

        clear_screen();
    } 

    let config_file_contents: String = fs::read_to_string(conf_path)
                                .unwrap();
    
    let mut wallpapers_path: PathBuf = PathBuf::new();

    for line in config_file_contents.lines() {
        if line.trim().starts_with("wallpaper_path") {
            let wallpapers_path_s: &str = line
                .split('=')
                .nth(1)
                .expect("No wallpaper_path on config file");
            wallpapers_path = PathBuf::from(wallpapers_path_s.trim());
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

    for path in fs::read_dir(&wallpapers_path).expect("Couldn't read directory") {
        if let Ok(entry) = path {
            let path_buf = entry.path();
            if is_img(&path_buf) {
                let str_path: String = String::from(path_buf.to_str()
                                                    .expect("Error converting to string")
                                                )
                    .split('/')
                    .last()
                    .expect("Error geting &str")
                    .to_string();
                images.push(str_path);
                images_p.push(path_buf);
            }
        }
    }
    
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

    for element in buttons.into_iter() {
        menu_v.push(element);
    }
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

    let mut selected_path: Option<PathBuf> = None;

    for n in images_p {
        if n.to_str().unwrap().contains(selected) {
            selected_path = Some(n);
            break;
        }
    }

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

}
