
mod sessions;
mod devices;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::system()?;
    let sessions = sessions::list_sessions(&connection)?;

    dbg!(&sessions);

    let seats: Vec<String> = sessions.iter().map(|sess| sess.seat.clone()).collect();

    let devices = devices::list_keyboards(&seats)?;

    dbg!(&devices);

    return Ok(());

    // devices::poll(|event| {
    //     let device = event.device();
    //     if device_is_keyboard(&device) {
    //         println!("IS KEYBOARD");
    //     }

    //     if device_has_tag(&device, "seat") {
    //         println!("HAS SEAT");
    //     }

    //     println!(
    //         "{}: {} {} (subsystem={}, sysname={}, devtype={})",
    //         event.sequence_number(),
    //         event.event_type(),
    //         event.syspath().to_str().unwrap_or("---"),
    //         event
    //             .subsystem()
    //             .map_or("", |s| { s.to_str().unwrap_or("") }),
    //         event.sysname().to_str().unwrap_or(""),
    //         event.devtype().map_or("", |s| { s.to_str().unwrap_or("") })
    //     );
    // })?;

    Ok(())
}

