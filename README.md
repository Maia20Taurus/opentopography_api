# OpenTopography API

A Rust gateway to the OpenTopography API, used to obtain geographical data from datasets such as SRTM; this crate returns GeoTiffs from API calls.

## Example

The following program queries OpenTopography on the SRTMGL1 dataset:
```rust
/// Get a 1x1 GeoTiff.

use opentopography_api::{Endpoint, endpoints};

fn main() {
    let srtm_endpoint = endpoints::global_dem::Endpoints::SrtmGl1;
    let gtiff = srtm_endpoint.get("demoapikeyot2022", &40.0, &41.0, &-3.0, &-4.0).unwrap();
    println!("GeoTiff metadata: {:?}", gtiff.metadata());
}
```

Please see https://portal.opentopography.org/apidocs/#/Public/getGlobalDem