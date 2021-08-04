
extern crate udev;

use std::{collections::HashSet, ffi::OsString, path::PathBuf};

use udev::{Device, Enumerator};


#[derive(PartialEq, Debug)]
pub enum InputType {
    Keyboard,
    Mouse,
}

#[derive(PartialEq, Debug)]
pub struct DeviceData {
    pub path: PathBuf,
    pub name: OsString,
    pub input_type: InputType,
}

fn device_is_keyboard(device: &Device) -> bool {
    return device
        .property_value("ID_INPUT_KEYBOARD")
        .unwrap_or_default()
        .eq("1");
}

fn device_has_tag(device: &Device, tag: &str) -> bool {
    let wrapped_tag = format!(":{}:", tag);
    return device
        .property_value("TAGS")
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .contains(&wrapped_tag);
}

fn device_has_any_tag(device: Device, tags: HashSet<&str>) -> bool {
    return tags.iter().any(|tag| device_has_tag(&device, tag));
}

fn seat_as_tag(seat: &str) -> &str {
    return if seat.eq("seat0") { "seat" } else { seat };
}

fn enumerate_keyboards_for_seat(seat: &str) -> Result<Enumerator, std::io::Error> {
    let mut enumerator = udev::Enumerator::new()?;

    enumerator.match_subsystem("input")?;
    enumerator.match_property("ID_INPUT_KEYBOARD", "1")?;
    enumerator.match_tag(seat_as_tag(seat))?;

    Ok(enumerator)
}

pub fn list_keyboards(seats: &Vec<String>) -> Result<Vec<DeviceData>, std::io::Error> {
    let mut initial_devices: Vec<DeviceData> = Vec::new();

    for tag in seats.iter() {
        let mut devices: Enumerator = enumerate_keyboards_for_seat(tag)?;
        for device in devices.scan_devices().unwrap() {
            initial_devices.push(DeviceData {
                path: device.syspath().to_owned(),
                name: device.sysname().to_owned(),
                input_type: InputType::Keyboard,
            });
        }
    }
    return Ok(initial_devices);
}

