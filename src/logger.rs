use std::fs::OpenOptions;
use std::io::Write;

// Appends telemetry data to a CSV log file
pub fn log_gpgga(
    utc_time: &str,
    latitude: f64,
    longitude: f64,
    altitude_m: f64,
    satellites: u8,
    fix_quality: u8,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("flight_log.csv")?;

    writeln!(
        file,
        "{},{:.6},{:.6},{:.1},{},{}",
        utc_time,
        latitude,
        longitude,
        altitude_m,
        satellites,
        fix_quality
    )?;

    Ok(())
}