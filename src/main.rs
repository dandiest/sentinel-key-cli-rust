use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
enum ServiceType {
    Local,
    Remote { ip: String, protocol: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct AccessEvent {
    timestamp: DateTime<Utc>,
    success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct CredentialVault {
    id: u32,
    service_name: String,
    service_type: ServiceType,
    access_history: Vec<AccessEvent>,
}

fn main() {
    println!("Sentinel-Key System Initializing...");

    let file_path = "vault.json";

    let mut database: Vec<CredentialVault> = match fs::read_to_string(file_path) {  
        Ok(content) => {
            println!("Database found. Loading...");
            serde_json::from_str(&content).expect("Error during JSON parsing")
        }
        Err(_) => {
            println!("Database not found. Creating new caveau...");
            Vec::new() 
        }
    };

    if database.is_empty() {           // If the database is empty, creates a new service
        let first_service = CredentialVault {
            id: 1,
            service_name: "Main server".to_string(),
            service_type: ServiceType::Remote {
                ip: String::from("192.168.1.100"),
                protocol: String::from("SSH"),
            },
            access_history: Vec::new(),
        };
        database.push(first_service);     
    };

    if let Some(service) = database.get_mut(0) { 
        let new_event = AccessEvent {  
            timestamp: Utc::now(),                   
            success: true,
        };

        println!("Login registration for: {}", service.service_name);
        service.access_history.push(new_event);
    }

    let json_data = serde_json::to_string_pretty(&database)          // Serialization
    .expect("Error during serialization");

    fs::write(file_path, json_data)                                  // Writing file on disk
    .expect("Error during file writing.");

    println!("Database updated and saved successfully!");
}
