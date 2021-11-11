use async_channel::Sender;
use futures::TryStreamExt;
use tokio::task;

use crate::messages::DeviceChange;
use crate::sessions::LoginProxy;

pub async fn spawn_local(sender: Sender<DeviceChange>) -> tokio::task::JoinHandle<()> {
    let conn = zbus::azync::Connection::system()
        .await
        .expect("Couldn't connect to dbus");
    
    let login_proxy = LoginProxy::new(&conn);

    conn.call_method(
        Some("org.freedesktop.DBus"),
        "/org/freedesktop/DBus",
        Some("org.freedesktop.DBus.Monitoring"),
        "BecomeMonitor",
        &(&[] as &[&str], 0u32),
    )
    .await.expect("Couldn't call monitor method");

    task::spawn_local(async move {
        loop {
            while let Ok(Some(item)) = conn.clone().try_next().await {
                dbg!(item);
            }
        }
    })
}
