# MSDP - Multicast Server Discovery Protocol (WIP)

## Introduction

MSDP is a in development multicast server discovery protocol project that was developed by a student studying networking  
Its purpose is to discover all server that are running the MSDP server on the local subnet and report some system information like uptime and load  

Example proxy client output:

```bash
----- Management VLAN: server1.example.net:10002 -----
Entry: syslog.example.net (e8fbe7f7-836e-48ed-ac75-cdbad1a61ecc)
 ▪ Address: 10.0.0.149
 ▪ Platform: Linux
 ▪ System version: 5.14.0-503.38.1.el9_5.x86_64
 ▪ Keepalive timer: 5s
 ▪ Last seen: 00:00:06
 ▪ Uptime: 5d 21:25:11
 ▪ Load: 0.00 0.00 0.00
Entry: monitoring.example.net (17cefdb0-e665-4942-bd13-11c67571a730)
 ▪ Address: 10.0.0.151
 ▪ Platform: Linux
 ▪ System version: 4.18.0-553.32.1.el8_10.x86_64
 ▪ Keepalive timer: 5s
 ▪ Last seen: 00:00:06
 ▪ Uptime: 5d 21:25:43
 ▪ Load: 0.11 0.60 0.70
Entry: proxy.example.net (819ce29d-b878-4f81-9fbf-0e4093a51923)
 ▪ Address: 10.0.0.148
 ▪ Platform: Linux
 ▪ System version: 5.14.0-503.38.1.el9_5.x86_64
 ▪ Keepalive timer: 5s
 ▪ Last seen: 00:00:06
 ▪ Uptime: 5d 21:25:19
 ▪ Load: 0.00 0.00 0.00
Entry: dns.example.net (1019dcd0-c94f-43c5-8ac8-41b031c02d73)
 ▪ Address: 10.0.0.152
 ▪ Platform: Linux
 ▪ System version: 5.14.0-503.35.1.el9_5.x86_64
 ▪ Keepalive timer: 5s
 ▪ Last seen: 00:00:02
 ▪ Uptime: 5d 21:25:58
 ▪ Load: 0.00 0.00 0.00
Entry: vpn.example.net (d86160c4-ecd6-42fd-8fc4-60124b994180)
 ▪ Address: 10.0.0.147
 ▪ Platform: Linux
 ▪ System version: 6.1.0-34-amd64
 ▪ Keepalive timer: 5s
 ▪ Last seen: 00:00:02
 ▪ Uptime: 2d 12:27:42
 ▪ Load: 0.00 0.00 0.00

----- Data VLAN: localhost:10002 -----
Entry: dev.example.net (03cbe184-83a2-468b-a772-1c3602e3eaf5)
 ▪ Address: 10.0.0.10
 ▪ Platform: Linux
 ▪ System version: 5.14.0-503.38.1.el9_5.x86_64
 ▪ Keepalive timer: 5s
 ▪ Last seen: 00:00:05
 ▪ Uptime: 5d 20:32:00
 ▪ Load: 0.01 0.12 0.08
```

## Components

![Component diagram](MSDP.svg)

This project has 4 main components:

- Client - client, connects to a server (locally) and shows neighbors of the server
- Proxy client - proxy client, connects to proxy and shows neighbors of the proxy server, can connect to multiple proxies (aggregates neighbors from multiple proxies), current implementation is only raw TCP
- Server - server daemon, handles all the multicast messages, runs the MSDP protocol, listens locally for connections from clients
- Proxy - works the same as a daemon but also listens for connections from proxy clients, currently has 2 implementations, raw TCP and HTTP

These components are implemented in these files:

- Client
  - Python: python/control_client.py
  - Rust: rust/src/bin/client.rs
- Proxy client
  - Python: python/proxy_client.py
  - Rust: rust/src/bin/proxy_client.rs
- Server
  - Python: python/server.py
  - Rust: rust/src/bin/server.rs
- Proxy
  - Python: python/proxy.py - Implements HTTP
  - Rust: rust/src/bin/http_proxy.rs - Implements HTTP
  - Rust: rust/src/bin/raw_proxy.rs - Implements raw TCP sockets

## Usage

The client is currently implemented in Rust and Python

You can run python files using your python interpreter:
```bash
python3 python/proxy.py
```

Rust programs need to be compiled:
```bash
cargo build --release
```
