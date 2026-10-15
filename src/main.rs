use opentopography_api::{Endpoint, endpoints};

fn main() {
    let srtm_endpoint = endpoints::global_dem::Endpoints::SrtmGl1;
    let gtiff= srtm_endpoint.get("335508e90e4fd9e2cd75c5a1817c8cb0", &40.0, &41.0, &-3.0, &-4.0).unwrap();
    println!("GeoTiff metadata: {:?}", gtiff.metadata());
}

