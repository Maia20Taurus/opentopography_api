//! # opentopography_api
//! A Rust gateway to the OpenTopography API, which is used to query
//! geographic data e.g. finding the elevation of a particular coordinate.
//! See <https://portal.opentopography.org/apidocs/#/Public> for more info on the API.
//!
//! ## Example
//! ```rust
//! // Get a 1x1 GeoTiff.
//! use opentopography_api::{Endpoint, endpoints};
//! fn main() {
//!     let srtm_endpoint = endpoints::global_dem::Endpoints::SrtmGl1;
//!     let gtiff = srtm_endpoint.get("demoapikeyot2022", &40.0, &41.0, &-3.0, &-4.0).unwrap();
//!     println!("GeoTiff metadata: {:?}", gtiff.metadata());
//! }
//! ```
//! 
//! ## Disclaimer
//! 
//! The current version only supports the /globaldem API route, but support for the others
//! is planned.

use geotiff_reader::GeoTiffFile;

pub trait Endpoint {
    /// Query the OpenTopography API for geographic data.
    /// See <https://portal.opentopography.org/apidocs/#/Public> for more info
    /// and obtaining an API key
    ///
    /// Returns The elevation data or the response if the call fails
    fn get(
        &self,
        api_key: &str,
        south: &f64,
        north: &f64,
        east: &f64,
        west: &f64,
    ) -> Result<GeoTiffFile, String>;
}

pub mod endpoints;
