use regex::Regex;
use std::io::Write;
use std::{env, process::Command, string::FromUtf8Error};

fn get_device_data() -> Result<String, FromUtf8Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ls -l /dev/disk/by-uuid")
        .output()
        .expect("failed to execute process");
    let devices_string = String::from_utf8(output.stdout)?;
    Ok(devices_string)
}

fn data_to_vec(data: String) -> Vec<String> {
    let data_lines = data.lines();
    let mut data_vec: Vec<String> = Vec::new();

    for line in data_lines {
        data_vec.push(String::from(line));
    }
    data_vec.remove(0); // Remove first index as it's not needed
    data_vec
}

// Get all of the uuids in the vec
fn get_all_uuids(device_vec: &Vec<String>, lr: Regex, sr: Regex) -> Option<Vec<String>> {
    let mut uuids: Vec<String> = vec![];

    for device in device_vec {
        match lr.captures(&device) {
            Some(uuid) => uuids.push(uuid[0].to_string()),
            None => {}
        }

        match sr.captures(&device) {
            Some(uuid) => {
                uuids.push(uuid[0].to_string());
            }
            None => {}
        }
    }
    Some(uuids)
}

// Just get the uuid of the device that was passed in args
fn get_uuid_of(
    device_name: &str,
    device_vec: &Vec<String>,
    lr: Regex,
    sr: Regex,
) -> Option<String> {
    let mut uuid = String::new();
    for device in device_vec {
        if device.contains(device_name) {
            match lr.captures(&device) {
                Some(u) => uuid = u[0].to_string(),
                None => match sr.captures(&device) {
                    Some(u) => uuid = u[0].to_string(),
                    None => {}
                },
            }
        }
    }
    Some(uuid)
}

fn print_help() {
    println!("############################################################################");
    println!("#    uuidof usage:                                                         #");
    println!("#    [uuidof sda1] returns the UUID assocaited with the passed drive name. #");
    println!("#    [uuidof] returns all UUIDs                                            #");
    println!("#    [uuidof help or uuidof h] prints this help message.                   #");
    println!("############################################################################");
}

fn main() -> Result<(), regex::Error> {
    let args: Vec<String> = env::args().collect(); // Capture args
    let uuid_short_regex: Regex = Regex::new(r"[A-F0-9]{4}-[A-F0-9]{4}")?; // For short UUIDs
    let uuid_long_regex: Regex =
        Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")?;

    let _disk_name_regex: Regex = Regex::new("../../[a-z0-9]*")?;

    let data = get_device_data().unwrap();

    match args.len() {
        1 => {
            // Print out UUIDs
            for uuid in
                get_all_uuids(&data_to_vec(data), uuid_long_regex, uuid_short_regex).unwrap()
            {
                writeln!(std::io::stdout(), "{}", uuid).expect("Failed to write to stdout");
            }
        }
        2 => {
            if args[1] == "help" || args[1] == "h" {
                print_help();
                return Ok(());
            }

            if let Some(uuid) = get_uuid_of(
                &args[1],
                &data_to_vec(data),
                uuid_long_regex,
                uuid_short_regex,
            ) {
                writeln!(std::io::stdout(), "{}", uuid).expect("Failed to write to stdout");
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_device_data_test() {
        let data = get_device_data().unwrap();
        assert!(data.contains("total"));
    }

    #[test]
    fn data_to_vec_test() {
        let data_vec = data_to_vec(get_device_data().unwrap());
        assert!(data_vec[0].contains("lrwxrwxrwx"))
    }

    #[test]
    fn get_all_uuids_test() {
        let uuid_short_regex: Regex = Regex::new(r"[A-F0-9]{4}-[A-F0-9]{4}").unwrap(); // For short UUIDs
        let uuid_long_regex: Regex = Regex::new(
            r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}",
        )
        .unwrap();
        let uuids = get_all_uuids(
            &data_to_vec(get_device_data().unwrap()),
            uuid_long_regex.clone(),
            uuid_short_regex.clone(),
        );
        for uuid in uuids.unwrap() {
            if uuid_long_regex.is_match(&uuid) {
                assert!(uuid_long_regex.is_match(&uuid));
            } else if uuid_short_regex.is_match(&uuid) {
                assert!(uuid_short_regex.is_match(&uuid));
            }
        }
    }

    #[test]
    fn get_uuid_of_test() {} // I don't know how to write this test
}
