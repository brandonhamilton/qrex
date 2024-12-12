use image::{self, DynamicImage};
use rqrr;

use crate::QRCode;

pub fn detect_qr_codes(image: DynamicImage) -> Result<Vec<Result<QRCode, String>>, String> {
    let img = image.to_luma8();
    let mut img = rqrr::PreparedImage::prepare(img);
    let grids = img.detect_grids();
    let mut results = Vec::new();
    for grid in grids.iter() {
        match grid.decode() {
            Ok((meta, text)) => {
                results.push(Ok(QRCode {
                    text,
                    version: meta.version.0,
                    modules: meta.version.to_size(),
                    ecc_level: meta.ecc_level,
                    bounds: grid.bounds.iter().map(|b| (b.x, b.y)).collect(),
                }));
            }
            Err(error) => {
                results.push(Err(error.to_string()));
            }
        }
    }
    Ok(results)
}
