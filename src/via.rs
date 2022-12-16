const BASE_URL: &str = "https://app.viaeurope.com";
const PARCEL_PATH: &str = "/parcels";

pub fn open(reference: &str) {
    let url = format!("{BASE_URL}{PARCEL_PATH}/{reference}");
    open::that(url).unwrap();
}
