extern crate udev;

use std::{collections::HashSet, ffi::OsString, path::PathBuf};

use udev::{Device, Enumerator};

mod systemd_service;
mod udev_service;

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

fn list_keyboard_devices(seats: &Vec<String>) -> Result<Vec<DeviceData>, std::io::Error> {
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

#[derive(PartialEq, Debug)]
enum InputType {
    Keyboard,
    Mouse,
}

#[derive(PartialEq, Debug)]
struct DeviceData {
    path: PathBuf,
    name: OsString,
    input_type: InputType,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::system()?;
    let sessions = systemd_service::list_sessions(&connection)?;

    dbg!(&sessions);

    let seats: Vec<String> = sessions.iter().map(|sess| sess.seat.clone()).collect();

    let devices = list_keyboard_devices(&seats)?;

    dbg!(&devices);

    return Ok(());

    udev_service::poll(|event| {
        let device = event.device();
        if device_is_keyboard(&device) {
            println!("IS KEYBOARD");
        }

        if device_has_tag(&device, "seat") {
            println!("HAS SEAT");
        }

        println!(
            "{}: {} {} (subsystem={}, sysname={}, devtype={})",
            event.sequence_number(),
            event.event_type(),
            event.syspath().to_str().unwrap_or("---"),
            event
                .subsystem()
                .map_or("", |s| { s.to_str().unwrap_or("") }),
            event.sysname().to_str().unwrap_or(""),
            event.devtype().map_or("", |s| { s.to_str().unwrap_or("") })
        );
    })?;

    Ok(())
}

