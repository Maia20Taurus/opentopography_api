use opentopography_api::{Endpoint, endpoints};

fn main() {
    let srtm_endpoint = endpoints::global_dem::Endpoints::SrtmGl1;
    let gtiff= srtm_endpoint.get("demoapikeyot2022", &40.0, &41.0, &-3.0, &-4.0).unwrap();
    println!("GeoTiff metadata: {:?}", gtiff.metadata());
}

