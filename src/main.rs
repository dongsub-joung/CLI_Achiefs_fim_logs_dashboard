use std::env;
use chrono::Local;
use std::path::Path;
use nix::unistd::Pid;
use rev_lines::RevLines;
use std::fs::{self, File};
use std::io::{self, BufReader};
use serde::{Deserialize, Serialize};
use nix::sys::signal::{Signal, kill};

#[derive(Serialize, Deserialize, Debug)]
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

// for fn print_all_data
fn print_json_data_pretty(v_events: &Vec<ChangedEvent>) {
    match serde_json::to_string_pretty(v_events) {
        Ok(json_str) => println!("{}", json_str),
        Err(e) => eprintln!("Failed to serialize for printing: {}", e),
    }
}

fn main() {
    let result_v_events = |data: String| -> Result<Vec<ChangedEvent>, serde_json::Error> {
        let v_events: Vec<ChangedEvent> = serde_json::from_str(&data)?;

        Ok(v_events)
    };

    let get_file_size = |json_path: &'static str| -> io::Result<u64> {
        // metadata() fetches file information from the OS
        let metadata = fs::metadata(json_path)?;

        // len() returns the size in bytes
        Ok(metadata.len())
    };

    let get_latest_json_lines = |json_path: &str, limit: usize| -> Vec<ChangedEvent> {
        let file = File::open(json_path).expect("Could not open file");
        let rev_lines = RevLines::new(BufReader::new(file));

        // Take the last 'limit' lines and parse them
        rev_lines
            .take(limit)
            .filter_map(|line| {
                let line_str = line.ok()?;
                serde_json::from_str::<ChangedEvent>(&line_str).ok()
            })
            .collect()
    };

    fn rotate_event_logs() -> io::Result<()> {
        
        // 1. Generate the filename with the current date: e.g., 2026-04-26_logs.json
        let date_str = Local::now().format("%Y-%m-%d").to_string();
        let destination_path = format!("{}_logs.json", date_str);

        // Check if source exists before trying to copy
        if Path::new(JSON_PATH).exists() {
            // 2. Copy the file to the new dated log
            fs::copy(JSON_PATH, &destination_path)?;
            println!("Successfully backed up to {}", destination_path);

            // 3. Overwrite/Clear the original file (makes it 0 bytes)
            // If you'd rather delete it entirely, use fs::remove_file(source_path)?;
            fs::write(JSON_PATH, "")?;
        } else {
            println!("Source file {} does not exist. Skipping.", JSON_PATH);
        }

        Ok(())
    }
    fn fim_process_sleep_for_backup() {
        let args: Vec<String> = env::args().collect();

        let string_pid = &args[1];
        let pid_raw = string_pid
            .parse::<u32>()
            .expect("Not a valid number. Plz input PID");

        let pid = nix::unistd::Pid::from_raw(pid_raw);

        match kill(pid, Signal::SIGSTOP) {
            Ok(_) => println!("Daemon {} paused.", pid_raw),
            Err(e) => eprintln!("Failed to pause: {}", e),
        }

        let result_backup= rotate_event_logs();
        match result_backup {
            Err(e) => { panic!("failed to backup process"); },
            Ok(()) => { }
        };

        match kill(pid, Signal::SIGCONT) {
            Ok(_) => println!("Daemon {} resumed.", pid_raw),
            Err(e) => eprintln!("Failed to resume: {}", e),
        }
    }

    // init
    loop {
        let data: String = fs::read_to_string(JSON_PATH).expect("failed to find path");

        let _result_data = result_v_events(data);

        // It mean get latest 2 Json data
        let recent_events = get_latest_json_lines(JSON_PATH, 2_usize);

        print_json_data_pretty(&recent_events);

        let file_size = get_file_size(JSON_PATH).expect("failed get a log file size");
        if file_size >= 20048000000_u64 {
            fim_process_sleep_for_backup();
        }

        std::process::exit(0);
    }
}

// Options
fn print_all_data(result_data: &Result<Vec<ChangedEvent>, serde_json::Error>) {
    match result_data {
        Ok(event) => {
            print_json_data_pretty(event);
        }
        Err(e) => {
            panic!("<{e}> failed parsing log data");
        }
    }
}
