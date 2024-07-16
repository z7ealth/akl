use std::{thread::sleep, time::Duration};

use hidapi::HidApi;
use psutil::sensors;

const VENDOR_ID: u16 = 0x3633;
const PRODUCT_ID: u16 = 0x0003;
const INTERVAL: u64 = 2;

fn get_bar_value(value: f64) -> f64 {
    return value - 1.0;
}

fn get_data(value: f64, mode: &str) -> Vec<u8> {
    let mut base_data = vec![0; 64];
    let numbers: Vec<u8> = value
        .to_string() // Convert the integer to a string
        .chars() // Get an iterator over the characters of the string
        .map(|c| c as u8) // Convert each character to a digit
        .collect();

    base_data[0] = 16;
    base_data[2] = get_bar_value(value) as u8;

    match mode {
        "start" => base_data[1] = 170,
        "util" => base_data[1] = 76,
        _ => base_data[1] = 19,
    }

    match numbers.len() {
        0 | 1 => base_data[5] = numbers[0],
        2 => {
            base_data[4] = numbers[0];
            base_data[5] = numbers[1];
        }
        3 => {
            base_data[3] = numbers[0];
            base_data[4] = numbers[1];
            base_data[5] = numbers[2];
        }
        _ => {
            base_data[3] = numbers[0];
            base_data[4] = numbers[1];
            base_data[5] = numbers[2];
            base_data[6] = numbers[3];
        }
    }

    return base_data;
}

fn get_cpu_temperature() -> f64 {
    let temperatures = sensors::temperatures();
    let mut cpu_temp = 0.0;
    let cpu_labels = ["CPU", "Tctl"];

    for temperature in temperatures {
        match temperature {
            Ok(sensor_temp) => {
                if cpu_labels.contains(&sensor_temp.label().unwrap_or_default()) {
                    cpu_temp = sensor_temp.current().celsius();
                }
            }
            Err(_) => (),
        }
    }

    println!("CPU Temp: {}", cpu_temp);

    cpu_temp
}

fn get_temp() -> Vec<u8> {
    let temp = get_cpu_temperature();
    get_data(temp, "temp")
}

fn main() {
    println!("Printing all available hid devices:");

    match HidApi::new() {
        Ok(api) => {
            let ak = api.open(VENDOR_ID, PRODUCT_ID).unwrap();

            println!(
                "Device found: {}",
                ak.get_product_string().unwrap().unwrap()
            );

            ak.set_blocking_mode(false).unwrap();
            ak.write(&get_data(0.0, "start")).unwrap();

            loop {
                ak.set_blocking_mode(false).unwrap();
                ak.write(&get_temp()).unwrap();
                sleep(Duration::from_secs(INTERVAL));
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
