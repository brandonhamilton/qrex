use bardecoder;
use bardecoder::detect::{Detect, LineScan, Location};
use bardecoder::util::qr::QRLocation;

use image::DynamicImage;
use image::GrayImage;

use std::cell::RefCell;
use std::rc::Rc;

use crate::QRCode;

struct LeakLocationsDetector<'a> {
    real_detector: Box<dyn Detect<GrayImage> + 'a>,
    pub locations: Rc<RefCell<Vec<Location>>>,
}

impl<'a> LeakLocationsDetector<'a> {
    pub fn new(real_detector: Box<impl Detect<GrayImage> + 'a>) -> LeakLocationsDetector<'a> {
        LeakLocationsDetector {
            real_detector,
            locations: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn get_locations(&self) -> Rc<RefCell<Vec<Location>>> {
        self.locations.clone()
    }
}

impl<'a> Detect<GrayImage> for LeakLocationsDetector<'a> {
    fn detect(&'_ self, prepared: &GrayImage) -> Vec<Location> {
        let locations = self.real_detector.detect(prepared);
        let mut mylocs = self.locations.borrow_mut();

        *mylocs = locations
            .iter()
            .map(|Location::QR(l)| {
                Location::QR(QRLocation {
                    top_left: l.top_left,
                    bottom_left: l.bottom_left,
                    top_right: l.top_right,
                    module_size: l.module_size,
                    version: l.version,
                })
            })
            .collect();

        locations
    }
}

pub fn detect_qr_codes(image: DynamicImage) -> Result<Vec<Result<QRCode, String>>, String> {
    let mut db = bardecoder::default_builder_with_info();
    let detector = LeakLocationsDetector::new(Box::new(LineScan::new()));
    let locations = detector.get_locations();
    db.detect(Box::new(detector));
    let decoder = db.build();
    let decoded = decoder.decode(&image);
    let mut results = Vec::new();
    let mut i = 0;
    for result in decoded {
        match result {
            Ok((content, info)) => {
                let cur_loc = &(locations.borrow())[i];
                i += 1;
                let Location::QR(loc) = cur_loc;
                let fourth = (
                    (loc.top_left.x + (loc.top_right.x - loc.bottom_left.x)) as i32,
                    (loc.top_left.y + (loc.bottom_left.y - loc.top_right.y)) as i32,
                );
                let bounds = vec![
                    (loc.top_left.x as i32, loc.top_left.y as i32),
                    (loc.top_right.x as i32, loc.top_right.y as i32),
                    fourth,
                    (loc.bottom_left.x as i32, loc.bottom_left.y as i32),
                ];
                results.push(Ok(QRCode {
                    text: content.clone(),
                    version: info.version as usize,
                    modules: (info.version as usize - 1) * 4 + 21,
                    ecc_level: info.ec_level as u16,
                    bounds,
                }))
            }
            Err(error) => results.push(Err(error.to_string())),
        }
    }
    Ok(results)
}
