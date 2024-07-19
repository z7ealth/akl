use std::{thread::sleep, time::Duration};

use hidapi::HidApi;
use psutil::sensors;

const VENDOR_ID: u16 = 0x3633;
const PRODUCT_ID: u16 = 0x0003;
const INTERVAL: u64 = 2;

fn get_bar_value(value: f64) -> f64 {
    return value / 10.0;
}

fn get_data(value: f64, mode: &str) -> Vec<u8> {
    let mut base_data = vec![0; 64];

    let numbers: Vec<char> = value
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

    if value >= 10.0 && value < 100.0 {
        base_data[3] = 0; // Tens digit
        base_data[4] = numbers[0].to_digit(10).unwrap() as u8; // Tens digit
        base_data[5] = numbers[1].to_digit(10).unwrap() as u8; // Ones digit
    } else {
        eprintln!("Unusual temp: {}", value);
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

    cpu_temp
}

fn get_temp() -> Vec<u8> {
    let temp = get_cpu_temperature();
    get_data(temp, "temp")
}

pub async fn start() {
    match HidApi::new() {
        Ok(api) => {
            let ak = api.open(VENDOR_ID, PRODUCT_ID).unwrap();

            println!(
                "Device found: {}",
                ak.get_product_string().unwrap().unwrap()
            );

            ak.set_blocking_mode(false).unwrap();
            ak.write(&get_data(get_cpu_temperature(), "start")).unwrap();

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
