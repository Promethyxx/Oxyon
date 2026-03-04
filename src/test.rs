// ═══════════════════════════════════════════════════════════════
//  OXYON — Tests automatisés exhaustifs
//  Lance : cargo test
// ═══════════════════════════════════════════════════════════════

use std::path::Path;
use std::fs;
use std::sync::{Once, Mutex};

const TEST_AUDIO: &str = "tests/audio";
const TEST_DOC:   &str = "tests/doc";
const TEST_PIC:   &str = "tests/pic";
const TEST_FMT:   &str = "tests/formats";
const TEST_VIDEO: &str = "tests/video";
const OUT:        &str = "tests/_output";

static INIT: Once = Once::new();
/// Mutex global pour sérialiser les opérations ffmpeg (évite les crashs concurrents)
static FFMPEG_LOCK: Mutex<()> = Mutex::new(());

fn setup() {
    INIT.call_once(|| {
        let _ = crate::modules::binaries::extraire_deps();
    });
    let _ = fs::create_dir_all(OUT);
}

/// Verrouille le mutex ffmpeg, exécute le spawn PUIS attend la fin (séquentiel garanti)
fn run_ffmpeg<F>(spawn_fn: F, context: &str)
where F: FnOnce() -> Result<std::process::Child, std::io::Error>
{
    let _lock = FFMPEG_LOCK.lock().unwrap();
    let result = spawn_fn();
    assert!(result.is_ok(), "{context} spawn échoué : {:?}", result.err());
    let status = result.unwrap().wait().unwrap();
    assert!(status.success(), "{context} code={:?}", status.code());
}

fn assert_output(path: &str, context: &str) {
    let p = Path::new(path);
    assert!(p.exists(), "ÉCHEC {context} — fichier absent : {path}");
    let size = fs::metadata(p).unwrap().len();
    assert!(size > 0, "ÉCHEC {context} — fichier vide : {path}");
    println!("  OK {context} — {} octets", size);
}

fn cleanup(path: &str) {
    let _ = fs::remove_file(path);
}

// ═══════════════════════════════════════════════════════════════
//  AUDIO — conversion entre formats
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_audio_mp3_vers_wav() {
    setup();
    let input = format!("{TEST_AUDIO}/MP3.mp3");
    let output = format!("{OUT}/audio_mp3_to_wav.wav");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::audio::convertir(Path::new(&input), &output, 1), "audio mp3→wav");
    assert_output(&output, "audio mp3→wav");
    cleanup(&output);
}

#[test]
fn test_audio_wav_vers_flac() {
    setup();
    let input = format!("{TEST_AUDIO}/WAV.wav");
    let output = format!("{OUT}/audio_wav_to_flac.flac");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::audio::convertir(Path::new(&input), &output, 1), "audio wav→flac");
    assert_output(&output, "audio wav→flac");
    cleanup(&output);
}

#[test]
fn test_audio_ogg_vers_mp3() {
    setup();
    let input = format!("{TEST_AUDIO}/OGG.ogg");
    let output = format!("{OUT}/audio_ogg_to_mp3.mp3");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::audio::convertir(Path::new(&input), &output, 1), "audio ogg→mp3");
    assert_output(&output, "audio ogg→mp3");
    cleanup(&output);
}

#[test]
fn test_audio_aac_vers_mp3() {
    setup();
    let input = format!("{TEST_AUDIO}/AAC.aac");
    let output = format!("{OUT}/audio_aac_to_mp3.mp3");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::audio::convertir(Path::new(&input), &output, 1), "audio aac→mp3");
    assert_output(&output, "audio aac→mp3");
    cleanup(&output);
}

#[test]
fn test_audio_flac_vers_mp3() {
    setup();
    let input = format!("{TEST_AUDIO}/FLAC.flac");
    let output = format!("{OUT}/audio_flac_to_mp3.mp3");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::audio::convertir(Path::new(&input), &output, 1), "audio flac→mp3");
    assert_output(&output, "audio flac→mp3");
    cleanup(&output);
}

#[test]
fn test_audio_wav_vers_mp3() {
    setup();
    let input = format!("{TEST_AUDIO}/WAV.wav");
    let output = format!("{OUT}/audio_wav_to_mp3.mp3");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::audio::convertir(Path::new(&input), &output, 1), "audio wav→mp3");
    assert_output(&output, "audio wav→mp3");
    cleanup(&output);
}

#[test]
fn test_audio_mp3_vers_ogg() {
    setup();
    let input = format!("{TEST_AUDIO}/MP3.mp3");
    let output = format!("{OUT}/audio_mp3_to_ogg.ogg");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::audio::convertir(Path::new(&input), &output, 1), "audio mp3→ogg");
    assert_output(&output, "audio mp3→ogg");
    cleanup(&output);
}

#[test]
fn test_audio_mp3_vers_aac() {
    setup();
    let input = format!("{TEST_AUDIO}/MP3.mp3");
    let output = format!("{OUT}/audio_mp3_to_aac.aac");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::audio::convertir(Path::new(&input), &output, 1), "audio mp3→aac");
    assert_output(&output, "audio mp3→aac");
    cleanup(&output);
}

#[test]
fn test_audio_detecter_extension_ogg() {
    setup();
    let ext = crate::modules::audio::detecter_extension(Path::new(&format!("{TEST_AUDIO}/OGG.ogg")));
    println!("  codec OGG: '{}'", ext);
    assert!(!ext.is_empty(), "detecter_extension vide pour OGG");
}

#[test]
fn test_audio_detecter_extension_mp3() {
    setup();
    let ext = crate::modules::audio::detecter_extension(Path::new(&format!("{TEST_AUDIO}/MP3.mp3")));
    println!("  codec MP3: '{}'", ext);
    assert!(!ext.is_empty(), "detecter_extension vide pour MP3");
}

#[test]
fn test_audio_detecter_extension_flac() {
    setup();
    let ext = crate::modules::audio::detecter_extension(Path::new(&format!("{TEST_AUDIO}/FLAC.flac")));
    println!("  codec FLAC: '{}'", ext);
    assert!(!ext.is_empty(), "detecter_extension vide pour FLAC");
}

#[test]
fn test_audio_formats_compatibles() {
    let fmts = crate::modules::audio::formats_compatibles("mp3");
    assert!(!fmts.is_empty(), "formats_compatibles mp3 est vide");
    assert!(fmts.contains(&"wav"), "mp3 devrait pouvoir aller vers wav");
    println!("  formats compatibles mp3: {:?}", fmts);
}

// ═══════════════════════════════════════════════════════════════
//  IMAGE — compresser tous formats
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_pic_compresser_jpg() {
    setup();
    let output = format!("{OUT}/pic_jpg_c.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::compresser(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output, 2));
    assert_output(&output, "compresser JPG");
    cleanup(&output);
}

#[test]
fn test_pic_compresser_png() {
    setup();
    let output = format!("{OUT}/pic_png_c.png");
    cleanup(&output);
    assert!(crate::modules::pic::compresser(Path::new(&format!("{TEST_PIC}/PNG.png")), &output, 2));
    assert_output(&output, "compresser PNG");
    cleanup(&output);
}

#[test]
fn test_pic_compresser_webp() {
    setup();
    let output = format!("{OUT}/pic_webp_c.png");
    cleanup(&output);
    assert!(crate::modules::pic::compresser(Path::new(&format!("{TEST_PIC}/WEBP.webp")), &output, 2));
    assert_output(&output, "compresser WEBP");
    cleanup(&output);
}

#[test]
fn test_pic_compresser_gif() {
    setup();
    let output = format!("{OUT}/pic_gif_c.png");
    cleanup(&output);
    assert!(crate::modules::pic::compresser(Path::new(&format!("{TEST_PIC}/GIF.gif")), &output, 2));
    assert_output(&output, "compresser GIF");
    cleanup(&output);
}

#[test]
fn test_pic_compresser_svg() {
    setup();
    let output = format!("{OUT}/pic_svg_c.png");
    cleanup(&output);
    assert!(crate::modules::pic::compresser(Path::new(&format!("{TEST_PIC}/SVG.svg")), &output, 1));
    assert_output(&output, "compresser SVG");
    cleanup(&output);
}

#[test]
fn test_pic_compresser_psd() {
    setup();
    let output = format!("{OUT}/pic_psd_c.png");
    cleanup(&output);
    assert!(crate::modules::pic::compresser(Path::new(&format!("{TEST_PIC}/PSD.psd")), &output, 1));
    assert_output(&output, "compresser PSD");
    cleanup(&output);
}

#[test]
fn test_pic_compresser_ico() {
    setup();
    let output = format!("{OUT}/pic_ico_c.png");
    cleanup(&output);
    assert!(crate::modules::pic::compresser(Path::new(&format!("{TEST_PIC}/ICO.ico")), &output, 2));
    assert_output(&output, "compresser ICO");
    cleanup(&output);
}

#[test]
fn test_pic_compresser_tiff() {
    setup();
    let output = format!("{OUT}/pic_tiff_c.png");
    cleanup(&output);
    assert!(crate::modules::pic::compresser(Path::new(&format!("{TEST_PIC}/Tiff.tiff")), &output, 2));
    assert_output(&output, "compresser TIFF");
    cleanup(&output);
}

// ═══════════════════════════════════════════════════════════════
//  IMAGE — pivoter
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_pic_pivoter_90() {
    setup();
    let output = format!("{OUT}/pic_rot90.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::pivoter(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output, 90));
    assert_output(&output, "pivoter 90°");
    cleanup(&output);
}

#[test]
fn test_pic_pivoter_180() {
    setup();
    let output = format!("{OUT}/pic_rot180.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::pivoter(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output, 180));
    assert_output(&output, "pivoter 180°");
    cleanup(&output);
}

#[test]
fn test_pic_pivoter_270() {
    setup();
    let output = format!("{OUT}/pic_rot270.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::pivoter(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output, 270));
    assert_output(&output, "pivoter 270°");
    cleanup(&output);
}

// ═══════════════════════════════════════════════════════════════
//  IMAGE — recadrer, redimensionner
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_pic_recadrer() {
    setup();
    let output = format!("{OUT}/pic_crop.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::recadrer(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output, 10, 10, 50, 50));
    assert_output(&output, "recadrer");
    cleanup(&output);
}

#[test]
fn test_pic_redimensionner_pixels() {
    setup();
    let output = format!("{OUT}/pic_resize_px.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::redimensionner_pixels(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output, 200, 150));
    assert_output(&output, "resize 200x150");
    cleanup(&output);
}

#[test]
fn test_pic_redimensionner_poids() {
    setup();
    let output = format!("{OUT}/pic_resize_kb.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::redimensionner_poids(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output, 50));
    assert_output(&output, "resize max 50Ko");
    cleanup(&output);
}

// ═══════════════════════════════════════════════════════════════
//  IMAGE — conversions entre formats
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_pic_convertir_jpg_vers_png() {
    setup();
    let output = format!("{OUT}/pic_jpg2png.png");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output));
    assert_output(&output, "JPG→PNG");
    cleanup(&output);
}

#[test]
fn test_pic_convertir_png_vers_jpg() {
    setup();
    let output = format!("{OUT}/pic_png2jpg.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/PNG.png")), &output));
    assert_output(&output, "PNG→JPG");
    cleanup(&output);
}

#[test]
fn test_pic_convertir_svg_vers_png() {
    setup();
    let output = format!("{OUT}/pic_svg2png.png");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/SVG.svg")), &output));
    assert_output(&output, "SVG→PNG");
    cleanup(&output);
}

#[test]
fn test_pic_convertir_jpg_vers_webp() {
    setup();
    let output = format!("{OUT}/pic_jpg2webp.webp");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output));
    assert_output(&output, "JPG→WEBP");
    cleanup(&output);
}

#[test]
fn test_pic_convertir_png_vers_webp() {
    setup();
    let output = format!("{OUT}/pic_png2webp.webp");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/PNG.png")), &output));
    assert_output(&output, "PNG→WEBP");
    cleanup(&output);
}

#[test]
fn test_pic_convertir_webp_vers_png() {
    setup();
    let output = format!("{OUT}/pic_webp2png.png");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/WEBP.webp")), &output));
    assert_output(&output, "WEBP→PNG");
    cleanup(&output);
}

#[test]
fn test_pic_convertir_gif_vers_png() {
    setup();
    let output = format!("{OUT}/pic_gif2png.png");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/GIF.gif")), &output));
    assert_output(&output, "GIF→PNG");
    cleanup(&output);
}

#[test]
fn test_pic_convertir_tiff_vers_jpg() {
    setup();
    let output = format!("{OUT}/pic_tiff2jpg.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/Tiff.tiff")), &output));
    assert_output(&output, "TIFF→JPG");
    cleanup(&output);
}


#[test]
fn test_pic_convertir_jpg_vers_jxl() {
    setup();
    let output = format!("{OUT}/pic_jpg2jxl.jxl");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output));
    assert_output(&output, "JPG→JXL");
    cleanup(&output);
}

#[test]
fn test_pic_convertir_png_vers_jxl() {
    setup();
    let output = format!("{OUT}/pic_png2jxl.jxl");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/PNG.png")), &output));
    assert_output(&output, "PNG→JXL");
    cleanup(&output);
}

#[test]
fn test_pic_convertir_jxl_vers_jpg() {
    setup();
    let output = format!("{OUT}/pic_jxl2jpg.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/JXL.jxl")), &output));
    assert_output(&output, "JXL→JPG");
    cleanup(&output);
}

#[test]
fn test_pic_convertir_jxl_vers_png() {
    setup();
    let output = format!("{OUT}/pic_jxl2png.png");
    cleanup(&output);
    assert!(crate::modules::pic::convertir(Path::new(&format!("{TEST_PIC}/JXL.jxl")), &output));
    assert_output(&output, "JXL→PNG");
    cleanup(&output);
}

// ═══════════════════════════════════════════════════════════════
//  IMAGE — EXIF
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_pic_lire_exif() {
    setup();
    let tags = crate::modules::pic::lire_exif(Path::new(&format!("{TEST_PIC}/JPG.jpg")));
    println!("  EXIF JPG: {} entrées", tags.len());
}

#[test]
fn test_pic_supprimer_exif() {
    setup();
    let output = format!("{OUT}/pic_no_exif.jpg");
    cleanup(&output);
    assert!(crate::modules::pic::supprimer_exif(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output));
    assert_output(&output, "supprimer EXIF");
    cleanup(&output);
}

// ═══════════════════════════════════════════════════════════════
//  DOC — conversion pandoc
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_doc_convertir_md_vers_pdf() {
    setup();
    let output = format!("{OUT}/doc_md2pdf.pdf");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/MD.md")), &output), "MD→PDF échoué");
    assert_output(&output, "MD→PDF");
    cleanup(&output);
}

#[test]
fn test_doc_convertir_md_vers_html() {
    setup();
    let output = format!("{OUT}/doc_md2html.html");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/MD.md")), &output), "MD→HTML échoué");
    assert_output(&output, "MD→HTML");
    cleanup(&output);
}

#[test]
fn test_doc_convertir_md_vers_docx() {
    setup();
    let output = format!("{OUT}/doc_md2docx.docx");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/MD.md")), &output), "MD→DOCX échoué");
    assert_output(&output, "MD→DOCX");
    cleanup(&output);
}

#[test]
fn test_doc_convertir_docx_vers_pdf() {
    setup();
    let output = format!("{OUT}/doc_docx2pdf.pdf");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/DOCX.docx")), &output), "DOCX→PDF échoué");
    assert_output(&output, "DOCX→PDF");
    cleanup(&output);
}

#[test]
fn test_doc_convertir_docx_vers_html() {
    setup();
    let output = format!("{OUT}/doc_docx2html.html");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/DOCX.docx")), &output), "DOCX→HTML échoué");
    assert_output(&output, "DOCX→HTML");
    cleanup(&output);
}

#[test]
fn test_doc_convertir_html_vers_pdf() {
    setup();
    let output = format!("{OUT}/doc_html2pdf.pdf");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/HTML.html")), &output), "HTML→PDF échoué");
    assert_output(&output, "HTML→PDF");
    cleanup(&output);
}

#[test]
fn test_doc_convertir_html_vers_md() {
    setup();
    let output = format!("{OUT}/doc_html2md.md");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/HTML.html")), &output), "HTML→MD échoué");
    assert_output(&output, "HTML→MD");
    cleanup(&output);
}

#[test]
fn test_doc_convertir_txt_vers_pdf() {
    setup();
    let output = format!("{OUT}/doc_txt2pdf.pdf");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/TXT.txt")), &output), "TXT→PDF échoué");
    assert_output(&output, "TXT→PDF");
    cleanup(&output);
}

#[test]
fn test_doc_convertir_txt_vers_html() {
    setup();
    let output = format!("{OUT}/doc_txt2html.html");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/TXT.txt")), &output), "TXT→HTML échoué");
    assert_output(&output, "TXT→HTML");
    cleanup(&output);
}

#[test]
fn test_doc_convertir_odt_vers_pdf() {
    setup();
    let output = format!("{OUT}/doc_odt2pdf.pdf");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/ODT.odt")), &output), "ODT→PDF échoué");
    assert_output(&output, "ODT→PDF");
    cleanup(&output);
}

#[test]
fn test_doc_convertir_odt_vers_html() {
    setup();
    let output = format!("{OUT}/doc_odt2html.html");
    cleanup(&output);
    assert!(crate::modules::doc::convertir(Path::new(&format!("{TEST_DOC}/ODT.odt")), &output), "ODT→HTML échoué");
    assert_output(&output, "ODT→HTML");
    cleanup(&output);
}

// ═══════════════════════════════════════════════════════════════
//  DOC — opérations PDF
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_doc_pdf_split() {
    setup();
    let output_dir = format!("{OUT}/pdf_split_pages");
    let _ = fs::create_dir_all(&output_dir);
    let result = crate::modules::doc::pdf_split(Path::new(&format!("{TEST_FMT}/PDF.pdf")), &output_dir);
    assert!(result.is_ok(), "pdf_split échoué : {:?}", result);
    let _ = fs::remove_dir_all(&output_dir);
}

#[test]
fn test_doc_pdf_merge() {
    setup();
    let input = format!("{TEST_FMT}/PDF.pdf");
    let output = format!("{OUT}/doc_pdf_merged.pdf");
    cleanup(&output);
    let p = Path::new(&input);
    let result = crate::modules::doc::pdf_merge(&[p, p], &output);
    assert!(result.is_ok(), "pdf_merge échoué : {:?}", result);
    assert_output(&output, "pdf merge");
    cleanup(&output);
}

#[test]
fn test_doc_pdf_rotate() {
    setup();
    let output = format!("{OUT}/doc_pdf_rot90.pdf");
    cleanup(&output);
    let result = crate::modules::doc::pdf_rotate(Path::new(&format!("{TEST_FMT}/PDF.pdf")), &output, 90, None);
    assert!(result.is_ok(), "pdf_rotate 90 échoué : {:?}", result);
    assert_output(&output, "pdf rotate 90°");
    cleanup(&output);
}

#[test]
fn test_doc_pdf_rotate_180() {
    setup();
    let output = format!("{OUT}/doc_pdf_rot180.pdf");
    cleanup(&output);
    let result = crate::modules::doc::pdf_rotate(Path::new(&format!("{TEST_FMT}/PDF.pdf")), &output, 180, None);
    assert!(result.is_ok(), "pdf_rotate 180 échoué : {:?}", result);
    assert_output(&output, "pdf rotate 180°");
    cleanup(&output);
}

#[test]
fn test_doc_pdf_rotate_pages_specifiques() {
    setup();
    let output = format!("{OUT}/doc_pdf_rot_p1.pdf");
    cleanup(&output);
    let pages = vec![1u32];
    let result = crate::modules::doc::pdf_rotate(Path::new(&format!("{TEST_FMT}/PDF.pdf")), &output, 90, Some(&pages));
    assert!(result.is_ok(), "pdf_rotate page 1 échoué : {:?}", result);
    assert_output(&output, "pdf rotate page 1");
    cleanup(&output);
}

#[test]
fn test_doc_pdf_compress() {
    setup();
    let output = format!("{OUT}/doc_pdf_comp.pdf");
    cleanup(&output);
    let result = crate::modules::doc::pdf_compresser(Path::new(&format!("{TEST_FMT}/PDF.pdf")), &output);
    assert!(result.is_ok(), "pdf_compresser échoué : {:?}", result);
    assert_output(&output, "pdf compress");
    cleanup(&output);
}

#[test]
fn test_doc_pdf_crop() {
    setup();
    let output = format!("{OUT}/doc_pdf_crop.pdf");
    cleanup(&output);
    let result = crate::modules::doc::pdf_crop(Path::new(&format!("{TEST_FMT}/PDF.pdf")), &output, 10.0, 10.0, 80.0, 80.0, None);
    assert!(result.is_ok(), "pdf_crop échoué : {:?}", result);
    assert_output(&output, "pdf crop");
    cleanup(&output);
}

#[test]
fn test_doc_pdf_organiser() {
    setup();
    let output = format!("{OUT}/doc_pdf_org.pdf");
    cleanup(&output);
    let result = crate::modules::doc::pdf_organiser(Path::new(&format!("{TEST_FMT}/PDF.pdf")), &output, &[1]);
    assert!(result.is_ok(), "pdf_organiser échoué : {:?}", result);
    assert_output(&output, "pdf organiser");
    cleanup(&output);
}

#[test]
fn test_doc_pdf_supprimer_pages() {
    setup();
    // Merger pour avoir multi-pages
    let input = format!("{TEST_FMT}/PDF.pdf");
    let merged = format!("{OUT}/doc_pdf_for_del.pdf");
    cleanup(&merged);
    let p = Path::new(&input);
    let _ = crate::modules::doc::pdf_merge(&[p, p, p], &merged);

    let output = format!("{OUT}/doc_pdf_del.pdf");
    cleanup(&output);
    let result = crate::modules::doc::pdf_supprimer_pages(Path::new(&merged), &output, &[2]);
    assert!(result.is_ok(), "pdf_supprimer_pages échoué : {:?}", result);
    assert_output(&output, "pdf supprimer page");
    cleanup(&output);
    cleanup(&merged);
}

#[test]
fn test_doc_pdf_numeroter() {
    setup();
    let output = format!("{OUT}/doc_pdf_num.pdf");
    cleanup(&output);
    let result = crate::modules::doc::pdf_numeroter(
        Path::new(&format!("{TEST_FMT}/PDF.pdf")), &output, 1,
        crate::modules::doc::PositionNumero::BasCentre, 10.0
    );
    assert!(result.is_ok(), "pdf_numeroter échoué : {:?}", result);
    assert_output(&output, "pdf numéroter");
    cleanup(&output);
}

#[test]
fn test_doc_pdf_watermark() {
    setup();
    let output = format!("{OUT}/doc_pdf_wm.pdf");
    cleanup(&output);
    let result = crate::modules::doc::pdf_watermark(Path::new(&format!("{TEST_FMT}/PDF.pdf")), &output, "TEST", 40.0, 0.3, None);
    assert!(result.is_ok(), "pdf_watermark échoué : {:?}", result);
    assert_output(&output, "pdf watermark");
    cleanup(&output);
}

#[test]
fn test_doc_pdf_repair() {
    setup();
    let output = format!("{OUT}/doc_pdf_rep.pdf");
    cleanup(&output);
    let result = crate::modules::doc::pdf_reparer(Path::new(&format!("{TEST_FMT}/PDF.pdf")), &output);
    assert!(result.is_ok(), "pdf_reparer échoué : {:?}", result);
    assert_output(&output, "pdf repair");
    cleanup(&output);
}

#[test]
fn test_doc_pdf_protect_unlock() {
    setup();
    let protected = format!("{OUT}/doc_pdf_prot.pdf");
    let unlocked = format!("{OUT}/doc_pdf_unlk.pdf");
    cleanup(&protected);
    cleanup(&unlocked);

    let result = crate::modules::doc::pdf_proteger(Path::new(&format!("{TEST_FMT}/PDF.pdf")), &protected, "owner123", "user123", true, false);
    assert!(result.is_ok(), "pdf_proteger échoué : {:?}", result);
    assert_output(&protected, "pdf protect");

    let result = crate::modules::doc::pdf_dechiffrer(Path::new(&protected), &unlocked, "owner123");
    assert!(result.is_ok(), "pdf_dechiffrer échoué : {:?}", result);
    assert_output(&unlocked, "pdf unlock");

    cleanup(&protected);
    cleanup(&unlocked);
}

// ═══════════════════════════════════════════════════════════════
//  ARCHIVE — zip, 7z, tar + extraction
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_archive_compresser_zip() {
    setup();
    let output = format!("{OUT}/arc_test.zip");
    cleanup(&output);
    assert!(crate::modules::archive::compresser(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output, "zip", 6));
    assert_output(&output, "archive zip");
    cleanup(&output);
}

#[test]
fn test_archive_compresser_7z() {
    setup();
    let output = format!("{OUT}/arc_test.7z");
    cleanup(&output);
    assert!(crate::modules::archive::compresser(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output, "7z", 6));
    assert_output(&output, "archive 7z");
    cleanup(&output);
}

#[test]
fn test_archive_compresser_tar() {
    setup();
    let output = format!("{OUT}/arc_test.tar.gz");
    cleanup(&output);
    assert!(crate::modules::archive::compresser(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &output, "tar", 6));
    assert_output(&output, "archive tar");
    cleanup(&output);
}

#[test]
fn test_archive_extraire_zip() {
    setup();
    let zip = format!("{OUT}/arc_ext.zip");
    let dir = format!("{OUT}/arc_ext_zip");
    cleanup(&zip);
    let _ = fs::remove_dir_all(&dir);
    assert!(crate::modules::archive::compresser(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &zip, "zip", 6));
    let _ = fs::create_dir_all(&dir);
    assert!(crate::modules::archive::extraire(Path::new(&zip), &dir), "extraction zip échouée");
    cleanup(&zip);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_archive_extraire_7z() {
    setup();
    let sz = format!("{OUT}/arc_ext.7z");
    let dir = format!("{OUT}/arc_ext_7z");
    cleanup(&sz);
    let _ = fs::remove_dir_all(&dir);
    assert!(crate::modules::archive::compresser(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &sz, "7z", 6));
    let _ = fs::create_dir_all(&dir);
    assert!(crate::modules::archive::extraire(Path::new(&sz), &dir), "extraction 7z échouée");
    cleanup(&sz);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_archive_extraire_tar() {
    setup();
    let tar = format!("{OUT}/arc_ext.tar.gz");
    let dir = format!("{OUT}/arc_ext_tar");
    cleanup(&tar);
    let _ = fs::remove_dir_all(&dir);
    assert!(crate::modules::archive::compresser(Path::new(&format!("{TEST_PIC}/JPG.jpg")), &tar, "tar", 6));
    let _ = fs::create_dir_all(&dir);
    assert!(crate::modules::archive::extraire(Path::new(&tar), &dir), "extraction tar échouée");
    cleanup(&tar);
    let _ = fs::remove_dir_all(&dir);
}

// ═══════════════════════════════════════════════════════════════
//  VIDEO — conversions
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_video_mkv_vers_mp4() {
    setup();
    let input = std::path::PathBuf::from(format!("{TEST_VIDEO}/MKV.mkv"));
    let output = format!("{OUT}/vid_mkv2mp4.mp4");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::video::traiter_video(&input, &output, false, false, 1), "mkv→mp4");
    assert_output(&output, "mkv→mp4");
    cleanup(&output);
}

#[test]
fn test_video_mp4_vers_mkv() {
    setup();
    let input = std::path::PathBuf::from(format!("{TEST_VIDEO}/MP4.mp4"));
    let output = format!("{OUT}/vid_mp42mkv.mkv");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::video::traiter_video(&input, &output, false, false, 1), "mp4→mkv");
    assert_output(&output, "mp4→mkv");
    cleanup(&output);
}

#[test]
fn test_video_webm_vers_mp4() {
    setup();
    let input = std::path::PathBuf::from(format!("{TEST_VIDEO}/WEBM.webm"));
    let output = format!("{OUT}/vid_webm2mp4.mp4");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::video::traiter_video(&input, &output, false, false, 1), "webm→mp4");
    assert_output(&output, "webm→mp4");
    cleanup(&output);
}

#[test]
fn test_video_copie_flux() {
    setup();
    let input = std::path::PathBuf::from(format!("{TEST_VIDEO}/MKV.mkv"));
    let output = format!("{OUT}/vid_copy.mp4");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::video::traiter_video(&input, &output, true, false, 1), "copie flux");
    assert_output(&output, "copie flux");
    cleanup(&output);
}
// ═══════════════════════════════════════════════════════════════
//  AUDIO — extraction depuis vidéo
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_audio_extraire_depuis_mkv() {
    setup();
    let input = format!("{TEST_VIDEO}/MKV.mkv");
    let output = format!("{OUT}/audio_extrait.mkv");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::audio::extraire(Path::new(&input), &output), "audio extraire mkv");
    assert_output(&output, "audio extraire mkv");
    cleanup(&output);
}

#[test]
fn test_audio_extraire_depuis_mp4() {
    setup();
    let input = format!("{TEST_VIDEO}/MP4.mp4");
    let output = format!("{OUT}/audio_extrait_mp4.mkv");
    cleanup(&output);
        run_ffmpeg(|| crate::modules::audio::extraire(Path::new(&input), &output), "audio extraire mp4");
    assert_output(&output, "audio extraire mp4");
    cleanup(&output);
}

// ═══════════════════════════════════════════════════════════════
//  DOC — détection de formats
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_doc_detecter_format_entree_md() {
    setup();
    let fmt = crate::modules::doc::detecter_format_entree(Path::new(&format!("{TEST_DOC}/MD.md")));
    println!("  format entree MD: {:?}", fmt);
}

#[test]
fn test_doc_detecter_format_entree_docx() {
    setup();
    let fmt = crate::modules::doc::detecter_format_entree(Path::new(&format!("{TEST_DOC}/DOCX.docx")));
    println!("  format entree DOCX: {:?}", fmt);
}

#[test]
fn test_doc_detecter_format_entree_html() {
    setup();
    let fmt = crate::modules::doc::detecter_format_entree(Path::new(&format!("{TEST_DOC}/HTML.html")));
    println!("  format entree HTML: {:?}", fmt);
}

#[test]
fn test_doc_detecter_format_sortie_pdf() {
    setup();
    let fmt = crate::modules::doc::detecter_format_sortie("output.pdf");
    println!("  format sortie pdf: {:?}", fmt);
}

#[test]
fn test_doc_detecter_format_sortie_html() {
    setup();
    let fmt = crate::modules::doc::detecter_format_sortie("output.html");
    println!("  format sortie html: {:?}", fmt);
}

// ═══════════════════════════════════════════════════════════════
//  RENAME
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_rename_find_replace() {
    let cfg = crate::modules::rename::RenameConfig {
        find: "foo".into(),
        replace_with: "bar".into(),
        ..Default::default()
    };
    let files = vec![std::path::PathBuf::from("foo_test.jpg")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "bar_test.jpg");
    println!("  rename find/replace: OK");
}

#[test]
fn test_rename_insert() {
    let cfg = crate::modules::rename::RenameConfig {
        insert_text: "PREFIX_".into(),
        insert_pos: 0,
        ..Default::default()
    };
    let files = vec![std::path::PathBuf::from("test.jpg")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "PREFIX_test.jpg");
    println!("  rename insert: OK");
}

#[test]
fn test_rename_delete_range() {
    let cfg = crate::modules::rename::RenameConfig {
        delete_enabled: true,
        delete_from: 0,
        delete_count: 3,
        ..Default::default()
    };
    let files = vec![std::path::PathBuf::from("abctest.jpg")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "test.jpg");
    println!("  rename delete range: OK");
}

#[test]
fn test_rename_numbering_suffix() {
    let cfg = crate::modules::rename::RenameConfig {
        num_enabled: true,
        num_start: 1,
        num_step: 1,
        num_padding: 2,
        num_pos: crate::modules::rename::NumPos::Suffix,
        num_sep: "_".into(),
        ..Default::default()
    };
    let files = vec![
        std::path::PathBuf::from("photo.jpg"),
        std::path::PathBuf::from("photo.jpg"),
    ];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "photo_01.jpg");
    assert_eq!(previews[1].1, "photo_02.jpg");
    println!("  rename numbering suffix: OK");
}

#[test]
fn test_rename_numbering_prefix() {
    let cfg = crate::modules::rename::RenameConfig {
        num_enabled: true,
        num_start: 10,
        num_step: 2,
        num_padding: 3,
        num_pos: crate::modules::rename::NumPos::Prefix,
        num_sep: " - ".into(),
        ..Default::default()
    };
    let files = vec![std::path::PathBuf::from("file.mp4")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "010 - file.mp4");
    println!("  rename numbering prefix: OK");
}

#[test]
fn test_rename_case_upper() {
    let cfg = crate::modules::rename::RenameConfig {
        case_mode: crate::modules::rename::CaseMode::Upper,
        ..Default::default()
    };
    let files = vec![std::path::PathBuf::from("hello world.jpg")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "HELLO WORLD.jpg");
    println!("  rename case upper: OK");
}

#[test]
fn test_rename_case_title() {
    let cfg = crate::modules::rename::RenameConfig {
        case_mode: crate::modules::rename::CaseMode::Title,
        ..Default::default()
    };
    let files = vec![std::path::PathBuf::from("hello world.jpg")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "Hello World.jpg");
    println!("  rename case title: OK");
}

#[test]
fn test_rename_ext_lower() {
    let cfg = crate::modules::rename::RenameConfig {
        ext_mode: crate::modules::rename::ExtMode::Lower,
        ..Default::default()
    };
    let files = vec![std::path::PathBuf::from("FILE.JPG")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "FILE.jpg");
    println!("  rename ext lower: OK");
}

#[test]
fn test_rename_ext_replace() {
    let cfg = crate::modules::rename::RenameConfig {
        ext_mode: crate::modules::rename::ExtMode::Replace,
        ext_new: "png".into(),
        ..Default::default()
    };
    let files = vec![std::path::PathBuf::from("image.jpg")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "image.png");
    println!("  rename ext replace: OK");
}

#[test]
fn test_rename_strip_double_spaces() {
    let cfg = crate::modules::rename::RenameConfig {
        strip_double_spaces: true,
        ..Default::default()
    };
    let files = vec![std::path::PathBuf::from("hello  world.jpg")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "hello world.jpg");
    println!("  rename strip double spaces: OK");
}

#[test]
fn test_rename_strip_chars() {
    let cfg = crate::modules::rename::RenameConfig {
        strip_chars: "!?#".into(),
        ..Default::default()
    };
    let files = vec![std::path::PathBuf::from("he!llo?wo#rld.jpg")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "helloworld.jpg");
    println!("  rename strip chars: OK");
}

#[test]
fn test_rename_no_change() {
    let cfg = crate::modules::rename::RenameConfig::default();
    let files = vec![std::path::PathBuf::from("unchanged.mp4")];
    let previews = crate::modules::rename::preview(&files, &cfg);
    assert_eq!(previews[0].1, "unchanged.mp4");
    println!("  rename no change: OK");
}