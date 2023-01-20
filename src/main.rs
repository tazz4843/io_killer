//! Small Rust app to force a system reboot if load average is too high.
//! This assumes a system, configured to immediately reboot when
//! PID 1 receives SIGINT (default systemd behaviour).

#[cfg(not(target_os = "linux"))]
compile_error!("This program only works on Linux systems.");

fn main() {
    // force this process to have a very high priority so that it can
    // run even if the system is under heavy load
    unsafe {
        libc::nice(-40);
    }
    // check if we got EPERM, which means we don't have permission to
    // lower the priority of this process
    let last_error = std::io::Error::last_os_error()
        .raw_os_error()
        .expect("infallible");
    if last_error == libc::EPERM {
        eprintln!("could not lower priority of process, are you root?");
        std::process::exit(2);
    }

    // check we have permissions to send signals to PID 1 by sending signal 0
    // (which does nothing but check permissions)
    if unsafe { libc::kill(1, 0) } != 0 {
        eprintln!("could not send signal to PID 1, do you have permissions to kill PID 1?");
        std::process::exit(3);
    }

    // check our first command line argument to see the default load average to
    // trigger a reboot at
    let load_average_target = match std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<f64>().ok()) {
        Some(load_average) => load_average,
        None => {
            eprintln!("usage: {} LOAD_AVERAGE", std::env::args().nth(0).unwrap());
            std::process::exit(4);
        }
    };

    // enter main loop
    loop {
        // get load average
        let avg = get_load_average();
        // check if 15m load average is too high
        if avg[2] > load_average_target {
            // reboot system
            unsafe {
                libc::kill(1, libc::SIGINT);
            }
        }
        // sleep for 10 seconds
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

fn get_load_average() -> [f64; 3] {
    let mut loadavg = [0.0; 3];
    unsafe {
        libc::getloadavg(loadavg.as_mut_ptr(), 3);
    }
    loadavg
}
