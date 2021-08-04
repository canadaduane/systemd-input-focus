use std::collections::HashMap;
use std::iter::Map;

use zbus::dbus_proxy;
use zbus::Connection;
use zvariant::OwnedObjectPath;


#[derive(Debug)]
pub struct Session {
    pub session_id: String,
    pub user_id: u32,
    pub user_name: String,
    pub seat: String,
    pub active: bool,
}

#[dbus_proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait LoginManager {
    fn list_users(&self) -> zbus::Result<Vec<(u32, String, OwnedObjectPath)>>;

    fn list_sessions(&self) -> zbus::Result<Vec<(String, u32, String, String, OwnedObjectPath)>>;

    fn get_session(&self, session_id: &str) -> zbus::Result<OwnedObjectPath>;

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

pub fn list_sessions(connection: &Connection) -> Result<Vec<Session>, Box<dyn std::error::Error>> {
    let login_manager = LoginManagerProxy::new(&connection)?;

    let mut active_session_by_seat: HashMap<String, bool> = HashMap::new();

    let session_results: Result<Vec<Session>, _> = login_manager
        .list_sessions()?
        .iter()
        .map(|sess| -> Result<Session, _> {
            let seat = sess.3.to_owned();

            if !active_session_by_seat.contains_key(&seat) {
                let seat_path = format!("/org/freedesktop/login1/seat/{}", &seat);
                let seat_manager = SeatManagerProxy::builder(&connection)
                    .path(seat_path)?
                    .build()?;

                let active_result = seat_manager.active_session()?;
                active_session_by_seat.insert(seat, active_result.0.eq(&sess.0));
            }

            return Ok(Session {
                session_id: sess.0.to_owned(),
                user_id: sess.1,
                user_name: sess.2.to_owned(),
                seat: sess.3.to_owned(),
                active: active_session_by_seat.get(&sess.3).unwrap().to_owned(),
            });
        })
        .collect();

    return session_results;
}
