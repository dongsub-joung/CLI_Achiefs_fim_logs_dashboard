use serde::{Deserialize, Serialize};
use std::fs::{self, File, read};

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

fn print_json_data(v_events: &Vec<ChangedEvent>) {
    for event in v_events {
        println!("");
    }
}

// @TODO
fn get_file_size() -> usize {
    1
}

fn main() {
    let result_v_events = |data: String| -> Result<Vec<ChangedEvent>, serde_json::Error> {
        let v_events: Vec<ChangedEvent> = serde_json::from_str(&data)?;
        
        Ok(v_events)
    };
    
    loop {    
        let data: String = fs::read_to_string(JSON_PATH).expect("msg");
        match result_v_events(data) {
            Ok(event) => {
                print_json_data(&event);
            }
            Err(e) => {
                panic!("<{e}> failed parsing log data");
            }
        }

        let file_size = get_file_size();
        if file_size >= 20048000000 {
            // @TODO copy a events.json to a {Date}_logs.json
            // sleep fim process for unblcok
            // remove or overwrite a events.json
        }

        std::process::exit(0);
    }
}
