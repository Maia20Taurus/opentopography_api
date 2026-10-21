use crate::Endpoint;
use geotiff_reader::GeoTiffFile;

pub enum Endpoints {
    Usgs30m,
    Usgs10m,
    Usgs1m
}

impl Endpoints {
    fn as_str(&self) -> &'static str {
        match self {
            Endpoints::Usgs30m => "USGS30m",
            Endpoints::Usgs10m => "USGS10m",
            Endpoints::Usgs1m => "USGS1m"
        }
    }
}

impl ToString for Endpoints {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}