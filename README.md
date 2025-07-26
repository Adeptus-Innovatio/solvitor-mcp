# 🦾💀 solvitor-mcp: A Solvitor API MCP Server in Rust

## Overview

A Model Context Protocol (MCP) server for interacting with the Solvitor API. This server provides tools to access the AI-powered platform that helps developers extract IDL files from closed-source Solana smart contracts and decompile them.

## Features

This MCP server exposes the following tools for interacting with the Solvitor API:

1. `decode`
   - Extract IDL (Interface Definition Language) from any Solana program using reverse engineering techniques
   - Inputs:
     - `program_id` (string): Base58-encoded Solana program address (32-44 characters)
     - `url` (string, optional): Solana RPC endpoint URL (defaults to mainnet-beta)
   - Returns: IDL structure with program metadata and type ("anchor" or "native")

## Installation

Prerequisites:
- Rust toolchain (install via [rustup](https://rustup.rs/)) – for regular usage
- Solvitor API key. You can obtain one [here](https://solvitor.xyz/developer-settings) for free.

### Regular Usage

```bash
cargo install solvitor-mcp

where solvitor-mcp # -> /Users/$username/.cargo/bin/solvitor-mcp
```

Add the following to your `claude_desktop_config.json` or `claude_config.json`:

```json
{
  "mcpServers": {
    "solvitor-mcp": {
      "command": "/Users/$username/.cargo/bin/solvitor-mcp",
      "args": [],
      "env": {
        "SOLVITOR_API_KEY": "your_solvitor_api_key"
      }
    }
  }
}
```


### Remote MCP

Need remote MCP? Ping us on Twitter, we'll build the feature in a couple of days: [x.com/solvitor_xyz](https://x.com/solvitor_xyz)


## License

MIT
