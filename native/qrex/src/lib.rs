use rustler::{Binary, NifStruct};

mod decoders;

#[derive(Debug, NifStruct, Clone)]
#[module = "QRex.QRCode"]
pub struct QRCode {
    /// The textual content of the QR Code
    pub text: String,
    /// The version of the QR Code, between 1 and 40
    pub version: usize,
    /// The number of modules of the QR Code, between 21 and 177
    pub modules: usize,
    /// The error correction level, between 0 and 3
    pub ecc_level: u16,
    /// The four boundary points of the QR Code
    pub bounds: Vec<(i32, i32)>,
}

#[rustler::nif]
fn detect_qr_codes(bytes: Binary) -> Result<Vec<Result<QRCode, String>>, String> {
    let image = image::load_from_memory(bytes.as_slice()).map_err(|e| e.to_string())?;

    let decoders = [
        decoders::rxing::detect_qr_codes,
        decoders::rrqr::detect_qr_codes,
        decoders::bardecoder::detect_qr_codes,
    ];

    for decoder in decoders {
        if let Ok(results) = decoder(image.clone()) {
            if results.iter().any(|result| result.is_ok()) {
                return Ok(results);
            }
        }
    }
    Ok(Vec::new())
}

rustler::init!("Elixir.QRex");
