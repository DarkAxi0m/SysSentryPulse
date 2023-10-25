extern crate sysinfo;
extern crate uuid;

use serde::{Serialize, Deserialize};
use sysinfo::{System, SystemExt, DiskExt};
use std::ffi::OsStr;
use std::collections::HashMap;
use reqwest;



const APP_NAME: &str = "sirmon";

mod config;


#[derive(Debug, Deserialize, Serialize)]
struct Response {
   id: String,
   disks: HashMap<String, String>     
}


fn main()  -> Result<(), reqwest::Error> {
      let config = config:: read_config().expect("Failed to read config");


     println!("Using config: {:?}", config);

    let mut sys = System::new_all();
    sys.refresh_all();


    let mut diskmap = HashMap::new();
    for disk in sys.disks() {
        println!("Disk name: {:?}", disk.name());
        println!("Disk file system: {:?}", disk.file_system());
        println!("Total space: {:?}", disk.total_space());
        println!("Available space: {:?}", disk.available_space());
        
        let disk_name: &OsStr = OsStr::new(disk.name());

        diskmap.insert(
                    disk_name.to_str().unwrap_or("unknown").to_string(), 
                    disk.available_space().to_string() 
                   );
    }



    let r ={Response {
        id: config.id,
        disks: diskmap
    }};


    let client = reqwest::blocking::Client::new();
    let res = client.post("http://dockerhost.accede.com.au:1880/status")
        .json(&r)
        .send()?;

    println!("Status: {}", res.status());
    let text = res.text()?;
    println!("Body: {}", text);

    Ok(())

}
