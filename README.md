# io_killer
Simple tool to force a Linux system to reboot if load average is too high for too long.

# Why?
I've got a system that tends to lock up very quickly if IO is touched just a bit too much.
I could spend days debugging the issue (and I have!), or I could just make this.

* Zero external dependencies (besides libc)
* Very small executable (339KB as of 01.19.2023)
* Next to zero footprint in memory (130KB)

# Details
* If 15 minute load average is above threshold, send SIGINT to PID 1 and exit
* Threshold is defined as the first argument to the program (no default)

# Usage
`cargo build --release`, copy executable to /usr/bin, and copy `io_killer.service` to `/etc/systemd/system/`.

The unit file will start the service as early as possible in boot, and will restart it if it exits.

# License
Unlicense
