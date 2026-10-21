use geotiff_reader::GeoTiffFile;

pub trait Endpoint {
    /// Query the OpenTopography API for geographic data.
    /// See <https://portal.opentopography.org/apidocs/#/Public> for more info
    /// and obtaining an API key
    /// 
    /// Returns The elevation data or the response if the call fails
    fn get(&self,api_key: &str, south: &f64, north:&f64, east:&f64, west:&f64)
     -> Result<GeoTiffFile, String>;
}

pub mod endpoints;