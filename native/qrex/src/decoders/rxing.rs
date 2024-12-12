use image::{self, DynamicImage};
use rxing::{
    common::{cpp_essentials::ConcentricPattern, DetectorRXingResult, HybridBinarizer},
    qrcode::cpp_port::{
        decoder::Decode,
        detector::{FindFinderPatterns, GenerateFinderPatternSets, SampleQR},
    },
    BinaryBitmap, BufferedImageLuminanceSource,
};

use crate::QRCode;

pub fn detect_qr_codes(image: DynamicImage) -> Result<Vec<Result<QRCode, String>>, String> {
    let binary_image = BinaryBitmap::new(HybridBinarizer::new(BufferedImageLuminanceSource::new(
        image,
    )));
    let bit_matrix = binary_image.get_black_matrix();

    let mut all_finder_patterns = FindFinderPatterns(bit_matrix, true);
    let mut used_finder_patterns: Vec<ConcentricPattern> = Vec::new();
    let mut results: Vec<Result<QRCode, String>> = Vec::new();

    let all_finder_pattern_sets = GenerateFinderPatternSets(&mut all_finder_patterns);
    for finder_pattern_set in all_finder_pattern_sets {
        if used_finder_patterns.contains(&finder_pattern_set.bl)
            || used_finder_patterns.contains(&finder_pattern_set.tl)
            || used_finder_patterns.contains(&finder_pattern_set.tr)
        {
            continue;
        }
        let detector_result = SampleQR(bit_matrix, &finder_pattern_set);
        if let Ok(detector_result) = detector_result {
            let decoder_result = Decode(detector_result.getBits());
            let position = detector_result.getPoints();
            if let Ok(decoder_result) = decoder_result {
                if decoder_result.isValid() {
                    used_finder_patterns.push(finder_pattern_set.bl);
                    used_finder_patterns.push(finder_pattern_set.tl);
                    used_finder_patterns.push(finder_pattern_set.tr);

                    let version = decoder_result.versionNumber() as usize;
                    let modules = 17 + (4 * version);
                    let ecc_level = match decoder_result.ecLevel() {
                        "L" => 0,
                        "M" => 1,
                        "Q" => 2,
                        "H" => 3,
                        _ => 0,
                    };
                    let bounds = position.iter().map(|p| (p.x as i32, p.y as i32)).collect();
                    results.push(Ok(QRCode {
                        text: decoder_result.text(),
                        version,
                        modules,
                        ecc_level,
                        bounds,
                    }));
                }
            }
        }
    }
    Ok(results)
}
