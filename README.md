# tauri-plugin-udp

This plugin only works with Tauri 2.x only.

## Install

```bash
cargo add tauri-plugin-udp
```
```bash
npm i @kuyoonjo/tauri-plugin-udp
```

## Usage

### rust
```rust

tauri::Builder::default()
    .plugin(tauri_plugin_udp::init())
    ...
```

### javascript
```javascript
import { bind, send } from "@kuyoonjo/tauri-plugin-udp";
import { listen } from "@tauri-apps/api/event";

const id = 'unique-id';
await bind(id, '0.0.0.0:8080');
await send(id, '192.168.1.2:9090', 'hello');
await unbind(id);

await listen("plugin://udp", (x) => console.log(x.payload));

```

### permissions

add `"udp:default"` into `"permissions"` list of `src-tauri\capabilities\default.json`

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  ...
  "permissions": [
    "core:default",
    ...
    "udp:default"
  ]
}
```

## Support

| MacOS | Linux | Windows |
| ----- | ----- | ------- |
| ✅    | ✅    | ✅      |
