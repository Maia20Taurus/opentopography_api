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
            Endpoints::Aw3d30 => "",
            Endpoints::Aw3d30E => todo!(),
            Endpoints::SRTM15Plus => todo!(),
            Endpoints::NasaDem => todo!(),
            Endpoints::Cop30 => todo!(),
            Endpoints::Cop90 => todo!(),
            Endpoints::EuDtm => todo!(),
            Endpoints::Gedil3 => todo!(),
            Endpoints::GEBCOIceTopo => todo!(),
            Endpoints::GEBCOSubIceTopo => todo!(),
            Endpoints::CamrDemDsm => todo!(),
            Endpoints::CamrDemDtm => todo!(),
            Endpoints::Anadem => todo!(),
            Endpoints::Gedtm30 => todo!(),
        }
    }
}

impl ToString for Endpoints {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<String> for Endpoints {

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
        Err(String::new())
    }
}
