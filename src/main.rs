use futures::prelude::*;
use std::convert::TryInto;
use zbus::azync::Connection;

// fn set_return_type<T, F: Future<Output = T>>(_arg: &F) {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    let conn = zbus::azync::Connection::session().await?;

    let t = {
        let conn = conn.clone();

        async fn ztask(conn: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
            loop {
                while let Some(item) = conn.try_next().await? {
                    dbg!(item);
                }
            }
        }

        tokio::task::spawn(async move { ztask(&mut conn.clone())} );
        // tokio::task::spawn(async move {
        //     loop {
                // let op = "/org/freedesktop/ScreenSaver".try_into().unwrap();

                // let pred = |message: &zbus::Message| -> Result<bool, zbus::Error> {
                //     // check if message is our object
                //     let header = message.header()?;

                //     if header.message_type()? != zbus::MessageType::MethodCall {
                //         println!("type not right: {:?}", message);
                //         return Ok(false);
                //     }

                //     // is this an object we have?
                //     if header.path()? != Some(&op) {
                //         println!("path unhandled: {:?}", message);
                //         return Ok(false);
                //     }

                //     Ok(true)
                // };

                // let mut stream = conn.clone();
                // while let Some(item) = stream.try_next().await? {
                //     dbg!(item);
                //     // Ok(())
                // }
                // while let Some(item) = stream.next().await {
                //     dbg!(item);
                // }

                // let message = conn.receive_specific(|message| {
                //     ready(pred(message)).boxed()
                // }).await.unwrap();

                // let header = message.header().unwrap();

                // do we impliment this interface for that object? If not, return an error
                // if header.interface().unwrap() != Some("org.freedesktop.ScreenSaver") {
                //     println!("interface unhandled: {:?}", message);
                //     todo!();
                // }

                // does this interface impliment the requested method? If not, return an error
                // if header.member().unwrap() != Some("Inhbit") {
                //     todo!();
                // }

                // does the type signature match? alternately: does deserialization work?
                // "su"
                // let body: (String, u32) = message.body().unwrap();

                // println!("BODY: {:?}", body);
        //     }
        // })
    };

    conn.call_method(
        Some("org.freedesktop.DBus"),
        "/org/freedesktop/DBus",
        Some("org.freedesktop.DBus"),
        "RequestName",
        &("com.codyps.IdleMapper", 0u32),
    )
    .await?;

    println!("got name");

    // t.await?;

    Ok(())
}

/// # Examples
///
/// Here is how one would typically run the zbus executor through tokio's single-threaded
/// scheduler:
///
/// ```
/// use zbus::azync::Connection;
/// use tokio::runtime;
///
///# #[cfg(not(feature = "internal-executor"))]
/// runtime::Builder::new_current_thread()
///        .build()
///        .unwrap()
///        .block_on(async {
///     let conn = Connection::new_session().await.unwrap();
///     {
///        let conn = conn.clone();
///        tokio::task::spawn(async move {
///            loop {
///                conn.executor().tick().await;
///            }
///        });
///     }
///
///     // All your other async code goes here.
///
///     // Not needed for multi-threaded scheduler.
///     conn.shutdown().await;
/// });
/// ```
///
const A: u32 = 1;
