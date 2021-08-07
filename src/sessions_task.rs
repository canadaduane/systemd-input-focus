use std::convert::TryInto;

use futures::TryStreamExt;
use tokio::task;
use async_channel::Sender;
use futures_util::stream::StreamExt;
use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

use crate::messages::DeviceChange;

pub async fn spawn_local(sender: Sender<DeviceChange>) -> tokio::task::JoinHandle<()> {
    let conn = zbus::azync::Connection::system().await.expect("Couldn't connect to dbus");

    task::spawn_local(async move {
        loop {
            while let Ok(Some(item)) = conn.clone().try_next().await {
                dbg!(item);
            }
        }
    })
}
