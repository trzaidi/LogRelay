// Represents parsed NMEA GPGGA GPS fix data
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
    pub latitude_deg: f64,
    pub longitude_deg: f64,
}

// Converts raw NMEA coordinates into decimal degrees
fn nmea_coord_to_decimal(raw: &str, direction: &str) -> Result<f64, String> {
    let value: f64 = raw.parse().map_err(|_| "invalid coordinate")?;

    let degrees = (value / 100.0).floor();
    let minutes = value - (degrees * 100.0);

    let mut decimal = degrees + (minutes / 60.0);

    if direction == "S" || direction == "W" {
        decimal *= -1.0;
    }

    Ok(decimal)
}

// Parses a NMEA GPGGA sentence into a GPS fix structure
pub fn parse_gpgga(sentence: &str) -> Result<GpggaFix, String> {
    if !sentence.starts_with("$GPGGA") {
        return Err("not a GPGGA sentence".to_string());
    }

    let fields: Vec<&str> = sentence.split(',').collect();

    if fields.len() < 10 {
        return Err(format!(
            "bad GPGGA sentence: expected at least 10 fields, got {}",
            fields.len()
        ));
    }

    let latitude_deg = nmea_coord_to_decimal(fields[2], fields[3])?;
    let longitude_deg = nmea_coord_to_decimal(fields[4], fields[5])?;

    Ok(GpggaFix {
        utc_time: fields[1].to_string(),
        latitude_raw: fields[2].to_string(),
        latitude_dir: fields[3].to_string(),
        longitude_raw: fields[4].to_string(),
        longitude_dir: fields[5].to_string(),
        fix_quality: fields[6].parse().map_err(|_| "invalid fix quality")?,
        satellites: fields[7].parse().map_err(|_| "invalid satellite count")?,
        altitude_m: fields[9].parse().map_err(|_| "invalid altitude")?,
        latitude_deg,
        longitude_deg,
    })
}