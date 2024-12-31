use geo::{Point, Polygon, MultiPolygon};
use serde::{Deserialize, Deserializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;

pub fn deserialize_point<'de, D>(deserializer: D) -> Result<Point<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let coordinates: &str = Deserialize::deserialize(deserializer)?;
    let coords: Vec<f64> = coordinates
        .split(',')
        .map(|c| c.trim().parse::<f64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::de::Error::custom)?;

    if coords.len() == 2 {
        Ok(Point::new(coords[0], coords[1]))
    } else {
        Err(serde::de::Error::custom("Invalid point coordinates"))
    }
}

pub fn deserialize_optional_point<'de, D>(deserializer: D) -> Result<Option<Point<f64>>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the coordinates string into a Point
    let coordinates: Option<String> = Option::deserialize(deserializer)?;

    // If coordinates are present, parse them
    if let Some(coord_str) = coordinates {
        let coords: Vec<f64> = coord_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        // Assuming the string contains a single coordinate pair (x, y)
        if coords.len() == 2 {
            Ok(Some(Point::new(coords[0], coords[1])))
        } else {
            Err(de::Error::custom("Invalid coordinates for Point"))
        }
    } else {
        Ok(None)
    }
}

pub fn deserialize_polygon<'de, D>(deserializer: D) -> Result<Polygon<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let coordinates: &str = Deserialize::deserialize(deserializer)?;
    let points: Vec<Point<f64>> = coordinates
        .split_whitespace()
        .map(|pair| {
            let vals: Vec<f64> = pair
                .split(',')
                .map(|c| c.trim().parse::<f64>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(serde::de::Error::custom)?;

            if vals.len() == 2 {
                Ok(Point::new(vals[0], vals[1]))
            } else {
                Err(serde::de::Error::custom("Invalid polygon point coordinates"))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    if !points.is_empty() {
        Ok(Polygon::new(points.into(), vec![]))
    } else {
        Err(serde::de::Error::custom("Empty polygon coordinates"))
    }
}

pub fn deserialize_optional_polygon<'de, D>(deserializer: D) -> Result<Option<Polygon<f64>>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the coordinates string into a Polygon
    let coordinates: Option<String> = Option::deserialize(deserializer)?;

    // If coordinates are present, parse them
    if let Some(coord_str) = coordinates {
        let coords: Vec<f64> = coord_str
            .split_whitespace()
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        // Assuming the string contains multiple points for a polygon
        if coords.len() >= 6 && coords.len() % 2 == 0 {
            let points: Vec<Point<f64>> = coords
                .chunks(2)
                .map(|chunk| Point::new(chunk[0], chunk[1]))
                .collect();

            Ok(Some(Polygon::new(points.into(), vec![])))
        } else {
            Err(de::Error::custom("Invalid coordinates for Polygon"))
        }
    } else {
        Ok(None)
    }
}

pub fn deserialize_multipolygon<'de, D>(deserializer: D) -> Result<MultiPolygon<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct MultiPolygonVisitor;

    impl<'de> Visitor<'de> for MultiPolygonVisitor {
        type Value = MultiPolygon<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid MultiPolygon in XML format")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut polygons = Vec::new();

            while let Some((key, value)) = map.next_entry::<String, String>()? {
                if key == "Polygon" {
                    // Parse each individual polygon and add it to the MultiPolygon
                    let polygon = parse_polygon(&value).map_err(de::Error::custom)?;
                    polygons.push(polygon);
                }
            }

            Ok(MultiPolygon(polygons))
        }
    }

    deserializer.deserialize_map(MultiPolygonVisitor)
}

/// Helper function to parse a single Polygon from XML data
fn parse_polygon(xml_data: &str) -> Result<Polygon<f64>, &'static str> {
    // Logic to parse a single Polygon from XML.
    // You might need to use an XML parser here to extract coordinates
    // and construct the `Polygon<f64>`.
    //
    // For example:
    // - Extract exterior and interior rings.
    // - Convert them to `geo::LineString<f64>`.
    // - Return a `Polygon<f64>`.
    unimplemented!("Write the XML parsing logic here.")
}