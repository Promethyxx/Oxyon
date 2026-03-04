#![allow(dead_code)]
use image::imageops::FilterType;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zune_jpegxl::JxlSimpleEncoder;
use zune_core::options::EncoderOptions;
use zune_core::colorspace::ColorSpace;
use zune_core::bit_depth::BitDepth;

/// Liste des formats supportés
#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    // Formats de base
    Gif,
    Jpg,
    Png,
    WebP,
    // Formats avec features
    Exr,
    Jxl,
    Tiff,
    Ico,
    // Formats avec crates externes
    Svg,
    Raw,
    Dng,
    Psd,
}

impl ImageFormat {
    pub fn extension(&self) -> &str {
        match self {
            Self::Gif => "gif",
            Self::Jpg => "jpg",
            Self::Png => "png",
            Self::WebP => "webp",
            Self::Exr => "exr",
            Self::Jxl => "jxl",
            Self::Tiff => "tiff",
            Self::Ico => "ico",
            Self::Svg => "svg",
            Self::Raw => "raw",
            Self::Dng => "dng",
            Self::Psd => "psd",
        }
    }
}

/// Redimensionne l'image (Compression par dimension)
pub fn compresser(input: &Path, output: &str, ratio: u32) -> bool {
    crate::log_info(&format!("pic::compresser | ratio={} | {:?} -> {}", ratio, input, output));
    // Détection du format d'entrée
    if let Some(ext) = input.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "svg" => return compresser_svg(input, output, ratio),
            "psd" => return compresser_psd(input, output, ratio),
            "jxl" => return compresser_jxl(input, output, ratio),
            "dng" | "cr2" | "nef" | "arw" | "orf" | "rw2" => {
                crate::log_warn(&format!("pic::compresser format RAW non supporté pour {:?}", input));
                return false;
            },
            _ => {}
        }
    }

    // Format standard supporté par image crate
    match image::open(input) {
        Ok(img) => {
            let (w, h) = (img.width(), img.height());
            if ratio == 0 {
                crate::log_error(&format!("pic::compresser ratio=0 invalide pour {:?}", input));
                return false;
            }
            let scaled = img.resize(w / ratio, h / ratio, FilterType::Lanczos3);
            // Si la sortie est JXL, encoder via zune-jpegxl
            if output.to_lowercase().ends_with(".jxl") {
                return encoder_jxl(&scaled, output);
            }
            let ok = scaled.save(output).is_ok();
            if !ok {
                crate::log_error(&format!("pic::compresser échec save | {:?} -> {}", input, output));
            }
            ok
        },
        Err(e) => {
            crate::log_error(&format!("pic::compresser impossible d'ouvrir {:?} : {}", input, e));
            false
        }
    }
}

/// Conversion de format (ex: PNG -> JPG, WEBP -> PNG)
pub fn convertir(input: &Path, output: &str) -> bool {
    // Détection du format d'entrée
    if let Some(ext) = input.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "svg" => return convertir_svg(input, output),
            "psd" => return convertir_psd(input, output),
            "jxl" => return convertir_jxl(input, output),
            "dng" | "cr2" | "nef" | "arw" | "orf" | "rw2" => {
                crate::log_warn(&format!("pic::convertir format RAW non supporté pour {:?}", input));
                return false;
            },
            _ => {}
        }
    }

    // Format standard
    match image::open(input) {
        Ok(img) => {
            // Si la sortie est JXL, encoder via zune-jpegxl
            if output.to_lowercase().ends_with(".jxl") {
                return encoder_jxl(&img, output);
            }
            img.save(output).is_ok()
        },
        Err(_) => false,
    }
}

/// Lecture des métadonnées EXIF (GPS, Appareil, Date)
pub fn lire_exif(input: &Path) -> Vec<String> {
    let mut resultats = Vec::new();
    if let Ok(file) = File::open(input) {
        let mut reader = BufReader::new(file);
        let exifreader = exif::Reader::new();
        if let Ok(exdata) = exifreader.read_from_container(&mut reader) {
            for f in exdata.fields() {
                resultats.push(format!(
                    "{}: {}",
                    f.tag,
                    f.display_value().with_unit(&exdata)
                ));
            }
        }
    }
    resultats
}

/// Supprime l'EXIF en réenregistrant les pixels nus
pub fn supprimer_exif(input: &Path, output: &str) -> bool {
    if let Ok(img) = image::open(input) {
        // Enregistre uniquement les données de pixels, ignore les métadonnées sources
        img.save(output).is_ok()
    } else {
        false
    }
}

/// Rotation simple (90, 180, 270)
pub fn pivoter(input: &Path, output: &str, angle: u32) -> bool {
    crate::log_info(&format!("pic::pivoter | angle={} | {:?} -> {}", angle, input, output));
    match image::open(input) {
        Ok(img) => {
            let rotated = match angle {
                90 => img.rotate90(),
                180 => img.rotate180(),
                270 => img.rotate270(),
                _ => {
                    crate::log_warn(&format!("pic::pivoter angle invalide {} pour {:?}, image non modifiée", angle, input));
                    img
                }
            };
            let ok = rotated.save(output).is_ok();
            if !ok { crate::log_error(&format!("pic::pivoter échec save {:?}", output)); }
            ok
        },
        Err(e) => {
            crate::log_error(&format!("pic::pivoter impossible d'ouvrir {:?} : {}", input, e));
            false
        }
    }
}

/// Recadrage d'image (crop) - coordonnées en pourcentage de l'image
/// x, y = coin supérieur gauche (0-100)
/// width, height = dimensions du crop (0-100)
pub fn recadrer(input: &Path, output: &str, x_pct: u32, y_pct: u32, width_pct: u32, height_pct: u32) -> bool {
    crate::log_info(&format!("pic::recadrer | x={}% y={}% w={}% h={}% | {:?} -> {}", x_pct, y_pct, width_pct, height_pct, input, output));
    match image::open(input) {
        Ok(img) => {
            let (img_w, img_h) = (img.width(), img.height());
            
            // Convertir pourcentages en pixels
            let x = (img_w * x_pct) / 100;
            let y = (img_h * y_pct) / 100;
            let width = (img_w * width_pct) / 100;
            let height = (img_h * height_pct) / 100;
            
            // Vérifier les limites
            if x + width > img_w || y + height > img_h {
                crate::log_error(&format!(
                    "pic::recadrer crop hors limites | image={}x{} | crop x={} y={} w={} h={} | {:?}",
                    img_w, img_h, x, y, width, height, input
                ));
                return false;
            }
            
            let cropped = img.crop_imm(x, y, width, height);
            let ok = cropped.save(output).is_ok();
            if !ok { crate::log_error(&format!("pic::recadrer échec save {:?}", output)); }
            ok
        },
        Err(e) => {
            crate::log_error(&format!("pic::recadrer impossible d'ouvrir {:?} : {}", input, e));
            false
        }
    }
}

/// Redimensionne à une largeur/hauteur spécifique en pixels
pub fn redimensionner_pixels(input: &Path, output: &str, target_width: u32, target_height: u32) -> bool {
    crate::log_info(&format!("pic::redimensionner_pixels | {}x{} | {:?} -> {}", target_width, target_height, input, output));
    match image::open(input) {
        Ok(img) => {
            let resized = img.resize_exact(target_width, target_height, FilterType::Lanczos3);
            let ok = resized.save(output).is_ok();
            if !ok { crate::log_error(&format!("pic::redimensionner_pixels échec save {:?}", output)); }
            ok
        },
        Err(e) => {
            crate::log_error(&format!("pic::redimensionner_pixels impossible d'ouvrir {:?} : {}", input, e));
            false
        }
    }
}

/// Redimensionne pour atteindre un poids maximum (en Ko)
/// Réduit progressivement jusqu'à atteindre le poids cible
pub fn redimensionner_poids(input: &Path, output: &str, max_size_kb: u32) -> bool {
    crate::log_info(&format!("pic::redimensionner_poids | max={}Ko | {:?} -> {}", max_size_kb, input, output));
    let img = match image::open(input) {
        Ok(i) => i,
        Err(e) => {
            crate::log_error(&format!("pic::redimensionner_poids impossible d'ouvrir {:?} : {}", input, e));
            return false;
        }
    };
    
    let (orig_w, orig_h) = (img.width(), img.height());
    let max_size_bytes = max_size_kb as u64 * 1024;
    
    // Essayer différents ratios jusqu'à obtenir la taille voulue
    for ratio in 1..=10 {
        let new_w = orig_w / ratio;
        let new_h = orig_h / ratio;
        
        if new_w < 10 || new_h < 10 {
            crate::log_warn(&format!("pic::redimensionner_poids trop petit à ratio={} ({}x{}) pour {:?}", ratio, new_w, new_h, input));
            break;
        }
        
        let resized = img.resize(new_w, new_h, FilterType::Lanczos3);
        
        if resized.save(output).is_ok() {
            if let Ok(metadata) = std::fs::metadata(output) {
                crate::log_info(&format!("pic::redimensionner_poids ratio={} -> {}Ko (cible={}Ko)", ratio, metadata.len() / 1024, max_size_kb));
                if metadata.len() <= max_size_bytes {
                    return true;
                }
            }
        }
    }
    
    crate::log_error(&format!("pic::redimensionner_poids impossible d'atteindre {}Ko pour {:?}", max_size_kb, input));
    false
}

// === FONCTIONS POUR FORMATS SPÉCIAUX ===

/// Conversion SVG vers format raster
fn convertir_svg(input: &Path, output: &str) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };
    
    let mut svg_data = Vec::new();
    if file.read_to_end(&mut svg_data).is_err() {
        return false;
    }

    let opt = resvg::usvg::Options::default();
    let tree = match resvg::usvg::Tree::from_data(&svg_data, &opt) {
        Ok(t) => t,
        Err(_) => return false,
    };

    let size = tree.size();
    let mut pixmap = match resvg::tiny_skia::Pixmap::new(size.width() as u32, size.height() as u32) {
        Some(p) => p,
        None => return false,
    };
    
    resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());

    // Convertir en image::RgbaImage
    let img = match image::RgbaImage::from_raw(
        pixmap.width(),
        pixmap.height(),
        pixmap.data().to_vec(),
    ) {
        Some(i) => i,
        None => return false,
    };

    image::DynamicImage::ImageRgba8(img).save(output).is_ok()
}

/// Compression SVG (rasterise puis redimensionne)
fn compresser_svg(input: &Path, output: &str, ratio: u32) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };
    
    let mut svg_data = Vec::new();
    if file.read_to_end(&mut svg_data).is_err() {
        return false;
    }

    let opt = resvg::usvg::Options::default();
    let tree = match resvg::usvg::Tree::from_data(&svg_data, &opt) {
        Ok(t) => t,
        Err(_) => return false,
    };

    let size = tree.size();
    let mut pixmap = match resvg::tiny_skia::Pixmap::new(size.width() as u32, size.height() as u32) {
        Some(p) => p,
        None => return false,
    };
    
    resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let img = match image::RgbaImage::from_raw(
        pixmap.width(),
        pixmap.height(),
        pixmap.data().to_vec(),
    ) {
        Some(i) => i,
        None => return false,
    };

    let dynamic_img = image::DynamicImage::ImageRgba8(img);
    let (w, h) = (dynamic_img.width(), dynamic_img.height());
    let scaled = dynamic_img.resize(w / ratio, h / ratio, FilterType::Lanczos3);
    scaled.save(output).is_ok()
}

// === FONCTIONS POUR FORMAT JXL ===

/// Encode une DynamicImage en JXL via zune-jpegxl (lossless)
fn encoder_jxl(img: &image::DynamicImage, output: &str) -> bool {
    let rgba = img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    let pixels = rgba.as_raw();
    let opts = EncoderOptions::new(w as usize, h as usize, ColorSpace::RGBA, BitDepth::Eight);
    let encoder = JxlSimpleEncoder::new(pixels, opts);
    match encoder.encode() {
        Ok(data) => std::fs::write(output, &data).is_ok(),
        Err(e) => {
            crate::log_error(&format!("pic::encoder_jxl échec encodage JXL : {:?}", e));
            false
        }
    }
}

/// Décodage JXL vers DynamicImage via jxl-oxide
fn decoder_jxl(input: &Path) -> Option<image::DynamicImage> {
    let data = match std::fs::read(input) {
        Ok(d) => d,
        Err(e) => {
            crate::log_error(&format!("pic::decoder_jxl impossible de lire {:?} : {}", input, e));
            return None;
        }
    };
    let cursor = std::io::Cursor::new(data);
    let decoder = match jxl_oxide::integration::JxlDecoder::new(cursor) {
        Ok(d) => d,
        Err(e) => {
            crate::log_error(&format!("pic::decoder_jxl échec init décodeur {:?} : {}", input, e));
            return None;
        }
    };
    match image::DynamicImage::from_decoder(decoder) {
        Ok(img) => Some(img),
        Err(e) => {
            crate::log_error(&format!("pic::decoder_jxl échec décodage {:?} : {}", input, e));
            None
        }
    }
}

/// Conversion JXL vers format standard (PNG, JPG, etc.)
fn convertir_jxl(input: &Path, output: &str) -> bool {
    match decoder_jxl(input) {
        Some(img) => {
            if output.to_lowercase().ends_with(".jxl") {
                return encoder_jxl(&img, output);
            }
            img.save(output).is_ok()
        },
        None => false,
    }
}

/// Compression JXL (décode puis redimensionne)
fn compresser_jxl(input: &Path, output: &str, ratio: u32) -> bool {
    match decoder_jxl(input) {
        Some(img) => {
            let (w, h) = (img.width(), img.height());
            if ratio == 0 {
                crate::log_error(&format!("pic::compresser_jxl ratio=0 invalide pour {:?}", input));
                return false;
            }
            let scaled = img.resize(w / ratio, h / ratio, FilterType::Lanczos3);
            if output.to_lowercase().ends_with(".jxl") {
                return encoder_jxl(&scaled, output);
            }
            scaled.save(output).is_ok()
        },
        None => false,
    }
}
/// Conversion RAW vers format standard
fn convertir_raw(input: &Path, output: &str) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let decoded = match rawloader::decode(&mut file) {
        Ok(d) => d,
        Err(_) => return false,
    };

    let width = decoded.width;
    let height = decoded.height;
    
    // Extraire les données selon le type
    let image_data: Vec<u8> = match decoded.data {
        rawloader::RawImageData::Integer(ref data) => {
            data.iter().map(|&val| (val >> 8) as u8).collect()
        },
        rawloader::RawImageData::Float(ref data) => {
            data.iter().map(|&val| (val.clamp(0.0, 1.0) * 255.0) as u8).collect()
        },
    };

    let img = match image::RgbImage::from_raw(width as u32, height as u32, image_data) {
        Some(i) => i,
        None => return false,
    };

    image::DynamicImage::ImageRgb8(img).save(output).is_ok()
}

/// Compression RAW (décode puis redimensionne)
fn compresser_raw(input: &Path, output: &str, ratio: u32) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let decoded = match rawloader::decode(&mut file) {
        Ok(d) => d,
        Err(_) => return false,
    };

    let width = decoded.width;
    let height = decoded.height;
    
    let image_data: Vec<u8> = match decoded.data {
        rawloader::RawImageData::Integer(ref data) => {
            data.iter().map(|&val| (val >> 8) as u8).collect()
        },
        rawloader::RawImageData::Float(ref data) => {
            data.iter().map(|&val| (val.clamp(0.0, 1.0) * 255.0) as u8).collect()
        },
    };

    let img = match image::RgbImage::from_raw(width as u32, height as u32, image_data) {
        Some(i) => i,
        None => return false,
    };

    let dynamic_img = image::DynamicImage::ImageRgb8(img);
    let (w, h) = (dynamic_img.width(), dynamic_img.height());
    let scaled = dynamic_img.resize(w / ratio, h / ratio, FilterType::Lanczos3);
    scaled.save(output).is_ok()
}

/// Conversion PSD vers format standard
fn convertir_psd(input: &Path, output: &str) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut psd_data = Vec::new();
    if file.read_to_end(&mut psd_data).is_err() {
        return false;
    }

    let psd_file = match psd::Psd::from_bytes(&psd_data) {
        Ok(p) => p,
        Err(_) => return false,
    };

    // Récupérer l'image composite (flatten)
    let rgba = match psd_file.flatten_layers_rgba(&|_| true) {
        Ok(r) => r,
        Err(_) => return false,
    };

    let img = match image::RgbaImage::from_raw(
        psd_file.width(),
        psd_file.height(),
        rgba,
    ) {
        Some(i) => i,
        None => return false,
    };

    image::DynamicImage::ImageRgba8(img).save(output).is_ok()
}

/// Compression PSD (décode puis redimensionne)
fn compresser_psd(input: &Path, output: &str, ratio: u32) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut psd_data = Vec::new();
    if file.read_to_end(&mut psd_data).is_err() {
        return false;
    }

    let psd_file = match psd::Psd::from_bytes(&psd_data) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let rgba = match psd_file.flatten_layers_rgba(&|_| true) {
        Ok(r) => r,
        Err(_) => return false,
    };

    let img = match image::RgbaImage::from_raw(
        psd_file.width(),
        psd_file.height(),
        rgba,
    ) {
        Some(i) => i,
        None => return false,
    };

    let dynamic_img = image::DynamicImage::ImageRgba8(img);
    let (w, h) = (dynamic_img.width(), dynamic_img.height());
    let scaled = dynamic_img.resize(w / ratio, h / ratio, FilterType::Lanczos3);
    scaled.save(output).is_ok()
}