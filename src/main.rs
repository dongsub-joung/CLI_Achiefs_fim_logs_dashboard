use std::fs::File;

use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct ChangedEvent {

}

fn read_json_data() -> Result< Vec<ChangedEvent>, Box<dyn std::error::Error>> {
    let json_path= "/var/lib/fim/events.json";
    
    let data = fs::read_to_string(json_path)?;

    let v_events: Vec<ChangedEvent> = serde_json::from_str(&data)?;

    Ok(v_events)
}

fn print_json_data(v_events: & Vec<ChangedEvent>){
    for event in v_events {
        println!("");
    }
}

fn main() {
    let v_events= read_json_data();

    match v_events {
        Ok(event) =>{
            print_json_data(&event);
        },
        Err(e) => {
            panic!("<{e}> failed parsing log data");
        }
    }


}
