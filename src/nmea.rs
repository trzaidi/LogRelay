// Represents parsed NMEA GPGGA GPS fix data.
#[derive(Debug)]
pub struct GpggaFix {
    pub utc_time: String,
    pub latitude_raw: String,
    pub latitude_dir: String,
    pub longitude_raw: String,
    pub longitude_dir: String,
    pub fix_quality: u8,
    pub satellites: u8,
    pub altitude_m: f64,
}

// Parses a NMEA GPGGA sentence into a GPS fix structure.
pub fn parse_gpgga(sentence: &str) -> Result<GpggaFix, String> {
    if !sentence.starts_with("$GPGGA") {
        return Err("not a GPGGA sentence".to_string());
    }

    let fields: Vec<&str> = sentence.split(',').collect();

    if fields.len() < 10 {
        return Err(format!("bad GPGGA sentence: expected at least 10 fields, got {}", fields.len()));
    }

    Ok(GpggaFix {
        utc_time: fields[1].to_string(),
        latitude_raw: fields[2].to_string(),
        latitude_dir: fields[3].to_string(),
        longitude_raw: fields[4].to_string(),
        longitude_dir: fields[5].to_string(),
        fix_quality: fields[6].parse().map_err(|_| "invalid fix quality")?,
        satellites: fields[7].parse().map_err(|_| "invalid satellite count")?,
        altitude_m: fields[9].parse().map_err(|_| "invalid altitude")?,
    })
}
