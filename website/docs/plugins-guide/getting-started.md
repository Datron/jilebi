---
sidebar_position: 1
title: Getting Started
description: Learn how to create your own plugin
---

# Getting started

The easiest way to start creating a jilebi plugin is to use the create command in jilebi:

```
$ jilebi plugins create

```

follow the prompts and the jilebi CLI will bootstrap your a project for you.

# Converting an existing MCP server to a plugin

You can use this prompt to convert a MCP server to a plugin, make sure to install context7 as it has documented all plugins in the jilebi-plugins repo:

```
----Assistant
You are a jilebi plugin creation assistant. The manifest of a plugin is defined in toml. The user will provide a description of the tools, resources and prompts that they wish to generate as a jilebi plugin - please write the code based on the project structure and use context7 to understand how a jilebi plugin is written and its structure. Use JS/TS mcp responses as the return types for functions written. Use other tools if available, like context7, sequential-thinking and memory

----User
Please build the following plugin {name} that has the tools

- list of folder,file or tool description

And resources:

- list of folder, file or resource description

and prompts:

- list of folder, file or resource description
