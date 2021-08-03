# systemd-input-focus

[WIP] A systemd service that reports the focused window and attached input devices for each active seat.

## Problem

Key remapping (keyboard input interceptor) tools in Linux often need to know two important pieces of information to remap keys:

1. Where is the input focused--in other words, what X window or app has the cursor where typing is happening? and
2. Who is typing?

Apps started on login can find this information through environment variables and (for example) an Xlib call to `XGetInputFocus`; however, if the keyboard interceptor service is written at a low level in the system stack--for example by monitoring the Linux input events (`libevdev`)--then "where is the input focused" and "who is typing" are harder to come by. A few of the challenges:

1. Even though a service running as `root` has access to the entire system, the `HOME`, `USER`, `DISPLAY`, and `XAUTHORITY` environment variables are either incorrect (it's very rare that the "root" user is the one actually doing the typing in X windows), or not set at all.
2. Without the `HOME` or `USER` variables set, there's no direct way to know "Who is typing?"
3. Without the `DISPLAY` or `XAUTHORITY` variables set, there's no way to call `XOpenDisplay` (and, subsequently, `XGetInputFocus`) on the active X server without guessing (e.g. ":0" is usually a good guess for the `DISPLAY`, but not always correct).

## Solution

We can use the [dbus](https://www.freedesktop.org/wiki/Software/systemd/dbus/) (specifically the [systemd-logind service](https://www.freedesktop.org/wiki/Software/systemd/logind/) on dbus) to gather information such as users, sessions, and seats, including who is actively logged in. Combined with [udev](https://wiki.debian.org/udev), we can find out what input devices (e.g. mouse, keyboard) are connected at any given time.

Complicating things somewhat, however, is Xorg's concept of [multiseat](https://wiki.archlinux.org/title/xorg_multiseat). Multiseat allows a single-CPU computer station to be set up with multiple keyboards and monitors--so that two or more people can use it at the same time. In order to know which hardware (keyboard, mouse, etc.) belongs to which "seat", the devices in the Linux `udev` system are tagged with the seat they belong to (e.g. `seat0`, `seat1`, etc.)

By tracking new input device when it is plugged in (via `udev`), we can watch for a `seat[N]` tag that corresponds to an active X session. By tracking new X sessions whenever a user logs in (via `dbus`), we can watch for the assignment of input devices to a user.

Thus, whenever a keyboard key is pressed or mouse movement is made, it's possible to connect all the dots and tag each keystroke or movement with the user that made it, and the active X window app that has focus in an active X session.
