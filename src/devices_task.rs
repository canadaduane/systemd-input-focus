use std::convert::TryInto;

use async_channel::Sender;
use futures::future::ready;
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

    monitor.for_each(|event| async {
        if let Ok(event) = event {
            sender.send(DeviceChange::Added {
                syspath: event.device().syspath().to_owned(),
            }).await.expect("unable to send");
        }
        ()
    }).await;

    /*
    monitor
        .for_each_concurrent(1, |event| async {
            if let Ok(event) = event {
                match event.event_type() {
                    tokio_udev::EventType::Add => sender
                        .send(DeviceChange::Added {
                            syspath: event.device().syspath().to_owned(),
                        })
                        .await
                        .expect("Failed to send udev"),
                    tokio_udev::EventType::Remove => sender
                        .send(DeviceChange::Removed {
                            syspath: event.device().syspath().to_owned(),
                        })
                        .await
                        .expect("Failed to send udev"),
                    _ => (),
                }
            }
        })
        .await;
        */
}
