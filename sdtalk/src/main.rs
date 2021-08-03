extern crate udev;

use std::{collections::HashSet};

use udev::{Device, Enumerator};
use zbus::dbus_proxy;
use zvariant::OwnedObjectPath;

mod poll;

#[dbus_proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait LoginManager {
    fn list_sessions(&self) -> zbus::Result<Vec<(String, u32, String, String, OwnedObjectPath)>>;

    fn get_session(&self, session_id: &str) -> zbus::Result<OwnedObjectPath>;

    fn list_users(&self) -> zbus::Result<Vec<(u32, String, OwnedObjectPath)>>;

    fn list_seats(&self) -> zbus::Result<Vec<(String, OwnedObjectPath)>>;

    fn get_seat(&self, seat_id: &str) -> zbus::Result<OwnedObjectPath>;
}

#[dbus_proxy(
    interface = "org.freedesktop.login1.Seat",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1/seat"
)]
trait SeatManager {
    #[dbus_proxy(property)]
    fn active_session(&self) -> zbus::Result<(String, OwnedObjectPath)>;
}

fn device_is_keyboard(device: &Device) -> bool {
    return device
        .property_value("ID_INPUT_KEYBOARD")
        .unwrap_or_default()
        .eq("1");
}

fn device_has_tag(device: &Device, tag: &str) -> bool
{
    let wrapped_tag = format!(":{}:", tag);
    return device
        .property_value("TAGS")
        .unwrap_or_default()
        .to_str().unwrap_or_default()
        .contains(&wrapped_tag);
}

fn device_has_any_tag(device: Device, tags: HashSet<&str>) -> bool {
    return tags.iter().any(|tag| device_has_tag(&device, tag));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::system()?;

    let proxy = LoginManagerProxy::new(&connection)?;

    // dbg!(proxy.list_users()?);

    let seats = &proxy.list_seats()?;
    let seat_tags = seats.iter().map(|seat| &seat.0);

    dbg!(&seat_tags);

    // let sessions = &proxy.list_sessions()?;

    // dbg!(sessions);

    let proxy2 = SeatManagerProxy::builder(&connection)
        .path("/org/freedesktop/login1/seat/seat0")?
        .build()
        .unwrap();

    dbg!(proxy2.active_session()?);

    let initial_devices = seat_tags.iter().map(|seat| {
        let &mut enumerator = enumerate_keyboards_for_seat(seat);
        for (device in enumerator.scan_devices().unwrap()) {

        }
    });

    poll::poll(|event| {
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

    let mut seat0_devices = enumerate_keyboards_for_seat("seat0")?;
    let mut seat1_devices = enumerate_keyboards_for_seat("seat1")?;

    println!("seat0 devices:");
    print_devices(&mut seat0_devices.scan_devices().unwrap());

    println!("seat1 devices:");
    print_devices(&mut seat1_devices.scan_devices().unwrap());

    Ok(())
}

fn enumerate_keyboards_for_seat(seat: &str) -> Result<Enumerator, std::io::Error> {
    let mut enumerator = udev::Enumerator::new()?;

    enumerator.match_subsystem("input")?;
    enumerator.match_property("ID_INPUT_KEYBOARD", "1")?;
    enumerator.match_tag(if seat.eq("seat0") { "seat" } else { seat })?;

    Ok(enumerator)
}

fn print_devices(devices: &mut udev::List<Enumerator, Device>) {
    for device in devices {
        println!("device: {:?}", device.syspath());
        println!("  attributes:");
        for attribute in device.attributes() {
            println!("    {:?} = {:?}", attribute.name(), attribute.value());
        }
        println!("  properties:");
        for property in device.properties() {
            println!("    {:?} = {:?}", property.name(), property.value());
        }
    }
}
