use log::{info, error};
use serde::{Deserialize, Serialize};
use exif::{self, In};

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: (f64, f64, f64),
    pub longitude: (f64, f64, f64),
}

impl Coordinates {
    fn from((lat, lon): (Vec<f64>, Vec<f64>)) -> Option<Self> {
        if lat.len() != 3 || lon.len() != 3 {
            return None;
        }
        Some(Coordinates {
            latitude: (lat[0], lat[1], lat[2]),
            longitude: (lon[0], lon[1], lon[2]),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub name: String,
    pub coordinates: Option<Coordinates>,
}

impl Image {
    pub fn new(file_path: &str) -> Result<Image, String> {
        info!("Working on file: {}", file_path);
        let file = match std::fs::File::open(file_path) {
            Ok(file) => file,
            Err(e) => return Err(format!("Error opening file: {:?}", e)),
        };
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = match exifreader.read_from_container(&mut bufreader){
            Ok(exif) => exif,
            Err(e) => return Err(format!("Error reading exif data: {:?}", e)),
        };

        let mut coordinates = (Vec::new(), Vec::new());
        match exif.get_field(exif::Tag::GPSLatitude, In::PRIMARY) {
            Some(field) => {
                coordinates.0 = match &field.value {
                    exif::Value::Rational(value) => value.iter().map(|&x| x.num as f64 / x.denom as f64).collect(),
                    _ => return Err("GPSLatitude has Invalid value".to_string())
                };
            },
            None => println!("Unable to find GPSLatitude"),
        }

        match exif.get_field(exif::Tag::GPSLongitude, In::PRIMARY) {
            Some(field) => {
                coordinates.1 = match &field.value {
                    exif::Value::Rational(value) => value.iter().map(|&x| x.num as f64 / x.denom as f64).collect(),
                    _ => return Err("GPSLongitude has Invalid value".to_string())
                };
            },
            None => println!("Unable to find GPSLongitude"),
        }

        let coordinates: Option<Coordinates> = match coordinates {
            (lat, lon) if !lat.is_empty() && !lon.is_empty() => Coordinates::from((lat, lon)),
            _ => None
        };

        Ok(Image {
            name: file_path.to_string(),
            coordinates
        })
    }
}