use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use exif::{self, In, Tag, Field, Value};
use exif::experimental::Writer;
use img_parts::jpeg::{markers, Jpeg, JpegSegment};
use img_parts::{Bytes, ImageEXIF};

use std::io::{Cursor, Read};
use std::fs::{self, File};
use std::path::Path;
use super::coordinates::Coordinates;


#[derive(Serialize, Deserialize)]
pub struct Image {
    pub path: String,
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

        let mut coordinates = ('N', Vec::new(), 'E', Vec::new());
        match exif.get_field(Tag::GPSLatitude, In::PRIMARY) {
            Some(field) => {
                coordinates.1 = match &field.value {
                    Value::Rational(value) => value.iter().map(|&x| x.num as f64 / x.denom as f64).collect(),
                    _ => return Err("GPSLatitude has Invalid value".to_string())
                };
            },
            None => warn!("Unable to find GPSLatitude"),
        }

        match exif.get_field(Tag::GPSLatitudeRef, In::PRIMARY) {
            Some(field) => {
                coordinates.0 = match &field.value {
                    Value::Ascii(value) => value[0][0] as char,
                    _ => return Err(format!("GPSLatitudeRef has Invalid value {:?}", field.value))
                };
            },
            None => warn!("Unable to find GPSLatitudeRef"),
        }

        match exif.get_field(Tag::GPSLongitude, In::PRIMARY) {
            Some(field) => {
                coordinates.3 = match &field.value {
                    Value::Rational(value) => value.iter().map(|&x| x.num as f64 / x.denom as f64).collect(),
                    _ => return Err("GPSLongitude has Invalid value".to_string())
                };
            },
            None => warn!("Unable to find GPSLongitude"),
        }

        match exif.get_field(Tag::GPSLongitudeRef, In::PRIMARY) {
            Some(field) => {
                coordinates.2 = match &field.value {
                    Value::Ascii(value) => value[0][0] as char,
                    _ => return Err(format!("GPSLatitudeRef has Invalid value {:?}", field.value))
                };
            },
            None => warn!("Unable to find GPSLatitudeRef"),
        }

        let coordinates: Option<Coordinates> = match coordinates {
            (latref, lat, lonref, lon) if !lat.is_empty() && !lon.is_empty() => Coordinates::from_4uple((latref, lat, lonref, lon)),
            _ => None
        };

        Ok(Image {
            path: file_path.to_string(),
            coordinates
        })
    }

    pub fn save(&self) -> Result<(), String> {
        let mut writer = Writer::new();
        let mut buffer = Cursor::new(Vec::new());

        let latitude_field: Field;
        let latituderef_field: Field;
        let longitude_field: Field;
        let longituderef_field: Field;

        if let Some(coordinates) = &self.coordinates {
            let (latitude, longitude) = coordinates.to_rational();

            latitude_field = Field {
                tag: Tag::GPSLatitude,
                ifd_num: In::PRIMARY,
                value: Value::Rational(latitude),
            };

            latituderef_field = Field {
                tag: Tag::GPSLatitudeRef,
                ifd_num: In::PRIMARY,
                value: Value::Ascii(vec![vec![coordinates.latitude.0 as u8]]),
            };

            longitude_field = Field {
                tag: Tag::GPSLongitude,
                ifd_num: In::PRIMARY,
                value: Value::Rational(longitude),
            };

            longituderef_field = Field {
                tag: Tag::GPSLongitudeRef,
                ifd_num: In::PRIMARY,
                value: Value::Ascii(vec![vec![coordinates.longitude.0 as u8]]),
            };

            writer.push_field(&latitude_field);
            writer.push_field(&latituderef_field);
            writer.push_field(&longitude_field);
            writer.push_field(&longituderef_field);
        }

        if let Err(e) = writer.write(&mut buffer, false) {
            error!("Error writing exif data: {:?}", e);
            return Err(format!("Error writing exif data: {:?}", e));
        }
        
        let output_path = Path::new(&self.path);
        info!("Writing to: {:?}", output_path);
        let input = match fs::read(&self.path) {
            Ok(input) => input,
            Err(e) => return Err(format!("Error reading input file: {:?}", e)),
        };
        let output = File::create(output_path).expect("Failed to create output file");

        if self.path.contains(".jpg") || self.path.contains(".jpeg") {
            let mut jpeg = match Jpeg::from_bytes(input.into()) {
                Ok(jpeg) => jpeg,
                Err(e) => return Err(format!("Error reading input file: {:?}", e)),
            };

            jpeg.set_exif(Some(buffer.into_inner().into()));
            jpeg.encoder().write_to(output).expect("Failed to write to output file");
        } else {
            error!("Unsupported file format: {:?}", self.path.split('.').last());
            return Err("Unsupported file type".to_string());
        }

        Ok(())
    }
}