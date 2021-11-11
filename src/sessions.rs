use std::collections::HashMap;
use std::iter::Map;

//use async_channel::Sender;
use futures::TryFutureExt;
use zbus::SignalHandlerId;
use zbus::dbus_proxy;
use zbus::azync::Connection;
use zvariant::OwnedObjectPath;

use std::sync::mpsc::Sender;
use std::sync::mpsc::{channel, Receiver};

use crate::messages::SessionChange;

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
pub trait LoginManager {
    fn list_users(&self) -> zbus::Result<Vec<(u32, String, OwnedObjectPath)>>;

    fn list_sessions(&self) -> zbus::Result<Vec<(String, u32, String, String, OwnedObjectPath)>>;

    fn get_session(&self, session_id: &str) -> zbus::Result<OwnedObjectPath>;

    fn list_seats(&self) -> zbus::Result<Vec<(String, OwnedObjectPath)>>;

    fn get_seat(&self, seat_id: &str) -> zbus::Result<OwnedObjectPath>;

    #[dbus_proxy(signal)]
    fn session_new(&self, session_id: &str, object_path: OwnedObjectPath) -> zbus::Result<()>;
}

pub struct LoginProxy<'a>(LoginManagerProxy<'a>);

impl<'a> LoginProxy<'a> {
    #[inline]
    pub fn new(conn: &Connection) -> zbus::Result<Self> {
        Ok(LoginProxy(LoginManagerProxy::new(conn)?))
    }

    #[inline]
    pub fn proxy(&self) -> &LoginManagerProxy<'a> {
        &self.0
    }

    #[inline]
    pub fn connect_session_new(&self, sender: Sender<SessionChange>) -> zbus::fdo::Result<SignalHandlerId> {
        self.0
            .connect_session_new(move |session_id: &str, object_path: OwnedObjectPath| {
                sender
                    .send(SessionChange::Added {
                        session_id: session_id.to_owned(),
                    })
                    .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))?;
                Ok(())
            })
    }
}

#[dbus_proxy(
    interface = "org.freedesktop.login1.Seat",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1/seat"
)]
pub trait SeatManager {
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
