# Fake Minecraft Server

[![CI](https://github.com/OreQr/fake-minecraft-server/actions/workflows/ci.yml/badge.svg)](https://github.com/OreQr/fake-minecraft-server/actions/workflows/ci.yml)

Lightweight Rust implementation of a fake Minecraft server. It supports server list responses (status/ping) and connection attempts, useful for testing clients or as a placeholder server.

<img src="https://github.com/user-attachments/assets/829ff354-60a0-41d8-8de5-3d6863de6b19" alt="Server list" width="600"/>
<br>
<img src="https://github.com/user-attachments/assets/a0f20c5a-36a7-4827-b2cd-3a5677de48db" alt="Connection attempt" width="400"/>

## Configuration

Server uses a YAML configuration file (`config.yml`). If no configuration is found, default values will be used.

You can specify a custom path to the configuration file by setting the `CONFIG_PATH` environment variable.

Example configuration file with comments is provided in [`config.example.yml`](https://github.com/OreQr/fake-minecraft-server/blob/master/config.example.yml).

### Default values:

```yaml
debug: false

host: 127.0.0.1
port: 25565

status:
  version:
    same: true

  players:
    max: 20
    online: 0

  motd: "§cFake Minecraft Server"
  favicon: server-icon.png

kick_message: "§c§lThis is a fake server!\n§eIt only responds to ping requests."
```

## Debugging

To enable debug logging, you can either:

1. Set the environment variable:

   ```bash
   DEBUG=true
   # or Rust lang
   RUST_LOG="debug"
   ```

2. Or enable debug in the config file:

   ```yaml
   debug: true
   ```

## License

[MIT](https://github.com/OreQr/fake-minecraft-server/blob/master/LICENSE)
