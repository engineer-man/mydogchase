use crate::constants::*;

use std::path::{Path, PathBuf};
use std::io::BufReader;
use std::env;
use std::fs::{self, File};
use std::io::{self};
use zip::ZipArchive;
use crate::solarized::{
    print_fancy, clear,
    VIOLET, BLUE, CYAN, GREEN, ORANGE, RED,
    PrintMode::NewLine,
};

pub fn generate_files() {
    print_fancy(&[
        ("Failed to read configuration\n", ORANGE, vec![]),
        ("Example environment can be created in the current active directory.\n", CYAN, vec![]),
        ("Would you like to create an example environment?\n", CYAN, vec![]),
        ("(", VIOLET, vec![]),
        ("y", BLUE, vec![]),
        ("/", VIOLET, vec![]),
        ("n", RED, vec![]),
        (")", VIOLET, vec![]),
    ], NewLine);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();
    if input == "y" || input == "yes" {
        clear();
        match fs::write("config.toml", EXAMPLE_CONFIG) {
            Ok(_) => {
                print_fancy(&[
                    ("Example ", CYAN, vec![]),
                    ("config.toml", VIOLET, vec![]),
                    (" file has been ", CYAN, vec![]),
                    ("created", GREEN, vec![]),
                    (".", CYAN, vec![]),
                ], NewLine);
            }
            Err(e) => {
                print_fancy(&[
                    ("Failed to create example config.toml file: ", ORANGE, vec![]),
                    (&format!("{}", e), RED, vec![]),
                ], NewLine);
            }
        }
        let templates = Path::new("templates");
        if !templates.exists() {
            match fs::create_dir_all(&templates) {
                Ok(_) => {
                    print_fancy(&[
                        ("The ", CYAN, vec![]),
                        ("", VIOLET, vec![]),
                        (" folder has been ", CYAN, vec![]),
                        ("created", GREEN, vec![]),
                        (".", CYAN, vec![]),
                    ], NewLine);
                }
                Err(e) => println!("Error creating templates: {:?}", e),
            }
        } else {
            print_fancy(&[
                ("static folder exists", ORANGE, vec![]),
            ], NewLine);
        }
        let public = Path::new("public");
        if !public.exists() {
            match fs::create_dir_all(&public) {
                Ok(_) => {
                    print_fancy(&[
                        ("The ", CYAN, vec![]),
                        ("public", VIOLET, vec![]),
                        (" folder has been ", CYAN, vec![]),
                        ("created", GREEN, vec![]),
                        (".", CYAN, vec![]),
                    ], NewLine);
                }
                Err(e) => {
                    print_fancy(&[
                        ("Error creating public: ", ORANGE, vec![]),
                        (&format!("{}", e), RED, vec![]),
                    ], NewLine);
                }
            }
        } else {
            println!("media folder exists");
        }
        let chase = Path::new("public/chase");
        if !chase.exists() {
            match fs::create_dir_all(&chase) {
                Ok(_) => {
                    print_fancy(&[
                        ("The ", CYAN, vec![]),
                        ("public/chase", VIOLET, vec![]),
                        (" folder has been ", CYAN, vec![]),
                        ("created", GREEN, vec![]),
                        (".", CYAN, vec![]),
                    ], NewLine);
                }
                Err(e) => {
                    print_fancy(&[
                        ("Error creating public/chase: ", ORANGE, vec![]),
                        (&format!("{}", e), RED, vec![]),
                    ], NewLine);
                }
            }
        } else {
            println!("media folder exists");
        }
        match fs::write("templates/home.html", EXAMPLE_HOME) {
            Ok(_) => {
                print_fancy(&[
                    ("Example ", CYAN, vec![]),
                    ("home.html", VIOLET, vec![]),
                    (" file has been ", CYAN, vec![]),
                    ("created", GREEN, vec![]),
                    (".", CYAN, vec![]),
                ], NewLine);
            }
            Err(e) => {
                print_fancy(&[
                    (&format!("{}", e), CYAN, vec![]),
                ], NewLine);
            }
        }
        match fs::write("templates/error.html", EXAMPLE_ERROR) {
            Ok(_) => {
                print_fancy(&[
                    ("Example ", CYAN, vec![]),
                    ("error.html", VIOLET, vec![]),
                    (" file has been ", CYAN, vec![]),
                    ("created", GREEN, vec![]),
                    (".", CYAN, vec![]),
                ], NewLine);
            }
            Err(e) => {
                print_fancy(&[
                    ("Failed to create example ", ORANGE, vec![]),
                    ("error.html", VIOLET, vec![]),
                    (" file: ", ORANGE, vec![]),
                    (&format!("{}", e), RED, vec![]),
                ], NewLine);
            }
        }
        let zip_path = "chase.zip";
        match std::fs::write(zip_path, ARCHIVE_DATA) {
            Ok(_) => {
                print_fancy(&[
                    ("Archive ", CYAN, vec![]),
                    (&format!("{}", zip_path), VIOLET, vec![]),
                    (" has been ", CYAN, vec![]),
                    ("saved", GREEN, vec![]),
                    (".", CYAN, vec![]),
                ], NewLine);
            }
            Err(e) => {
                print_fancy(&[
                    ("Failed to write image: ", ORANGE, vec![]),
                    (&format!("{}", e), RED, vec![]),
                ], NewLine);
            }
        }
        let file_path = Path::new("chase.zip");
        let file = File::open(&file_path).expect("Failed to open ZIP file");
        let mut archive = ZipArchive::new(BufReader::new(file)).expect("Failed to read ZIP archive");
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).expect("Failed to access file in ZIP archive");
            let file_name = file.name().to_string();
            fn construct_safe_path(file_name: &str) -> PathBuf {
                let mut path = PathBuf::new();
                for component in Path::new(file_name).components() {
                    match component {
                        std::path::Component::Normal(comp) => path.push(comp),
                        _ => {}
                    }
                }
                path
            }
            let outpath = construct_safe_path(&file_name);
            if file_name.ends_with('/') {
                print_fancy(&[
                    ("Directory ", CYAN, vec![]),
                    (&format!("{}", i), VIOLET, vec![]),
                    (" extracted to ", CYAN, vec![]),
                    (&format!("{}", outpath.display()), VIOLET, vec![]),
                ], NewLine);
                std::fs::create_dir_all(&outpath).expect("Failed to create directory");
            } else {
                print_fancy(&[
                    ("File ", CYAN, vec![]),
                    (&format!("{}", i), VIOLET, vec![]),
                    (" extracted to ", CYAN, vec![]),
                    (&format!("{}", outpath.display()), VIOLET, vec![]),
                ], NewLine);
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent).expect("Failed to create directory");
                }
                let mut outfile = std::fs::File::create(&outpath).expect("Failed to create file");
                std::io::copy(&mut file, &mut outfile).expect("Failed to copy file");
            }
        }
        print_fancy(&[
            ("ZIP archive ", CYAN, vec![]),
            ("extracted", GREEN, vec![]),
        ], NewLine);
        std::fs::remove_file(file_path).expect("Failed to delete ZIP file");
        print_fancy(&[
            ("ZIP file deleted ", CYAN, vec![]),
            ("successfully", GREEN, vec![]),
        ], NewLine);
        let path = env::current_dir().expect("asdf");
        print_fancy(&[
            ("\nSetup in ", CYAN, vec![]),
            (&format!("{}", path.display()), VIOLET, vec![]),
            (" is ", CYAN, vec![]),
            ("complete", GREEN, vec![]),
            (".\n", CYAN, vec![]),
            ("Please read config and edit ", CYAN, vec![]),
            ("config.toml", VIOLET, vec![]),
            (" to preferences.", CYAN, vec![]),
        ], NewLine);
    }
}
