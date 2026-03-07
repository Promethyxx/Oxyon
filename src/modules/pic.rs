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

/// Convertit/compresse l'image avec un contrôle de qualité (1=rapide, 10=max qualité).
/// La résolution n'est JAMAIS modifiée.
pub fn compresser(input: &Path, output: &str, qualite: u32) -> bool {
    crate::log_info(&format!("pic::compresser | qualite={} | {:?} -> {}", qualite, input, output));
    // Détection du format d'entrée
    if let Some(ext) = input.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "svg" => return convertir_svg(input, output),
            "psd" => return convertir_psd(input, output),
            "jxl" => return compresser_jxl_qualite(input, output, qualite),
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
            // Si la sortie est JXL, encoder via zune-jpegxl
            if output.to_lowercase().ends_with(".jxl") {
                return encoder_jxl(&img, output);
            }
            // Si la sortie est JPEG, appliquer la qualité
            if output.to_lowercase().ends_with(".jpg") || output.to_lowercase().ends_with(".jpeg") {
                return sauvegarder_jpeg(&img, output, qualite);
            }
            // Si la sortie est WebP, appliquer la qualité
            if output.to_lowercase().ends_with(".webp") {
                return sauvegarder_webp(&img, output, qualite);
            }
            // Formats lossless (PNG, TIFF, etc.) : sauvegarde directe
            let ok = img.save(output).is_ok();
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

/// Sauvegarde JPEG avec qualité (1-10 → 10-100%)
fn sauvegarder_jpeg(img: &image::DynamicImage, output: &str, qualite: u32) -> bool {
    use image::codecs::jpeg::JpegEncoder;
    let q = (qualite.clamp(1, 10) * 10) as u8; // 1→10%, 10→100%
    let file = match std::fs::File::create(output) {
        Ok(f) => f,
        Err(e) => {
            crate::log_error(&format!("pic::sauvegarder_jpeg création fichier {} : {}", output, e));
            return false;
        }
    };
    let mut encoder = JpegEncoder::new_with_quality(file, q);
    let rgb = img.to_rgb8();
    match encoder.encode(
        rgb.as_raw(),
        rgb.width(),
        rgb.height(),
        image::ExtendedColorType::Rgb8,
    ) {
        Ok(()) => true,
        Err(e) => {
            crate::log_error(&format!("pic::sauvegarder_jpeg échec encodage {} : {}", output, e));
            false
        }
    }
}

/// Sauvegarde WebP — qualité via image crate save (lossless)
fn sauvegarder_webp(img: &image::DynamicImage, output: &str, _qualite: u32) -> bool {
    // La crate image encode en WebP lossless par défaut
    match img.save(output) {
        Ok(()) => true,
        Err(e) => {
            crate::log_error(&format!("pic::sauvegarder_webp échec save {} : {}", output, e));
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

/// Compression SVG (rasterise sans redimensionnement)
fn compresser_svg(input: &Path, output: &str) -> bool {
    convertir_svg(input, output)
}

// === FONCTIONS POUR FORMAT JXL ===

/// Encode une DynamicImage en JXL via zune-jpegxl (lossless)
fn encoder_jxl(img: &image::DynamicImage, output: &str) -> bool {
    let rgba = img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    let pixels = rgba.as_raw();
    let opts = EncoderOptions::new(w as usize, h as usize, ColorSpace::RGBA, BitDepth::Eight);
    let encoder = JxlSimpleEncoder::new(pixels, opts);
    let mut buf: Vec<u8> = Vec::new();
    match encoder.encode(&mut buf) {
        Ok(_) => std::fs::write(output, &buf).is_ok(),
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

/// Compression JXL avec qualité (décode puis réencode sans resize)
fn compresser_jxl_qualite(input: &Path, output: &str, _qualite: u32) -> bool {
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

/// Compression RAW (décode sans redimensionnement)
fn compresser_raw(input: &Path, output: &str) -> bool {
    convertir_raw(input, output)
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

/// Compression PSD (décode sans redimensionnement)
fn compresser_psd(input: &Path, output: &str) -> bool {
    convertir_psd(input, output)
}

// ════════════════════════════════════════════════════════════════════════
//  JXL BATCH — remplace oxyj.ps1, oxyjf.ps1, oxyjp.ps1
// ════════════════════════════════════════════════════════════════════════

/// Extensions source pour la conversion JXL
fn est_source_jxl(ext: &str) -> bool {
    matches!(ext, "jpg" | "jpeg" | "png" | "webp")
}

/// Convertit un fichier image en JXL lossless (en place, à côté du fichier source).
/// Skip si le .jxl existe déjà. (oxyj.ps1)
pub fn convertir_jxl_lossless(input: &Path) -> Result<(), String> {
    let out = input.with_extension("jxl");
    if out.exists() {
        return Ok(()); // déjà converti
    }

    crate::log_info(&format!("pic::convertir_jxl_lossless | {:?} -> {:?}", input, out));

    let img = image::open(input)
        .map_err(|e| format!("Impossible d'ouvrir {:?} : {}", input, e))?;

    let out_str = out.to_string_lossy().to_string();
    if encoder_jxl(&img, &out_str) {
        Ok(())
    } else {
        Err(format!("Encodage JXL échoué pour {:?}", input))
    }
}

/// Convertit en JXL dans un dossier séparé "{dossier} jxl".
/// Tente d'abord un encodage direct, puis fallback pixel mode. (oxyjf.ps1)
pub fn convertir_jxl_dossier(input: &Path) -> Result<(), String> {
    let parent = input.parent().ok_or("Pas de dossier parent")?;
    let dir_name = parent.file_name().unwrap_or_default().to_string_lossy().to_string();

    // Ne pas traiter les fichiers déjà dans un dossier " jxl"
    if dir_name.ends_with(" jxl") {
        return Ok(());
    }

    let jxl_dir = parent.parent()
        .unwrap_or(parent)
        .join(format!("{} jxl", dir_name));

    if !jxl_dir.exists() {
        std::fs::create_dir_all(&jxl_dir)
            .map_err(|e| format!("Impossible de créer {:?} : {}", jxl_dir, e))?;
    }

    let stem = input.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let out = jxl_dir.join(format!("{}.jxl", stem));

    if out.exists() {
        return Ok(());
    }

    crate::log_info(&format!("pic::convertir_jxl_dossier | {:?} -> {:?}", input, out));

    let img = image::open(input)
        .map_err(|e| format!("Impossible d'ouvrir {:?} : {}", input, e))?;

    let out_str = out.to_string_lossy().to_string();
    if encoder_jxl(&img, &out_str) {
        Ok(())
    } else {
        Err(format!("Encodage JXL échoué pour {:?}", input))
    }
}

/// Convertit en JXL via PNG pivot (pour les fichiers problématiques). (oxyjp.ps1)
/// Décode l'image via la crate image (équivalent du System.Drawing du PS1),
/// sauvegarde en PNG temporaire, puis encode en JXL.
pub fn convertir_jxl_pivot(input: &Path) -> Result<(), String> {
    let stem = input.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let parent = input.parent().ok_or("Pas de dossier parent")?;
    let out = parent.join(format!("{}_pivot.jxl", stem));

    if out.exists() {
        return Ok(());
    }

    crate::log_info(&format!("pic::convertir_jxl_pivot | {:?} -> {:?}", input, out));

    // Charger via image crate (équivalent du System.Drawing.Bitmap)
    let img = image::open(input)
        .map_err(|e| format!("Impossible d'ouvrir {:?} : {}", input, e))?;

    // Sauvegarder en PNG temporaire (format pivot propre)
    let temp_png = std::env::temp_dir().join(format!("oxyon_pivot_{}.png", stem));
    img.save(&temp_png)
        .map_err(|e| format!("Impossible de sauvegarder PNG pivot : {}", e))?;

    // Réouvrir le PNG propre et encoder en JXL
    let clean_img = image::open(&temp_png)
        .map_err(|e| format!("Impossible de réouvrir PNG pivot : {}", e))?;

    let out_str = out.to_string_lossy().to_string();
    let result = if encoder_jxl(&clean_img, &out_str) {
        Ok(())
    } else {
        Err(format!("Encodage JXL pivot échoué pour {:?}", input))
    };

    // Nettoyage
    let _ = std::fs::remove_file(&temp_png);
    result
}

/// Collecte récursivement les fichiers images (jpg, png, webp) dans un dossier.
pub fn collecter_sources_jxl(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut result = Vec::new();
    collecter_sources_jxl_inner(dir, &mut result);
    result
}

fn collecter_sources_jxl_inner(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip les dossiers " jxl" déjà créés
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                if !name.ends_with(" jxl") {
                    collecter_sources_jxl_inner(&path, out);
                }
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if est_source_jxl(&ext.to_lowercase()) {
                    out.push(path);
                }
            }
        }
    }
}