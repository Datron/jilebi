---
sidebar_position: 5
title: Logging and debugging
description: How to see jilebi and plugin logs
---

# Logging and Debugging

Logs outputted by jilebi are outputted to the file `jilebi.log` and logs from plugins are written to `<plugin_name>.logs`. These files can be found at:

- windows: C:\Users\<user>\AppData\Roaming\jilebi\jilebi-server\data\logs
- linux: .local/share/jilebi-server/logs/
- macOS: ~/Library/Application\ Support/ai.jilebi.jilebi-server/logs

You can view jilebi logs either by reading the file in your favourite text editor or by running:

```bash
jilebi log
```

and similarily for plugins:

```bash
jilebi plugins log <plugin_id>
```

# Writing logs in your Plugin

You can write logs in JS/TS using `console`:
```js
console.log
console.warn
console.debug
console.error
console.info
```