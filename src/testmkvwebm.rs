use std::path::Path;
use std::fs;
use std::sync::Once;

const TEST_VIDEO: &str = "tests/video";
const OUT:        &str = "tests/_output";

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        let _ = crate::modules::binaries::extraire_deps();
    });
    let _ = fs::create_dir_all(OUT);
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
//  VIDEO — conversions
// ═══════════════════════════════════════════════════════════════
#[test]
fn test_video_mkv_vers_webm() {
    setup();
    let input = std::path::PathBuf::from(format!("{TEST_VIDEO}/MKV.mkv"));
    let output = format!("{OUT}/vid_mkv2webm.webm");
    cleanup(&output);
    let result = crate::modules::video::traiter_video(&input, &output, false, false);
    assert!(result.is_ok(), "mkv→webm spawn échoué");
    let status = result.unwrap().wait().unwrap();
    assert!(status.success(), "mkv→webm code={:?}", status.code());
    assert_output(&output, "mkv→webm");
    cleanup(&output);
}