---
sidebar_position: 4
title: Application Contexts
description: Different tools for different AI applications
---

# Application Contexts

Jilebi lets you create an application context to limit the tools available to different AI applications. For example, you can expose context7 to Zed and wikipedia to Claude Desktop.

## Creating Contexts

To create a context, use the `jilebi context create` command. For example, to create a context named `ide` with access to the `context7` tool:

```bash
jilebi context create ide context7
```

Use the `jilebi context --help` command to see all available options