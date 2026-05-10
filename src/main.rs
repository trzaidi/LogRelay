use std::{thread, time};

// Stores parsed drone telemetry data.
#[derive(Debug)]
struct Telemetry {
    lat: f64,
    lon: f64,
    alt_ft: f64,
    speed_kt: f64,
}

// Parses a telemetry packet string into a Telemetry struct.
fn parse_packet(packet: &str) -> Result<Telemetry, String> {
    let parts: Vec<&str> = packet.split(',').collect();

    if parts.len() != 4 {
        return Err(format!("bad packet: expected 4 fields, got {}", parts.len()));
    }

    Ok(Telemetry {
        lat: parts[0].parse().map_err(|_| "invalid latitude")?,
        lon: parts[1].parse().map_err(|_| "invalid longitude")?,
        alt_ft: parts[2].parse().map_err(|_| "invalid altitude")?,
        speed_kt: parts[3].parse().map_err(|_| "invalid speed")?,
    })
}

// Simulates live telemetry ingestion and basic flight monitoring.
fn main() {
    // Simulated incoming telemetry packets in the format: "lat,lon,alt_ft,speed_kt" (NMEA-like).
    let packets = vec![
        "40.7128,-74.0060,1200,85",
        "40.7130,-74.0058,1180,84",
        "40.7135,-74.0055,900,95",
        "40.7140,-74.0050,400,120",
    ];

    let mut previous_altitude: Option<f64> = None;

    for packet in packets {
        match parse_packet(packet) {
            Ok(t) => {
                println!("==============================");
                println!("TRACK UPDATE");
                println!("LAT: {:.5}", t.lat);
                println!("LON: {:.5}", t.lon);
                println!("ALT: {:.1} ft", t.alt_ft);
                println!("SPD: {:.1} kt", t.speed_kt);

                if let Some(prev_alt) = previous_altitude {
                    let altitude_loss = prev_alt - t.alt_ft;
                    
                    // Detect significant altitude drop in between updates.
                    if altitude_loss > 200.0 {
                        println!("ALERT: Rapid Altitude Loss detected");
                    }
                }

                previous_altitude = Some(t.alt_ft);
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
            }
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}