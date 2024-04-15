# Raunch

A safe wrapper around launchd `launch_activate_socket`.

```rust,no_run
#[cfg(target_os = "macos")]
let descriptors = raunch::activate_socket("socket-name").expect("activation to work");
```

The name of the socket (here `socket-name`) needs to match the socket name in the `plist` file:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>Label</key>
	<string>com.example.agent</string>
	<key>OnDemand</key>
	<true/>
	<key>ProgramArguments</key>
	<array>
		<string>example-agent</string>
	</array>
	<key>RunAtLoad</key>
	<true/>
	<key>Sockets</key>
	<dict>
		<key>socket-name</key>
		<dict>
			<key>SockPathName</key>
			<string>/Users/test/.openpgp-card-ssh-agent</string>
			<key>SockFamily</key>
			<string>Unix</string>
		</dict>
	</dict>
</dict>
</plist>
```

The file can be placed in `~/Library/LaunchAgents` and loaded via `launchctl load ~/Library/LaunchAgents/com.example.agent.plist`.
