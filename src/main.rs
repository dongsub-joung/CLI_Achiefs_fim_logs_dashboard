use serde::Deserialize;
use std::fs::{self, File};

#[derive(Deserialize, Debug)]
struct ChangedEvent {

}

const JSON_PATH: &'static str= "/var/lib/fim/events.json";

fn read_json_data() -> Result< Vec<ChangedEvent>, Box<dyn std::error::Error>> {
    // @TODO need to caching or read only last data
    let data = fs::read_to_string(JSON_PATH)?;
    
    let v_events: Vec<ChangedEvent> = serde_json::from_str(&data)?;

    Ok(v_events)
}

fn print_json_data(v_events: & Vec<ChangedEvent>){
    for event in v_events {
        println!("");
    }
}

// @TODO
fn get_file_size(){
    
}

fn main() {
    loop {
        let v_events= read_json_data();
    
        match v_events {
            Ok(event) =>{
                print_json_data(&event);
            },
            Err(e) => {
                panic!("<{e}> failed parsing log data");
            }
        }
        
        let file_size= get_file_size();
        if file_size >= 20048000000 {
            // @TODO copy a events.json to a {Date}_logs.json
            // sleep fim process for unblcok
            // remove or overwrite a events.json
        }

        std::process::exit(0);
    }
}
