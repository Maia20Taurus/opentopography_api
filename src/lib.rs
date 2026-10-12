/// Container for geographic data returned by the OpenTopographyAPI
pub struct ElevationData {
    elevation: Vec<Vec<u8>>,
    south: f64,
    north: f64,
    east: f64,
    west: f64
}

impl ElevationData {
    pub fn new(elevation: Vec<Vec<u8>>, south:&f64, north:&f64, east:&f64, west:&f64) -> Self {
        Self {
            elevation,
            south: *south,
            north: *north,
            east: *east,
            west: *west
        }
    }
}


pub trait Endpoint {
    /// Query the OpenTopography API for geographic data
    /// See https://portal.opentopography.org/apidocs/#/Public for more info
    /// and obtaining an API key
    /// 
    /// Returns The elevation data or the response if the call fails
    fn get(&self,api_key: &str, dataset_name: &str, south: &f64, north:&f64, east:&f64, west:&f64)
     -> Result<ElevationData, String>;
}

pub mod endpoints;