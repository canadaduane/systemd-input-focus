extern crate libc;
extern crate udev;

use std::io;
use std::ptr;
use std::thread;
use std::time::Duration;

use std::os::unix::io::AsRawFd;

use libc::{c_int, c_short, c_ulong, c_void};
use udev::Event;

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct sigset_t {
    __private: c_void,
}

#[allow(non_camel_case_types)]
type nfds_t = c_ulong;

const POLLIN: c_short = 0x0001;

extern "C" {
    fn ppoll(
        fds: *mut pollfd,
        nfds: nfds_t,
        timeout_ts: *mut libc::timespec,
        sigmask: *const sigset_t,
    ) -> c_int;
}

pub fn poll(action: impl Fn(Event)) -> io::Result<()> {
    let mut socket = udev::MonitorBuilder::new()?
        .match_subsystem("input")?
        .listen()?;

    let mut fds = vec![pollfd {
        fd: socket.as_raw_fd(),
        events: POLLIN,
        revents: 0,
    }];

    loop {
        let result = unsafe {
            ppoll(
                (&mut fds[..]).as_mut_ptr(),
                fds.len() as nfds_t,
                ptr::null_mut(),
                ptr::null(),
            )
        };

        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        let event = match socket.next() {
            Some(evt) => action(evt),
            None => {
                thread::sleep(Duration::from_millis(10));
                continue;
            }
        };
    }
}
