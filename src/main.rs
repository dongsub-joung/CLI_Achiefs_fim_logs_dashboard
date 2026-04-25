use serde::Deserialize;
use std::fs::{self, File, read};

#[derive(Deserialize, Debug)]
struct ChangedEvent {
    detailed_operation: &'static str,
    files: &'static str,
    file_size: usize,
    fpid: usize,
    hostname: &'static str,
    labels: &'static str,
    operation: &'static str,
    system: &'static str,
    timestamp: &'static str,
}

const JSON_PATH: &'static str = "/var/lib/fim/events.json";

fn read_json_data() -> Result<Vec<ChangedEvent>, serde_json::Error> {
    // {
        let opened_file = File::open(JSON_PATH);
        let file = match opened_file {
            Ok(_file) => _file,
            Err(e) => {
                panic!("{e}")
            }
        };

        let buf_reader = std::io::BufReader::new(file);

        let v_events = serde_json::Deserializer::from_reader(buf_reader);
        

    // }

    // & life time problme
    {
        // @TODO need to caching or read only last data
        let data = fs::read_to_string(JSON_PATH).expect("msg");

        let v_events: Vec<ChangedEvent> = serde_json::from_str(&data)?;

    }

    Ok(v_events)
}

fn print_json_data(v_events: &Vec<ChangedEvent>) {
    for event in v_events {
        println!("");
    }
}

// @TODO
fn get_file_size() {}

fn main() {
    loop {
        let v_events = read_json_data();

        match v_events {
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
