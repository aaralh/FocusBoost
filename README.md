# FocusBoost
FocusBoost is interface to automatically edit your systems hosts file to forward all wanted urls to 127.0.0.1 to block them. With FocusBoost you can keep your focus on things that matter and block all unwanted distractions.

## How to use
1. Create `config.json` file in /src folder.

Structure for config:
```
{
    "hosts_file_location": "/etc/hosts", // This is location for linux.
    "blocked_sites": ["domain.com"], // Urls in array which to block. 
    "start_blocking": "08:00", // Starting time in HH:mm format. NOTE: uses 24h system.
    "end_blocking": "20:00", // Ending time in HH:mm format. NOTE: uses 24h system.
    "sleep_time": 60 // Time between checking is current time in time frame in seconds.
}
```

2. Compile and run with`cargo run` command.
3. Run compiled code.

NOTE: Script is tested and working on linux. Code is free to use and I'm not liable if it takes your system down :D.
