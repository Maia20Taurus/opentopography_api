use crate::{ElevationData, Endpoint};

// deriving debug allows the enums to be converted to strings easily
#[derive(Debug)]
pub enum Endpoints {
    SrtmGl1,
    SrtmGl3,
    SrtmGl1E,
    Aw3d30,
    Aw3d30E,
    SRTM15Plus,
    NasaDem,
    Cop30,
    Cop90,
    EuDtm,
    Gedil3,
    GEBCOIceTopo,
    GEBCOSubIceTopo,
    CamrDemDsm,
    CamrDemDtm,
    Anadem,
    Gedtm30,
}

impl Endpoints {
    fn as_str(&self) -> &'static str {
        match self {
            Endpoints::SrtmGl1 => "SRTMGl1",
            Endpoints::SrtmGl3 => "SRTMGL3",
            Endpoints::SrtmGl1E => "SRTMGL1_E",
            Endpoints::Aw3d30 => "AW3D30",
            Endpoints::Aw3d30E => "AW3D30_E",
            Endpoints::SRTM15Plus => "SRTM15Plus",
            Endpoints::NasaDem => "NASADEM",
            Endpoints::Cop30 => "COP30",
            Endpoints::Cop90 => "COP90",
            Endpoints::EuDtm => "EU_DTM",
            Endpoints::Gedil3 => "GEDI_L3",
            Endpoints::GEBCOIceTopo => "GEBCOIceTopo",
            Endpoints::GEBCOSubIceTopo => "GEBCOSubIceTopo",
            Endpoints::CamrDemDsm => "CA_MRDEM_DSM",
            Endpoints::CamrDemDtm => "CA_MRDEM_DTM",
            Endpoints::Anadem => "ANADEM",
            Endpoints::Gedtm30 => "GEDTM30",
        }
    }
}

impl ToString for Endpoints {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
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
        
    }
}
