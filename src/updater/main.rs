use std::{fs, time::Duration};

fn main() {
    
    let app_data_directory = match std::env::var("LocalAppData") {
        Ok(program_files_directory) => program_files_directory,
        Err(error) => {
            println!("Failed to find app data directory: {}", error.to_string());
            //std::thread::sleep(web_time::Duration::from_secs(5));
            panic!();
        }
    };

    let application_directory = format!("{}\\Programs\\liquidators", app_data_directory);

    let application_binary_path = format!("{}\\editor-client.exe", application_directory);

    // create application directory if it doesnt already exist
    match fs::create_dir_all(application_directory.clone()) {
        Ok(_) => {},
        Err(error) => {
            println!("Failed to create application directory: {}", error.to_string());
            //std::thread::sleep(Duration::from_secs(5));
            panic!();
        },
    }

    match download("https://liquidators.vxny.io/editor-client.exe") {
        Ok(bytes) => {
            
            match fs::write(application_binary_path, bytes) {
                Ok(_) => {},
                Err(error) => {
                    println!("Failed to write download to disk: {}", error.to_string());
                    std::thread::sleep(Duration::from_secs(5));
                    panic!();
                }

            }
        },
        Err(error) => {
            println!("Failed to download latest binary: {}\n\nLaunching existing binary...", error.to_string());
            std::thread::sleep(Duration::from_secs(5));
        },
    }

    match std::process::Command::new(format!("{}\\editor-client.exe", application_directory.clone())).output() {
        Ok(_) => {},
        Err(error) => {
            println!("Failed to run binary: {}", error.to_string());
            std::thread::sleep(Duration::from_secs(5));
            panic!();
        },
    }
}

fn download(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = reqwest::blocking::get(url).expect("Failed to download");

    response.error_for_status_ref()?;
    
    let bytes = response.bytes()
        .expect("Unable to download")
        .to_vec();

    Ok(bytes)


}