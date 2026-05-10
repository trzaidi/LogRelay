# Drone Tracker

Lightweight Rust-based drone telemetry tracker designed for parsing, monitoring, and simulating live flight data streams.

## Features

* Telemetry packet parsing
* Simulated live flight updates
* Altitude monitoring and alerting
* Timestamped telemetry packets
* Modular telemetry data structure
* Rust systems programming foundation

## Example Telemetry Packet

Format:
packet_id,timestamp,latitude,longitude,altitude_ft,speed_kt,heading_deg,signal_dbm

Data:
1001,22:14:01,40.7128,-74.0060,1200,85,270,-61


## Example Output

TRACK UPDATE

PKT: 1001
TIME: 22:14:01
LAT: 40.71280
LON: -74.00600
ALT: 1200.0 ft
SPD: 85.0 kt
HDG: 270.0 deg
SIG: -61 dBm

## Roadmap

* Serial telemetry ingestion
* GPS/NMEA integration
* MAVLink support
* Telemetry replay system
* Jetson edge deployment
* Live dashboard visualization
* RF signal quality metrics
* Packet loss simulation

## Tech Stack

* Rust
* Cargo
* VS Code
* Windows / Linux / macOS compatible

## Purpose

This project serves as a lightweight telemetry and flight-monitoring sandbox for learning:

* Systems programming
* Embedded software workflows
* Aerospace telemetry concepts
* Drone/UAS software development
* Mission systems engineering

## License

MIT
