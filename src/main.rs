use opentopography_api::{Endpoint, endpoints};

fn main() {
    let srtm_endpoint = endpoints::global_dem::Endpoints::SrtmGl1;
    srtm_endpoint.get("335508e90e4fd9e2cd75c5a1817c8cb0", &40.0, &51.0, &-3.0, &-3.1).unwrap();
}

