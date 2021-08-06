use std::convert::TryInto;

use async_channel::Sender;
use futures_util::future::ready;
use futures_util::stream::StreamExt;
use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

use crate::messages::DeviceChange;

pub async fn listen(sender: Sender<DeviceChange>) {
    let builder = MonitorBuilder::new()
        .expect("Couldn't create builder")
        .match_subsystem("input")
        .expect("Failed to add filter for input devices");

    let monitor: AsyncMonitorSocket = builder
        .listen()
        .expect("Couldn't listen to MonitorSocket")
        .try_into()
        .expect("Couldn't create AsyncMonitorSocket");

    monitor
        .for_each_concurrent(1, |event| async {
            if let Ok(event) = event {
                sender
                    .send(DeviceChange::Added {
                        syspath: event.device().syspath().to_owned(),
                    })
                    .await
                    .expect("Failed to send udev");
            }
            ()
        })
        .await;
}

// println!(
//     "Hotplug event: {}: {}",
//     event.event_type(),
//     event.device().syspath().display()
// );
