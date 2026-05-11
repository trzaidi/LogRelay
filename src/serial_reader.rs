use std::io::{BufRead, BufReader};
use std::time::Duration;

use crate::logger;
use crate::nmea;

// Reads NMEA-style telemetry lines from a serial device
pub fn read_serial_lines(
    port_name: &str,
    baud_rate: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()?;

    let reader = BufReader::new(port);

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("$GPGGA") {
            println!("NMEA: {}", line);

            match nmea::parse_gpgga(&line) {
                Ok(fix) => {
                    println!("UTC: {}", fix.utc_time);
                    println!("LAT: {:.6}", fix.latitude_deg);
                    println!("LON: {:.6}", fix.longitude_deg);
                    println!("ALT: {:.1} m", fix.altitude_m);

                    if let Err(e) = logger::log_gpgga(
                        &fix.utc_time,
                        fix.latitude_deg,
                        fix.longitude_deg,
                        fix.altitude_m,
                        fix.satellites,
                        fix.fix_quality,
                    ) {
                        eprintln!("LOGGER ERROR: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("PARSE ERROR: {}", e);
                }
            }
        }
    }

    Ok(())
}