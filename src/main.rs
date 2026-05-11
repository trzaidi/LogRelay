mod nmea;
mod serial_reader;
mod logger;

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
    heading_deg: f64,
    signal_dbm: i32,
}

// Parses a telemetry packet string into a Telemetry struct.
fn parse_packet(packet: &str) -> Result<Telemetry, String> {
    let parts: Vec<&str> = packet.split(',').collect();

    if parts.len() != 8 {
        return Err(format!(
            "bad packet: expected 8 fields, got {}",
            parts.len()
        ));
    }

    Ok(Telemetry {
        packet_id: parts[0].parse().map_err(|_| "invalid packet id")?,
        timestamp: parts[1].to_string(),
        lat: parts[2].parse().map_err(|_| "invalid latitude")?,
        lon: parts[3].parse().map_err(|_| "invalid longitude")?,
        alt_ft: parts[4].parse().map_err(|_| "invalid altitude")?,
        speed_kt: parts[5].parse().map_err(|_| "invalid speed")?,
        heading_deg: parts[6].parse().map_err(|_| "invalid heading")?,
        signal_dbm: parts[7].parse().map_err(|_| "invalid signal strength")?,
    })
}

// Simulates live telemetry ingestion and flight monitoring.
fn main() {
    let gpgga_sentence =
        "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";

    match nmea::parse_gpgga(gpgga_sentence) {
        Ok(fix) => {
            println!("==============================");
            println!("NMEA GPGGA FIX");
            println!("UTC: {}", fix.utc_time);
            println!("LAT: {} {}", fix.latitude_raw, fix.latitude_dir);
            println!("LON: {} {}", fix.longitude_raw, fix.longitude_dir);
            println!("FIX: {}", fix.fix_quality);
            println!("SATS: {}", fix.satellites);
            println!("ALT: {:.1} m", fix.altitude_m);
            
            if let Err(e) = logger::log_gpgga(
                &fix.utc_time,
                fix.latitude_deg,
                fix.longitude_deg,
                fix.altitude_m,
                fix.satellites,
                fix.fix_quality,
            )   {
            eprintln!("LOGGER ERROR: {}", e);
            }   
        }
        Err(e) => {
            eprintln!("NMEA ERROR: {}", e);
        }
        
    }

    // Simulated incoming telemetry packets.
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
                println!("PKT: {}", t.packet_id);
                println!("TIME: {}", t.timestamp);
                println!("LAT: {:.5}", t.lat);
                println!("LON: {:.5}", t.lon);
                println!("ALT: {:.1} ft", t.alt_ft);
                println!("SPD: {:.1} kt", t.speed_kt);
                println!("HDG: {:.1} deg", t.heading_deg);
                println!("SIG: {} dBm", t.signal_dbm);

                if let Some(prev_alt) = previous_altitude {
                    let altitude_loss = prev_alt - t.alt_ft;

                    // Detect significant altitude loss between updates.
                    if altitude_loss > 200.0 {
                        println!("ALERT: rapid altitude loss detected");
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


// Main will be condensed to the below or the like once we start ingesting live Orin data from the serial port. The above is just a simulation of the telemetry processing flow.
/*
fn main() {
    serial_reader::read_serial_lines("/dev/ttyUSB0", 9600)
        .expect("failed to start serial reader");
}
*/
