use tokio::task;

mod devices_task;
mod sessions_task;

mod messages;
use messages::DeviceChange;

mod sessions;
use sessions::LoginManagerProxy;

// type Db = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    let (sender, receiver) = async_channel::unbounded::<DeviceChange>();

    let local = tokio::task::LocalSet::new();

    local
        .run_until(async move {
            let t1 = devices_task::spawn_local(sender.clone());

            let t2 = sessions_task::spawn_local(sender.clone());

            let t3 = task::spawn_local(async move {
                while let Ok(message) = receiver.recv().await {
                    println!("GOT = {:?}", message);
                }
            });

            t1.await;
            t2.await;
            t3.await.expect("message loop stopped");
        })
        .await;

    Ok(())
}
