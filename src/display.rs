use std::{sync::Arc, thread::sleep, time::Duration};

use hidapi::HidApi;
use sysinfo::{Components, System};
use tokio::sync::Mutex;

const VENDOR_ID: u16 = 0x3633;
const PRODUCT_ID: u16 = 0x0003;
const INTERVAL: u64 = 2;

fn get_bar_value(value: f32) -> f32 {
    
    if value < 10.0 {
        return  0.0;
    }

    value / 10.0
}

fn get_data(value: f32, mode: &str) -> Vec<u8> {
    let mut base_data = vec![0; 64];

    let numbers: Vec<char> = (value as i32)
        .to_string() // Convert the integer to a string
        .chars()
        .collect(); // Get an iterator over the characters of the string

    base_data[0] = 16; // ?

    match mode {
        "start" => base_data[1] = 170,
        "util" => base_data[1] = 76,
        _ => base_data[1] = 19, // CPU Temp mode
    }

    base_data[2] = get_bar_value(value) as u8; // Bar

    if numbers.len() == 1 {
        base_data[5] = numbers[0].to_digit(10).unwrap() as u8;
    }

    if numbers.len() == 2 {
        base_data[4] = numbers[0].to_digit(10).unwrap() as u8;
        base_data[5] = numbers[1].to_digit(10).unwrap() as u8;
    }

    if numbers.len() == 3 {
        base_data[3] = numbers[0].to_digit(10).unwrap() as u8;
        base_data[4] = numbers[1].to_digit(10).unwrap() as u8;
        base_data[5] = numbers[2].to_digit(10).unwrap() as u8;
    }

    base_data
}

fn get_cpu_temperature() -> f32 {
    let mut cpu_temp = 0.0;
    let amd_cpu_label = "k10temp Tctl";
    let intel_cpu_label = "coretemp";
    let components = Components::new_with_refreshed_list();

    for component in components.list() {
        if component.label().contains(amd_cpu_label) || component.label().contains(intel_cpu_label)
        {
            cpu_temp = component.temperature()
        }
    }

    cpu_temp
}

fn get_cpu_utilization() -> f32 {
    let mut system = System::new_all();

    // First we need to update all information of our system struct.
    system.refresh_cpu();

    system.global_cpu_info().cpu_usage()
}

fn get_temp() -> Vec<u8> {
    let temp = get_cpu_temperature();
    get_data(temp, "temp")
}

fn get_util() -> Vec<u8> {
    let util = get_cpu_utilization();
    get_data(util, "util")
}

pub async fn start(mode: Arc<Mutex<&str>>) {
    match HidApi::new() {
        Ok(api) => {
            let ak = api.open(VENDOR_ID, PRODUCT_ID).unwrap();

            println!(
                "Device found: {}",
                ak.get_product_string().unwrap().unwrap()
            );

            ak.set_blocking_mode(false).unwrap();
            ak.write(&get_data(0.0, "start")).unwrap();

            sleep(Duration::from_secs(1));

            loop {
                ak.set_blocking_mode(false).unwrap();

                println!("Selected mode: {:?}", *mode);

                match *mode.lock().await {
                    "util" => ak.write(&get_util()).unwrap(),
                    _ => ak.write(&get_temp()).unwrap(),
                };

                sleep(Duration::from_secs(INTERVAL));
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
