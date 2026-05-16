# LogRelay

Rust telemetry relay for Group 1-3 UAS.
Parses MAVLink from ArduPilot, logs flight data, and publishes vehicle entities to Anduril Lattice.
Runs on NVIDIA Jetson Orin.

## Telemetry Format

packet_id,timestamp,latitude,longitude,altitude_ft,speed_kt,heading_deg,signal_dbm,1001,22:14:01,40.7128,-74.0060,1200,85,270,-61

## Roadmap

- [x] NMEA/GPGGA parsing
- [x] CSV flight logger
- [x] Simulated telemetry with altitude alerting
- [ ] MAVLink parser
- [ ] Multi-vehicle tracking
- [ ] HTTP/WebSocket server
- [ ] Jetson Orin deployment via [logrelay-nix](https://github.com/trzaidi/LogRelay-Nix)
- [ ] Anduril LatticeOS entity publisher - requires SDK access

## Related

[logrelay-nix](https://github.com/trzaidi/LogRelay-Nix)
[jetpack-nixos](https://github.com/anduril/jetpack-nixos)
[Lattice SDK](https://developer.anduril.com)

## License

MIT
