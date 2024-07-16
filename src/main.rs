use hidapi::HidApi;

const VENDOR_ID: u16 = 0x3633;
const PRODUCT_ID: u16 = 0x0003;

fn get_bar_value(value: u8) -> u8 {
    return value - 1;
}

fn get_data(value: u8, mode: &str) -> Vec<u8> {
    let mut base_data = vec![0; 64];
    let numbers: Vec<u8> = value
        .to_string() // Convert the integer to a string
        .chars() // Get an iterator over the characters of the string
        .map(|c| c.to_digit(10).unwrap() as u8) // Convert each character to a digit
        .collect();

    base_data[0] = 16;
    base_data[2] = get_bar_value(value);

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
            ak.write(&get_data(0, "start")).unwrap();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
