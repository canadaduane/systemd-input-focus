Requirements: systemd, systemd-logind (i.e. access to loginctl)

List all DISPLAY env vars on all processes:
https://unix.stackexchange.com/a/308985
    $ ps e
    $ strings /proc/[PID]/environ
- Look for DISPLAY env var
- Also of interest may be HOME env var

Show all X windows & details:
$ xlsclients -al
- does this really show ALL users' windows, or just current user?


So we can get a list of all local DISPLAYs, but how do we know which one is "active", i.e. the one that the keyboard is actively typing to?


$ loginctl list-sessions
SESSION  UID USER  SEAT  TTY
     13 1001 rella seat0 tty3
      2 1000 duane seat0 tty2

$ loginctl session-status 2
2 - duane (1000)
           Since: Sat 2021-07-31 15:17:35 MDT; 20h ago
          Leader: 1509 (gdm-session-wor)
            Seat: seat0; vc2
             TTY: tty2
         Service: gdm-password; type x11; class userN
           State: active
            Unit: session-2.scope
                  ├─1509 gdm-session-worker [pam/gdm-password]
                  ├─1546 /usr/bin/gnome-keyring-daemon --daemonize --login
                  ├─1703 /usr/libexec/gdm-x-session --run-script env GNOME_SHELL_SESSION_MODE=pop /usr/bin/gnome-session --session=pop
                  ├─1706 /usr/lib/xorg/Xorg vt2 -displayfd 3 -auth /run/user/1000/gdm/Xauthority -nolisten tcp -background none -noreset -keeptty -novtswitch -verbose 3
                  ├─2205 /usr/libexec/gnome-session-binary --systemd --session=pop
                  └─7721 /usr/bin/ssh-agent -D -a /run/user/1000/keyring/.ssh

The Xauthority file is in an interesting spot:
    /run/user/1000/gdm/Xauthority

List all devices attached to a "seat":
$ loginctl seat-status seat0
seat0
        Sessions: 13 *2
         Devices:
                  ├─/sys/devices/LNXSYSTM:00/LNXPWRBN:00/input/input2
                  │ input:input2 "Power Button"
                  ├─/sys/devices/LNXSYSTM:00/LNXSYBUS:00/PNP0C0C:00/input/input1
                  │ input:input1 "Power Button"
                  ...

Detect what TTY I am:
https://askubuntu.com/questions/902998/how-to-check-which-tty-im-currently-using
e.g. readlink /proc/self/fd/0


procps/w.c source code:
https://gitlab.com/procps-ng/procps/-/blob/master/w.c


Interception plugins:
https://gitlab.com/interception/linux/tools
https://github.com/kbairak/s2arrows
https://github.com/maricn/interception-vimproved
https://github.com/zsugabubus/interception-k2k
