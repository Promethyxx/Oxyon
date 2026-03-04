#![allow(dead_code)]
use std::path::PathBuf;
use std::process::Child;
use crate::modules::binaries;

/// Convertit ou change le conteneur d'une vidéo
pub fn traiter_video(
    input: &PathBuf,
    output: &str,
    copie_flux: bool,
    est_audio_uniquement: bool,
    speed: u32,
) -> Result<Child, std::io::Error> {
    let ffmpeg = binaries::get_ffmpeg();
    crate::log_info(&format!(
        "video::traiter_video | ffmpeg={:?} | copie_flux={} | audio_only={} | speed={} | {:?} -> {}",
        ffmpeg, copie_flux, est_audio_uniquement, speed, input, output
    ));
    let mut args = vec!["-i".to_string(), input.to_str().unwrap().to_string()];

    if copie_flux {
        if est_audio_uniquement {
            args.extend(["-vn", "-c:a", "copy"].map(String::from));
        } else {
            args.extend(["-c", "copy"].map(String::from));
        }
    } else if est_audio_uniquement {
        args.push("-vn".to_string());
    } else {
        // Appliquer preset selon le format de sortie
        let ext = std::path::Path::new(output)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        match ext.as_str() {
            "webm" => {
                let threads = num_cpus();
                args.extend(["-c:v", "libvpx-vp9", "-row-mt", "1"].map(String::from));
                args.extend(["-threads".to_string(), threads.to_string()]);
                args.extend(["-speed".to_string(), speed.to_string()]);
            },
            "mp4" | "mkv" | "mov" => {
                let preset = match speed {
                    0..=1 => "slow",
                    2..=4 => "medium",
                    5..=6 => "fast",
                    _     => "ultrafast",
                };
                args.extend(["-c:v", "libx264", "-preset", preset].map(String::from));
            },
            _ => {}
        }
    }

    args.extend(["-y".to_string(), output.to_string()]);

    let child = binaries::silent_cmd(binaries::get_ffmpeg())
        .args(&args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();
    if let Err(ref e) = child {
        crate::log_error(&format!("video::traiter_video impossible de lancer ffmpeg : {}", e));
    }
    child
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

/// Analyse le codec audio d'un fichier via ffprobe
pub fn extraire_nom_codec(input: &PathBuf) -> String {
    let out = binaries::silent_cmd(binaries::get_ffprobe())
        .args(&[
            "-v",
            "error",
            "-select_streams",
            "a:0",
            "-show_entries",
            "stream=codec_name",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            input.to_str().unwrap(),
        ])
        .output();

    if let Ok(o) = out {
        String::from_utf8_lossy(&o.stdout).trim().to_string()
    } else {
        String::new()
    }
}