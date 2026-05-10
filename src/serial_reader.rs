use std::io::{BufRead, BufReader};
use std::time::Duration;

// Reads NMEA-style telemetry lines from a serial device.
pub fn read_serial_lines(port_name: &str, baud_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()?;

    let reader = BufReader::new(port);

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("$GPGGA") {
            println!("NMEA: {}", line);
        }
    }

    Ok(())
}
