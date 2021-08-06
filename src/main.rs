use std::convert::TryInto;
use tokio::runtime::Runtime;
use tokio::task;
use zbus::azync::Connection;

use std::sync::{Arc, Mutex};

use async_channel::{Receiver, Sender};

mod devices_task;

mod messages;
use messages::DeviceChange;

use futures_util::stream::StreamExt;
use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

// type Db = Arc<Mutex<HashMap<String, String>>>;

// fn set_return_type<T, F: Future<Output = T>>(_arg: &F) {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    let (sender, receiver) = async_channel::unbounded::<DeviceChange>();

    // devices_task::listen(sender.clone()).await;
    // tokio::spawn(async move {
    //     devices_task::listen(sender.clone()).await;
    //     ()
    // });

    let local = tokio::task::LocalSet::new();

    local
        .run_until(async move {
            let builder = MonitorBuilder::new()
                .expect("Couldn't create builder")
                .match_subsystem("input")
                .expect("Failed to add filter for input devices");

            let monitor: AsyncMonitorSocket = builder
                .listen()
                .expect("Couldn't listen to MonitorSocket")
                .try_into()
                .expect("Couldn't create AsyncMonitorSocket");

            let t1 = task::spawn_local(async move {
                monitor
                    .for_each(|event| async {
                        if let Ok(event) = event {
                            sender
                                .send(DeviceChange::Added {
                                    syspath: event.device().syspath().to_owned(),
                                })
                                .await
                                .expect("unable to send");
                        }
                        ()
                    })
                    .await;
            });

            let t2 = task::spawn_local(async move {
                while let Ok(message) = receiver.recv().await {
                    println!("GOT = {:?}", message);
                }
            });

            t1.await.expect("udev monitor stopped");
            t2.await.expect("message loop stopped");
        })
        .await;

    // let conn = zbus::azync::Connection::session().await?;

    // let t = {
    //     let conn = conn.clone();

    //     async fn ztask(conn: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
    //         loop {
    //             while let Some(item) = conn.try_next().await? {
    //                 dbg!(item);
    //             }
    //         }
    //     }

    //     tokio::task::spawn(async move { ztask(&mut conn.clone()) });
    // };

    Ok(())
}
