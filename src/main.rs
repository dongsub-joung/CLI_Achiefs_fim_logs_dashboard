use chrono::Local;
use serde::Deserialize;
use std::fs::{self};
use std::io;
use std::path::Path;

#[derive(serde::Serialize, Deserialize, Debug)]
struct ChangedEvent {
    detailed_operation: String,
    files: String,
    file_size: usize,
    fpid: usize,
    hostname: String,
    labels: String,
    operation: String,
    system: String,
    timestamp: String,
}

const JSON_PATH: &'static str = "/var/lib/fim/events.json";

fn main() {
    let result_v_events = |data: String| -> Result<Vec<ChangedEvent>, serde_json::Error> {
        let v_events: Vec<ChangedEvent> = serde_json::from_str(&data)?;

        Ok(v_events)
    };
    let print_json_data_pretty =
        |v_events: &Vec<ChangedEvent>| match serde_json::to_string_pretty(v_events) {
            Ok(json_str) => println!("{}", json_str),
            Err(e) => eprintln!("Failed to serialize for printing: {}", e),
        };
    let get_file_size = |json_path: &'static str| -> io::Result<u64> {
        // metadata() fetches file information from the OS
        let metadata = fs::metadata(json_path)?;

        // len() returns the size in bytes
        Ok(metadata.len())
    };

    let rotate_event_logs= || -> io::Result<()> {
        let source_path = "events.json";

        // 1. Generate the filename with the current date: e.g., 2026-04-26_logs.json
        let date_str = Local::now().format("%Y-%m-%d").to_string();
        let destination_path = format!("{}_logs.json", date_str);

        // Check if source exists before trying to copy
        if Path::new(source_path).exists() {
            // 2. Copy the file to the new dated log
            fs::copy(source_path, &destination_path)?;
            println!("Successfully backed up to {}", destination_path);

            // 3. Overwrite/Clear the original file (makes it 0 bytes)
            // If you'd rather delete it entirely, use fs::remove_file(source_path)?;
            fs::write(source_path, "")?;
        } else {
            println!("Source file {} does not exist. Skipping.", source_path);
        }

        Ok(())
    };


    // init
    loop {
        // @TODO I need to get a few of data(lastest)
        let data: String = fs::read_to_string(JSON_PATH).expect("failed to find path");
        match result_v_events(data) {
            Ok(event) => {
                print_json_data_pretty(&event);
            }
            Err(e) => {
                panic!("<{e}> failed parsing log data");
            }
        }

        let file_size = get_file_size(JSON_PATH).expect("failed get a log file size");
        if file_size >= 20048000000 {
            // @TODO sleep fim process for unblcok
            let _ = rotate_event_logs();
        }

        
        std::process::exit(0);
    }
}
