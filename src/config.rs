use std::path::Path;

/// Validation error codes
enum Validation {
    MissingFilePath,
    FileDoesntExist,
    WrongImageFormat,
    Succeeded,
    NeedsHelp,
}

pub struct Config {
    pub decode: bool,
    pub help: bool,
    pub org_image_path: Option<String>,
    pub file_to_hide_path: Option<String>,
    pub save_path: Option<String>,
}

impl Config {
    fn new() -> Config {
        Config {
            decode: false,
            help: false,
            org_image_path: None,
            file_to_hide_path: None,
            save_path: None,
        }
    }

    pub fn from_args(args: &Vec<String>) -> Result<Config, String> {
        let mut conf = Config::new();

        if args.len() == 1 {
            conf.help = true;
            return Ok(conf);
        }

        let mut args_iter = args.iter();

        loop {
            let arg = args_iter.next();

            if arg == None {
                break;
            }

            match arg.unwrap().as_str() {
                "--decode" => conf.decode = true,
                s if s == "-h" || s == "--help" => conf.help = true,
                "-i" => conf.org_image_path = Some(args_iter.next().unwrap().clone()),
                "-f" => conf.file_to_hide_path = Some(args_iter.next().unwrap().clone()),
                "-o" => conf.save_path = Some(args_iter.next().unwrap().clone()),
                _ => {}
            }
        }

        match conf.validate() {
            Validation::MissingFilePath => Err("Missing file paths!".to_string()),
            Validation::FileDoesntExist => Err("File doesn't exist!".to_string()),
            Validation::WrongImageFormat => Err("Wrong image format!".to_string()),
            Validation::NeedsHelp => Ok(conf),
            Validation::Succeeded => Ok(conf),
        }
    }

    fn validate(&self) -> Validation {
        if self.help {
            return Validation::NeedsHelp;
        }

        if self.decode {
            if self.org_image_path == None || self.save_path == None {
                return Validation::MissingFilePath;
            }

            if !check_file_extension(self.org_image_path.as_ref()) {
                return Validation::WrongImageFormat;
            }

            if !check_file(self.org_image_path.as_ref()) {
                return Validation::FileDoesntExist;
            }

            return Validation::Succeeded;
        }

        if self.org_image_path == None || self.save_path == None || self.file_to_hide_path == None {
            return Validation::MissingFilePath;
        }

        if !check_file_extension(self.org_image_path.as_ref())
            || !check_file_extension(self.save_path.as_ref())
        {
            return Validation::WrongImageFormat;
        }

        if !(check_file(self.org_image_path.as_ref())
            && check_file(self.file_to_hide_path.as_ref()))
        {
            return Validation::FileDoesntExist;
        }

        return Validation::Succeeded;
    }
}

fn check_file(path: Option<&String>) -> bool {
    if path == None {
        return false;
    }

    Path::new(path.unwrap().as_str()).is_file()
}

fn check_file_extension(path: Option<&String>) -> bool {
    if path == None {
        return true;
    }

    path.unwrap().ends_with("png")
}
