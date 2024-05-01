use serde::{Deserialize, Serialize};
use exif::{self, Rational};
use fraction::Fraction;

use super::utils::frac_to_tuple;

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: (char, f64, f64, f64),
    pub longitude: (char, f64, f64, f64),
}

impl Coordinates {
    pub fn from_4uple((latref, lat, lonref, lon): (char, Vec<f64>, char, Vec<f64>)) -> Option<Self> {
        if lat.len() != 3 || lon.len() != 3 {
            return None;
        }
        Some(Coordinates {
            latitude: (latref, lat[0], lat[1], lat[2]),
            longitude: (lonref, lon[0], lon[1], lon[2]),
        })
    }

    pub fn to_rational(&self) -> (Vec<Rational>, Vec<Rational>) {
        let latitude = vec![
            Rational::from(frac_to_tuple(Fraction::from(self.latitude.1))),
            Rational::from(frac_to_tuple(Fraction::from(self.latitude.2))),
            Rational::from(frac_to_tuple(Fraction::from(self.latitude.3)))
        ];

        let longitude = vec![
            Rational::from(frac_to_tuple(Fraction::from(self.longitude.1))),
            Rational::from(frac_to_tuple(Fraction::from(self.longitude.2))),
            Rational::from(frac_to_tuple(Fraction::from(self.longitude.3)))
        ];

        (latitude, longitude)
    }
}
