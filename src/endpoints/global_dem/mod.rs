use crate::{ElevationData, Endpoint};

// deriving debug allows the enums to be converted to strings easily
#[derive(Debug)]
pub enum Endpoints {
    SRTMGL1,
    SRTMGL3,
    SRTMGL1_E,
    AW3D30,
    AW3D30_E,
    SRTM15Plus,
    NASADEM,
    COP30,
    COP90,
    EU_DTM,
    GEDI_L3,
    GEBCOIceTopo,
    GEBCOSubIceTopo,
    CA_MRDEM_DSM,
    CA_MRDEM_DTM,
    ANADEM,
    GEDTM30,
}

impl Endpoint for Endpoints {
    fn get(
        &self,
        api_key: &str,
        south: &f64,
        north: &f64,
        east: &f64,
        west: &f64,
    ) -> Result<ElevationData, String> {
        let dataset_name: String = format!("{:?}", self);
    }
}
