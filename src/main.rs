use std::{thread, time};

// Stores parsed drone telemetry data.
#[derive(Debug)]
struct Telemetry {
    packet_id: u32,
    timestamp: String,
    lat: f64,
    lon: f64,
    alt_ft: f64,
    speed_kt: f64,

}

// Parses a telemetry packet string into a Telemetry struct.
fn parse_packet(packet: &str) -> Result<Telemetry, String> {
    let parts: Vec<&str> = packet.split(',').collect();

    if parts.len() != 8 {
        return Err(format!("bad packet: expected 6 fields, got {}", parts.len()));
    }
    // Fields
    Ok(Telemetry {
        packet_id: parts[0].parse().map_err(|_| "Invalid packet id")?,
        timestamp: parts[1].to_string(),
        lat: parts[2].parse().map_err(|_| "Invalid latitude")?,
        lon: parts[3].parse().map_err(|_| "Invalid longitude")?,
        alt_ft: parts[4].parse().map_err(|_| "Invalid altitude")?,
        speed_kt: parts[5].parse().map_err(|_| "Invalid speed")?,
        heading_deg: parts[6].parse().map_err(|_| "invalid heading")?,
        signal_dbm: parts[7].parse().map_err(|_| "invalid signal strength")?,
    })
}

// Simulates live telemetry ingestion and basic flight monitoring.
fn main() {
    // Simulated incoming telemetry packets in the format: "lat,lon,alt_ft,speed_kt" (NMEA-like).
    let packets = vec![
    "1001,22:14:01,40.7128,-74.0060,1200,85,270,-61",
    "1002,22:14:02,40.7130,-74.0058,1180,84,271,-63",
    "1003,22:14:03,40.7135,-74.0055,900,95,273,-70",
    "1004,22:14:04,40.7140,-74.0050,400,120,278,-81",
    "1005,22:14:05,40.7145,-74.0046,380,122,281,-83",
    "1006,22:14:06,40.7152,-74.0041,350,126,285,-88",
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