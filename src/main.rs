use futures::prelude::*;
use std::convert::TryInto;
use zbus::azync::Connection;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_channel::{Sender, Receiver};

mod devices_task;

mod messages;
use messages::DeviceChange;

// type Db = Arc<Mutex<HashMap<String, String>>>;

// fn set_return_type<T, F: Future<Output = T>>(_arg: &F) {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    let (sender, receiver) = async_channel::unbounded::<DeviceChange>();


    tokio::spawn(async move {
        devices_task::listen(sender.clone());
    });

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
