---
sidebar_position: 4
title: Test and Run
description: How to test and run your plugin
---

# Running your plugin

When you create a plugin, jilebi adds it to the current set of plugins. If you make any changes with envs, secrets or permissions, or introduce new resources, tools or prompts - use the command

```bash
jilebi plugins setup <name>
```

to set it up for testing. If you want to disable the plugin for some reason, you can comment it in the plugins.toml file which can be found at 

- windows: C:\Users\<user>\AppData\Roaming\jilebi\jilebi-server\data\plugins
- linux: .local/share/jilebi-server/plugins/
- macOS: ~/Library/Application\ Support/ai.jilebi.jilebi-server/plugins