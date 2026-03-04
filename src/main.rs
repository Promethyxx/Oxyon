#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
const VERSION: &str = env!("CARGO_PKG_VERSION");
pub mod lang;
pub mod modules;
#[cfg(test)]
#[path = "test.rs"]
#[cfg(test)]
#[path = "testmkvwebm.rs"]
mod test;
use eframe::egui;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use std::io::Write;

// ─────────────────────────────────────────────
//  SYSTÈME DE LOG
// ─────────────────────────────────────────────
fn log_entry(level: &str, message: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let thread = std::thread::current();
    let thread_name = thread.name().unwrap_or("worker");
    let log_line = format!("[{}] [{}] [{}] {}\n", timestamp, level, thread_name, message);
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("oxyon.log")
    {
        let _ = file.write_all(log_line.as_bytes());
    }
    // Aussi visible en console debug
    eprintln!("{}", log_line.trim());
}

fn log_info(msg: &str)  { log_entry("INFO",  msg); }
fn log_warn(msg: &str)  { log_entry("WARN",  msg); }
fn log_error(msg: &str) { log_entry("ERROR", msg); }

// ─────────────────────────────────────────────

#[cfg(feature = "api")]
#[derive(Clone)]
struct ScrapeEntry {
    data: modules::scrap::ScrapeResult,
    texture: Option<egui::TextureHandle>,
}
#[derive(PartialEq, Clone, Copy, Debug)]
enum ModuleType {
    #[cfg(feature = "api")]
    Archive,
    #[cfg(feature = "api")]
    Audio,
    Doc,
    Image,
    #[cfg(feature = "api")]
    Scrapper,
    #[cfg(feature = "api")]
    Tag,
    #[cfg(feature = "api")]
    Video,
    Rename,
    Settings,
}
struct OxyonApp {
    #[cfg(feature = "api")]
    copie_flux: bool,
    current_files: Vec<PathBuf>,
    current_stem: String,
    format_choisi: String,
        module_actif: ModuleType,
        process: Option<std::process::Child>,
        ratio_img: u32,
        #[cfg(feature = "api")]
        results_ui: Arc<Mutex<Vec<ScrapeEntry>>>,
        status: Arc<Mutex<String>>,
        deps_manquantes: Vec<String>,
        #[cfg(feature = "api")]
        audio_action: String,
        #[cfg(feature = "api")]
        audio_formats_dispo: Vec<String>,
        #[cfg(feature = "api")]
        tag_edit_val: String,
        current_theme: String,
        lang: &'static crate::lang::Lang,
        lang_id: &'static str,
        rename_cfg: modules::rename::RenameConfig,
        rename_previews: Vec<(std::path::PathBuf, String)>,
        rename_results: Vec<modules::rename::RenameResult>,
        tmdb_api_key: String,
        fanart_api_key: String,
        save_doc_format: bool,
        save_image_format: bool,
        #[cfg(feature = "api")]
        save_archive_format: bool,
        #[cfg(feature = "api")]
        archive_niveau: u32,
        #[cfg(feature = "api")]
        save_audio_format: bool,
        #[cfg(feature = "api")]
        audio_qualite: u32,
        #[cfg(feature = "api")]
        save_video_format: bool,
        #[cfg(feature = "api")]
        video_speed: u32,
        image_action: String,
        rotation_angle: u32,
        crop_x: u32,
        crop_y: u32,
        crop_width: u32,
        crop_height: u32,
        resize_width: String,
        resize_height: String,
        resize_max_kb: String,
        doc_action: String,
        pdf_rotation_angle: u16,
        pdf_pages_spec: String,
        pdf_crop_x: f64,
        pdf_crop_y: f64,
        pdf_crop_w: f64,
        pdf_crop_h: f64,
        pdf_num_debut: u32,
        pdf_num_position: String,
        pdf_num_taille: f64,
        pdf_owner_pass: String,
        pdf_user_pass: String,
        pdf_allow_print: bool,
        pdf_allow_copy: bool,
        pdf_unlock_pass: String,
        pdf_wm_texte: String,
        pdf_wm_taille: f64,
        pdf_wm_opacite: f64,
        pdf_nouvel_ordre: String,
        max_parallel_jobs: usize,
        active_jobs: Arc<Mutex<usize>>,
        completed_jobs: Arc<Mutex<usize>>,
        total_jobs: Arc<Mutex<usize>>,
        job_queue: Arc<Mutex<Vec<PathBuf>>>,
}
impl Default for OxyonApp {
    fn default() -> Self {
        Self {
            #[cfg(feature = "api")]
            copie_flux: false,
            current_files: Vec::new(),
            current_stem: String::new(),
            format_choisi: "mp4".into(),
                module_actif: ModuleType::Image,
                process: None,
                ratio_img: 2,
                #[cfg(feature = "api")]
                results_ui: Arc::new(Mutex::new(Vec::new())),
                status: Arc::new(Mutex::new(crate::lang::EN.drop_files.into())),
                deps_manquantes: Vec::new(),
                #[cfg(feature = "api")]
                audio_action: "Convert".into(),
                #[cfg(feature = "api")]
                audio_formats_dispo: vec!["aac","flac","m4a","mp3","ogg","opus","wav"].into_iter().map(String::from).collect(),
                #[cfg(feature = "api")]
                tag_edit_val: String::new(),
                current_theme: "Dark".into(),
                tmdb_api_key: String::new(),
                fanart_api_key: String::new(),
                save_doc_format: false,
                save_image_format: false,
                #[cfg(feature = "api")]
                save_archive_format: false,
                #[cfg(feature = "api")]
                archive_niveau: 6,
                #[cfg(feature = "api")]
                save_audio_format: false,
                #[cfg(feature = "api")]
                audio_qualite: 2,
                #[cfg(feature = "api")]
                save_video_format: false,
                #[cfg(feature = "api")]
                video_speed: 4,
                image_action: "Convert".into(),
                rotation_angle: 90,
                crop_x: 0,
                crop_y: 0,
                crop_width: 100,
                crop_height: 100,
                resize_width: String::new(),
                resize_height: String::new(),
                resize_max_kb: String::new(),
                doc_action: "Convert".into(),
                pdf_rotation_angle: 90,
                pdf_pages_spec: "1-end".into(),
                pdf_crop_x: 0.0,
                pdf_crop_y: 0.0,
                pdf_crop_w: 100.0,
                pdf_crop_h: 100.0,
                pdf_num_debut: 1,
                pdf_num_position: "BasCentre".into(),
                pdf_num_taille: 10.0,
                pdf_owner_pass: String::new(),
                pdf_user_pass: String::new(),
                pdf_allow_print: true,
                pdf_allow_copy: true,
                pdf_unlock_pass: String::new(),
                pdf_wm_texte: "CONFIDENTIEL".into(),
                pdf_wm_taille: 60.0,
                pdf_wm_opacite: 0.15,
                pdf_nouvel_ordre: String::new(),
                max_parallel_jobs: 4,
                active_jobs: Arc::new(Mutex::new(0)),
                completed_jobs: Arc::new(Mutex::new(0)),
                total_jobs: Arc::new(Mutex::new(0)),
                job_queue: Arc::new(Mutex::new(Vec::new())),
                rename_cfg: modules::rename::RenameConfig::default(),
                rename_previews: Vec::new(),
                rename_results: Vec::new(),
                lang: &crate::lang::EN,
                lang_id: "en",
        }
    }
}
impl OxyonApp {
    fn load_config(&mut self) {
        match self.module_actif {
            ModuleType::Image => self.format_choisi = String::new(),
            ModuleType::Doc => self.format_choisi = String::new(),
            ModuleType::Rename => {},
            #[cfg(feature = "api")]
            ModuleType::Video => self.format_choisi = String::new(),
            #[cfg(feature = "api")]
            ModuleType::Audio => self.format_choisi = String::new(),
            #[cfg(feature = "api")]
            ModuleType::Archive => self.format_choisi = String::new(),
            _ => (),
        }
        if let Ok(c) = std::fs::read_to_string("config.toml") {
            if let Ok(parsed) = c.parse::<toml::Table>() {
                if let Some(theme) = parsed.get("display").and_then(|d| d.get("theme")).and_then(|t| t.as_str()) {
                    self.current_theme = theme.to_string();
                }
                if let Some(max_jobs) = parsed.get("performance").and_then(|p| p.get("max_parallel_jobs")).and_then(|j| j.as_integer()) {
                    self.max_parallel_jobs = max_jobs as usize;
                }
                if let Some(lang_str) = parsed.get("app").and_then(|a| a.get("lang")).and_then(|l| l.as_str()) {
                    self.lang = match lang_str { "fr" => &crate::lang::FR, _ => &crate::lang::EN };
                    self.lang_id = match lang_str { "fr" => "fr", _ => "en" };
                }
                if let Some(doc) = parsed.get("doc") {
                    if let Some(fmt) = doc.get("format").and_then(|f| f.as_str()) {
                        if self.module_actif == ModuleType::Doc {
                            self.format_choisi = fmt.to_string();
                        }
                    }
                }
                if let Some(img) = parsed.get("image") {
                    if let Some(fmt) = img.get("format").and_then(|f| f.as_str()) {
                        if self.module_actif == ModuleType::Image {
                            self.format_choisi = fmt.to_string();
                        }
                    }
                    if let Some(ratio) = img.get("ratio_img").and_then(|r| r.as_integer()) {
                        self.ratio_img = ratio as u32;
                    }
                }
                #[cfg(feature = "api")]
                if let Some(arc) = parsed.get("archive") {
                    if let Some(fmt) = arc.get("format").and_then(|f| f.as_str()) {
                        if self.module_actif == ModuleType::Archive {
                            self.format_choisi = fmt.to_string();
                        }
                    }
                    if let Some(n) = arc.get("niveau").and_then(|v| v.as_integer()) {
                        self.archive_niveau = n as u32;
                    }
                }
                #[cfg(feature = "api")]
                if let Some(aud) = parsed.get("audio") {
                    if let Some(fmt) = aud.get("format").and_then(|f| f.as_str()) {
                        if self.module_actif == ModuleType::Audio {
                            self.format_choisi = fmt.to_string();
                        }
                    }
                    if let Some(q) = aud.get("qualite").and_then(|v| v.as_integer()) {
                        self.audio_qualite = q as u32;
                    }
                }
                #[cfg(feature = "api")]
                if let Some(vid) = parsed.get("video") {
                    if let Some(fmt) = vid.get("format").and_then(|f| f.as_str()) {
                        if self.module_actif == ModuleType::Video {
                            self.format_choisi = fmt.to_string();
                        }
                    }
                    if let Some(copie) = vid.get("copie_flux").and_then(|c| c.as_bool()) {
                        self.copie_flux = copie;
                    }
                    if let Some(speed) = vid.get("speed").and_then(|s| s.as_integer()) {
                        self.video_speed = speed as u32;
                    }
                }
            }
        }
        dotenvy::dotenv().ok();
        if let Ok(k) = std::env::var("TMDB_API_KEY") { self.tmdb_api_key = k; }
        if let Ok(k) = std::env::var("FANART_API_KEY") { self.fanart_api_key = k; }
    }
    fn save_config(&self) {
        let mut parsed = if let Ok(c) = std::fs::read_to_string("config.toml") {
            c.parse::<toml::Table>().unwrap_or_else(|_| toml::Table::new())
        } else {
            toml::Table::new()
        };
        let display = parsed.entry("display").or_insert(toml::Value::Table(toml::Table::new()));
        if let Some(display_table) = display.as_table_mut() {
            display_table.insert("theme".to_string(), toml::Value::String(self.current_theme.clone()));
        }
        let perf = parsed.entry("performance").or_insert(toml::Value::Table(toml::Table::new()));
        if let Some(perf_table) = perf.as_table_mut() {
            perf_table.insert("max_parallel_jobs".to_string(), toml::Value::Integer(self.max_parallel_jobs as i64));
        }
        let app = parsed.entry("app").or_insert(toml::Value::Table(toml::Table::new()));
        if let Some(app_table) = app.as_table_mut() {
            app_table.insert("lang".to_string(), toml::Value::String(self.lang_id.to_string()));
        }
        if self.save_doc_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Doc {
            let doc = parsed.entry("doc").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(doc_table) = doc.as_table_mut() {
                doc_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
            }
        } else if !self.save_doc_format && self.module_actif == ModuleType::Doc {
            if let Some(doc_table) = parsed.get_mut("doc").and_then(|v| v.as_table_mut()) {
                doc_table.remove("format");
            }
        }
        if self.save_image_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Image {
            let image = parsed.entry("image").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(img_table) = image.as_table_mut() {
                img_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
                img_table.insert("ratio_img".to_string(), toml::Value::Integer(self.ratio_img as i64));
            }
        } else if !self.save_image_format && self.module_actif == ModuleType::Image {
            if let Some(img_table) = parsed.get_mut("image").and_then(|v| v.as_table_mut()) {
                img_table.remove("format");
            }
        }
        if self.module_actif == ModuleType::Image {
            let image = parsed.entry("image").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(img_table) = image.as_table_mut() {
                img_table.insert("ratio_img".to_string(), toml::Value::Integer(self.ratio_img as i64));
            }
        }
        #[cfg(feature = "api")]
        {
            if self.save_archive_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Archive {
                let archive = parsed.entry("archive").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(arc_table) = archive.as_table_mut() {
                    arc_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
                }
            } else if !self.save_archive_format && self.module_actif == ModuleType::Archive {
                if let Some(arc_table) = parsed.get_mut("archive").and_then(|v| v.as_table_mut()) {
                    arc_table.remove("format");
                }
            }
            if self.module_actif == ModuleType::Archive {
                let archive = parsed.entry("archive").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(arc_table) = archive.as_table_mut() {
                    arc_table.insert("niveau".to_string(), toml::Value::Integer(self.archive_niveau as i64));
                }
            }
            if self.save_audio_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Audio {
                let audio = parsed.entry("audio").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(aud_table) = audio.as_table_mut() {
                    aud_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
                }
            } else if !self.save_audio_format && self.module_actif == ModuleType::Audio {
                if let Some(aud_table) = parsed.get_mut("audio").and_then(|v| v.as_table_mut()) {
                    aud_table.remove("format");
                }
            }
            if self.module_actif == ModuleType::Audio {
                let audio = parsed.entry("audio").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(aud_table) = audio.as_table_mut() {
                    aud_table.insert("qualite".to_string(), toml::Value::Integer(self.audio_qualite as i64));
                }
            }
            if self.save_video_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Video {
                let video = parsed.entry("video").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(vid_table) = video.as_table_mut() {
                    vid_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
                }
            } else if !self.save_video_format && self.module_actif == ModuleType::Video {
                if let Some(vid_table) = parsed.get_mut("video").and_then(|v| v.as_table_mut()) {
                    vid_table.remove("format");
                }
            }
            if self.module_actif == ModuleType::Video {
                let video = parsed.entry("video").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(vid_table) = video.as_table_mut() {
                    vid_table.insert("copie_flux".to_string(), toml::Value::Boolean(self.copie_flux));
                    vid_table.insert("speed".to_string(), toml::Value::Integer(self.video_speed as i64));
                }
            }
        }
        let _ = std::fs::write("config.toml", toml::to_string(&parsed).unwrap_or_default());
    }
    fn apply_theme(&self, ctx: &egui::Context) {
        match self.current_theme.as_str() {
            "Light" => ctx.set_visuals(egui::Visuals::light()),
            "Dark" => ctx.set_visuals(egui::Visuals::dark()),
            _ => ctx.set_visuals(egui::Visuals::default()),
        }
    }
    fn verifier_deps(&mut self) {
        self.deps_manquantes = Vec::new();
    }
    fn lancer_batch(&mut self, ctx: egui::Context) {
        log_info(&format!(
            "=== BATCH START | {} fichier(s) | {} workers max | module={:?} | action={} ===",
            self.current_files.len(),
            self.max_parallel_jobs,
            self.module_actif,
            match self.module_actif {
                ModuleType::Doc   => self.doc_action.as_str(),
                ModuleType::Image => self.image_action.as_str(),
                _                 => "-",
            }
        ));
        for f in &self.current_files {
            log_info(&format!("  Fichier en queue: {:?}", f));
        }

        *self.completed_jobs.lock().unwrap() = 0;
        *self.total_jobs.lock().unwrap() = self.current_files.len();
        *self.active_jobs.lock().unwrap() = 0;
        let mut queue = self.job_queue.lock().unwrap();
        queue.clear();
        queue.extend(self.current_files.clone());
        drop(queue);
        *self.status.lock().unwrap() = self.lang.starting_tasks.replace("{}", &self.current_files.len().to_string());
        for _ in 0..self.max_parallel_jobs.min(self.current_files.len()) {
            self.spawn_worker(ctx.clone());
        }
    }
    fn spawn_worker(&mut self, ctx: egui::Context) {
        let queue = Arc::clone(&self.job_queue);
        let active = Arc::clone(&self.active_jobs);
        let completed = Arc::clone(&self.completed_jobs);
        let total = Arc::clone(&self.total_jobs);
        let status_arc = Arc::clone(&self.status);
        let lang = self.lang;
        let module = self.module_actif;
        let fmt = self.format_choisi.clone();
        let ratio = self.ratio_img;
        #[cfg(feature = "api")]
        let copie = self.copie_flux;
        #[cfg(feature = "api")]
        let video_speed = self.video_speed;
        #[cfg(feature = "api")]
        let audio_action = self.audio_action.clone();
        #[cfg(feature = "api")]
        let audio_qualite = self.audio_qualite;
        #[cfg(feature = "api")]
        let archive_niveau = self.archive_niveau;
        let img_action = self.image_action.clone();
        let angle = self.rotation_angle;
        let crop_x = self.crop_x;
        let crop_y = self.crop_y;
        let crop_w = self.crop_width;
        let crop_h = self.crop_height;
        let resize_w = self.resize_width.parse::<u32>().unwrap_or(0);
        let resize_h = self.resize_height.parse::<u32>().unwrap_or(0);
        let resize_kb = self.resize_max_kb.parse::<u32>().unwrap_or(0);
        let doc_action = self.doc_action.clone();
        let pdf_angle = self.pdf_rotation_angle;
        let pdf_pages = self.pdf_pages_spec.clone();
        let pdf_merge_list = self.current_files.clone();
        let pdf_crop_x = self.pdf_crop_x;
        let pdf_crop_y = self.pdf_crop_y;
        let pdf_crop_w = self.pdf_crop_w;
        let pdf_crop_h = self.pdf_crop_h;
        let pdf_num_debut = self.pdf_num_debut;
        let pdf_num_position = self.pdf_num_position.clone();
        let pdf_num_taille = self.pdf_num_taille;
        let pdf_owner_pass = self.pdf_owner_pass.clone();
        let pdf_user_pass = self.pdf_user_pass.clone();
        let pdf_allow_print = self.pdf_allow_print;
        let pdf_allow_copy = self.pdf_allow_copy;
        let pdf_unlock_pass = self.pdf_unlock_pass.clone();
        let pdf_wm_texte = self.pdf_wm_texte.clone();
        let pdf_wm_taille = self.pdf_wm_taille;
        let pdf_wm_opacite = self.pdf_wm_opacite;
        let pdf_nouvel_ordre = self.pdf_nouvel_ordre.clone();
        std::thread::spawn(move || {
            loop {
                let job = {
                    let mut q = queue.lock().unwrap();
                    q.pop()
                };
                let input = match job {
                    Some(path) => path,
                    None => break,
                };
                *active.lock().unwrap() += 1;
                let effective_fmt = if module == ModuleType::Doc && doc_action != "Convert" {
                    "pdf".to_string()
                } else {
                    fmt.clone()
                };
                let output = input.parent().unwrap().join(format!(
                    "{}_oxyon.{}",
                    input.file_stem().unwrap_or_default().to_string_lossy(),
                    effective_fmt
                ));
                let out_str = output.to_str().unwrap().to_string();

                // ── Timing ──────────────────────────────────────────────
                let start = std::time::Instant::now();
                log_info(&format!(
                    "START | module={:?} | fichier={:?} | sortie={:?}",
                    module, input, out_str
                ));

                let current = *completed.lock().unwrap() + *active.lock().unwrap();
                let total_count = *total.lock().unwrap();
                *status_arc.lock().unwrap() = crate::lang::fmt2(lang.processing_files, &current.to_string(), &total_count.to_string());
                ctx.request_repaint();

                // ── Exécution avec résultat détaillé ────────────────────
                let result: Result<(), String> = match module {
                    #[cfg(feature = "api")]
                    ModuleType::Archive => {
                        log_info(&format!("Archive: compression fmt={} niveau={} | {:?}", fmt, archive_niveau, input));
                        if modules::archive::compresser(&input, &out_str, &fmt, archive_niveau) {
                            Ok(())
                        } else {
                            Err(format!("compresser() returned false | fmt={} | file={:?}", fmt, input))
                        }
                    },
                    #[cfg(feature = "api")]
                    ModuleType::Audio => {
                        match audio_action.as_str() {
                            "extract" => {
                                log_info(&format!("Audio: extraction | {:?}", input));
                                let ext = modules::audio::detecter_extension(&input);
                                let extract_out = input.parent().unwrap().join(format!(
                                    "{}_oxyon.{}",
                                    input.file_stem().unwrap_or_default().to_string_lossy(),
                                    if ext.is_empty() { "mka".to_string() } else { ext }
                                ));
                                let extract_str = extract_out.to_str().unwrap().to_string();
                                match modules::audio::extraire(&input, &extract_str) {
                                    Ok(mut child) => {
                                        match child.wait() {
                                            Ok(status) if status.success() => Ok(()),
                                            Ok(status) => Err(format!("audio extraction process exited with code={:?}", status.code())),
                                            Err(e) => Err(format!("erreur wait() extraction audio: {}", e)),
                                        }
                                    },
                                    Err(e) => Err(format!("impossible de lancer ffmpeg extraction: {}", e)),
                                }
                            },
                            _ => {
                                log_info(&format!("Audio: conversion | {:?}", input));
                                match modules::audio::convertir(&input, &out_str, audio_qualite) {
                                    Ok(mut child) => {
                                        match child.wait() {
                                            Ok(status) if status.success() => Ok(()),
                                            Ok(status) => Err(format!("audio process exited with code={:?}", status.code())),
                                            Err(e) => Err(format!("erreur wait() audio: {}", e)),
                                        }
                                    },
                                    Err(e) => Err(format!("impossible de lancer ffmpeg audio: {}", e)),
                                }
                            },
                        }
                    },
                    #[cfg(feature = "api")]
                    ModuleType::Video => {
                        log_info(&format!("Video: copie_flux={} speed={} | {:?}", copie, video_speed, input));
                        match modules::video::traiter_video(&input, &out_str, copie, false, video_speed) {
                            Ok(mut child) => {
                                match child.wait() {
                                    Ok(status) if status.success() => Ok(()),
                                    Ok(status) => Err(format!("video process exited with code={:?}", status.code())),
                                    Err(e) => Err(format!("wait() error video: {}", e)),
                                }
                            },
                            Err(e) => Err(format!("failed to start ffmpeg video: {}", e)),
                        }
                    },
                    ModuleType::Doc => {
                        log_info(&format!("Doc: action={} | {:?}", doc_action, input));
                        match doc_action.as_str() {
                            "Convert" => {
                                let format_entree = modules::doc::detecter_format_entree(&input);
                                let format_sortie = modules::doc::detecter_format_sortie(&out_str);
                                log_info(&format!("Doc Convert: entree={:?} sortie={:?}", format_entree, format_sortie));
                                if modules::doc::convertir_avec_formats(&input, &out_str, format_entree, format_sortie) {
                                    Ok(())
                                } else {
                                    Err(format!("Convert_avec_formats failed | input={:?} output={:?} | file={:?}", format_entree, format_sortie, input))
                                }
                            },
                            "pdf_split" => {
                                let output_dir = input.parent().unwrap().join(format!(
                                    "{}_pages",
                                    input.file_stem().unwrap_or_default().to_string_lossy()
                                ));
                                std::fs::create_dir_all(&output_dir).ok();
                                log_info(&format!("Doc pdf_split: output_dir={:?}", output_dir));
                                modules::doc::pdf_split(&input, output_dir.to_str().unwrap())
                                    .map(|_| ())
                                    .map_err(|e| format!("pdf_split failed: {}", e))
                            },
                            "pdf_merge" => {
                                let paths: Vec<&Path> = pdf_merge_list.iter().map(|p| p.as_path()).collect();
                                let output_merge = input.parent().unwrap().join("merged_oxyon.pdf");
                                log_info(&format!("Doc pdf_merge: {} fichiers -> {:?}", paths.len(), output_merge));
                                modules::doc::pdf_merge(&paths, output_merge.to_str().unwrap())
                                    .map_err(|e| format!("pdf_merge failed: {}", e))
                            },
                            "pdf_rotate" => {
                                let pages_opt = parse_pages_spec(&pdf_pages);
                                log_info(&format!("Doc pdf_rotate: angle={} pages={:?}", pdf_angle, pages_opt));
                                modules::doc::pdf_rotate(&input, &out_str, pdf_angle, pages_opt.as_deref())
                                    .map_err(|e| format!("pdf_rotate failed: {}", e))
                            },
                            "pdf_compress" => {
                                log_info(&format!("Doc pdf_compress: {:?}", input));
                                modules::doc::pdf_compresser(&input, &out_str)
                                    .map(|_| ())
                                    .map_err(|e| format!("pdf_compress failed: {}", e))
                            },
                            "pdf_crop" => {
                                let pages_opt = parse_pages_spec(&pdf_pages);
                                log_info(&format!("Doc pdf_crop: x={} y={} w={} h={} pages={:?}", pdf_crop_x, pdf_crop_y, pdf_crop_w, pdf_crop_h, pages_opt));
                                modules::doc::pdf_crop(&input, &out_str, pdf_crop_x, pdf_crop_y, pdf_crop_w, pdf_crop_h, pages_opt.as_deref())
                                    .map_err(|e| format!("pdf_crop failed: {}", e))
                            },
                            "pdf_organize" => {
                                let ordre: Vec<u32> = pdf_nouvel_ordre.split(',')
                                    .filter_map(|s| s.trim().parse::<u32>().ok())
                                    .collect();
                                log_info(&format!("Doc pdf_organize: ordre={:?}", ordre));
                                if ordre.is_empty() {
                                    Err("pdf_organize: ordre vide ou invalide".to_string())
                                } else {
                                    modules::doc::pdf_organiser(&input, &out_str, &ordre)
                                        .map_err(|e| format!("pdf_organize failed: {}", e))
                                }
                            },
                            "pdf_delete_pages" => {
                                let pages_a_sup: Vec<u32> = pdf_pages.split(',')
                                    .filter_map(|s| s.trim().parse::<u32>().ok())
                                    .collect();
                                log_info(&format!("Doc pdf_delete_pages: pages={:?}", pages_a_sup));
                                if pages_a_sup.is_empty() {
                                    Err("pdf_delete_pages: liste de pages vide ou invalide".to_string())
                                } else {
                                    modules::doc::pdf_supprimer_pages(&input, &out_str, &pages_a_sup)
                                        .map_err(|e| format!("pdf_delete_pages failed: {}", e))
                                }
                            },
                            "pdf_numbers" => {
                                let position = match pdf_num_position.as_str() {
                                    "BasGauche"  => modules::doc::PositionNumero::BasGauche,
                                    "BasDroite"  => modules::doc::PositionNumero::BasDroite,
                                    "HautCentre" => modules::doc::PositionNumero::HautCentre,
                                    "HautGauche" => modules::doc::PositionNumero::HautGauche,
                                    "HautDroite" => modules::doc::PositionNumero::HautDroite,
                                    _            => modules::doc::PositionNumero::BasCentre,
                                };
                                log_info(&format!("Doc pdf_numbers: debut={} position={} taille={}", pdf_num_debut, pdf_num_position, pdf_num_taille));
                                modules::doc::pdf_numeroter(&input, &out_str, pdf_num_debut, position, pdf_num_taille)
                                    .map_err(|e| format!("pdf_number_pages failed: {}", e))
                            },
                            "pdf_protect" => {
                                log_info(&format!("Doc pdf_protect: print={} copy={}", pdf_allow_print, pdf_allow_copy));
                                modules::doc::pdf_proteger(&input, &out_str, &pdf_owner_pass, &pdf_user_pass, pdf_allow_print, pdf_allow_copy)
                                    .map_err(|e| format!("pdf_protect failed: {}", e))
                            },
                            "pdf_unlock" => {
                                log_info("Doc pdf_unlock");
                                modules::doc::pdf_dechiffrer(&input, &out_str, &pdf_unlock_pass)
                                    .map_err(|e| format!("pdf_unlock failed: {}", e))
                            },
                            "pdf_repair" => {
                                log_info(&format!("Doc pdf_repair: {:?}", input));
                                modules::doc::pdf_reparer(&input, &out_str)
                                    .map_err(|e| format!("pdf_repair failed: {}", e))
                            },
                            "pdf_watermark" => {
                                let pages_opt = parse_pages_spec(&pdf_pages);
                                log_info(&format!("Doc pdf_watermark: texte='{}' taille={} opacite={}", pdf_wm_texte, pdf_wm_taille, pdf_wm_opacite));
                                modules::doc::pdf_watermark(&input, &out_str, &pdf_wm_texte, pdf_wm_taille, pdf_wm_opacite, pages_opt.as_deref())
                                    .map_err(|e| format!("pdf_watermark failed: {}", e))
                            },
                            autre => {
                                log_warn(&format!("Doc: action inconnue '{}', fallback Convert()", autre));
                                if modules::doc::convertir(&input, &out_str) {
                                    Ok(())
                                } else {
                                    Err(format!("Convert() fallback failed for {:?}", input))
                                }
                            },
                        }
                    },
                    ModuleType::Image => {
                        log_info(&format!("Image: action={} fmt={} ratio={} | {:?}", img_action, fmt, ratio, input));
                        match img_action.as_str() {
                            "Convert" => {
                                if modules::pic::compresser(&input, &out_str, ratio) { Ok(()) }
                                else { Err(format!("pic::compresser failed | fmt={} ratio={} | {:?}", fmt, ratio, input)) }
                            },
                            "resize" => {
                                log_info(&format!("Image resize: w={} h={} kb={}", resize_w, resize_h, resize_kb));
                                if resize_w > 0 && resize_h > 0 {
                                    if resize_kb > 0 {
                                        let temp = format!("{}_temp.{}", out_str, fmt);
                                        if modules::pic::redimensionner_pixels(&input, &temp, resize_w, resize_h) {
                                            if modules::pic::redimensionner_poids(Path::new(&temp), &out_str, resize_kb) {
                                                Ok(())
                                            } else {
                                                Err(format!("resize by size failed | max_kb={} | file={:?}", resize_kb, input))
                                            }
                                        } else {
                                            Err(format!("resize by pixels failed | w={} h={} | file={:?}", resize_w, resize_h, input))
                                        }
                                    } else {
                                        if modules::pic::redimensionner_pixels(&input, &out_str, resize_w, resize_h) { Ok(()) }
                                        else { Err(format!("resize by pixels failed | w={} h={} | file={:?}", resize_w, resize_h, input)) }
                                    }
                                } else if resize_kb > 0 {
                                    if modules::pic::redimensionner_poids(&input, &out_str, resize_kb) { Ok(()) }
                                    else { Err(format!("resize by size only failed | max_kb={} | file={:?}", resize_kb, input)) }
                                } else {
                                    log_warn("Image resize: no w/h or kb specified, fallback to compress");
                                    if modules::pic::compresser(&input, &out_str, 1) { Ok(()) }
                                    else { Err(format!("pic::compresser fallback failed for {:?}", input)) }
                                }
                            },
                            "rotate" => {
                                log_info(&format!("Image rotate: angle={}", angle));
                                if modules::pic::pivoter(&input, &out_str, angle) { Ok(()) }
                                else { Err(format!("pic::rotate failed | angle={} | file={:?}", angle, input)) }
                            },
                            "crop" => {
                                log_info(&format!("Image crop: x={} y={} w={} h={}", crop_x, crop_y, crop_w, crop_h));
                                if modules::pic::recadrer(&input, &out_str, crop_x, crop_y, crop_w, crop_h) { Ok(()) }
                                else { Err(format!("pic::crop failed | x={} y={} w={} h={} | file={:?}", crop_x, crop_y, crop_w, crop_h, input)) }
                            },
                            autre => {
                                log_warn(&format!("Image: action inconnue '{}', fallback compresser", autre));
                                if modules::pic::compresser(&input, &out_str, ratio) { Ok(()) }
                                else { Err(format!("pic::compresser fallback failed for {:?}", input)) }
                            },
                        }
                    },
                    _ => Ok(()),
                };

                // ── Résultat + timing ────────────────────────────────────
                let elapsed = start.elapsed();
                match &result {
                    Ok(()) => {
                        if elapsed.as_secs() > 30 {
                            log_warn(&format!(
                                "OK mais LENT ({:.1}s) | module={:?} | {:?}",
                                elapsed.as_secs_f32(), module, input
                            ));
                        } else {
                            log_info(&format!(
                                "OK ({:.2}s) | module={:?} | {:?}",
                                elapsed.as_secs_f32(), module, input
                            ));
                        }
                    },
                    Err(raison) => {
                        log_error(&format!(
                            "FAILED ({:.2}s) | module={:?} | file={:?} | reason={}",
                            elapsed.as_secs_f32(), module, input, raison
                        ));
                    }
                }

                *active.lock().unwrap() -= 1;
                *completed.lock().unwrap() += 1;
                let done = *completed.lock().unwrap();
                let total_count = *total.lock().unwrap();
                if done >= total_count {
                    log_info(&format!("=== BATCH END | {}/{} files processed ===", done, total_count));
                    *status_arc.lock().unwrap() = crate::lang::fmt2(lang.done_files, &done.to_string(), &total_count.to_string());
                } else {
                    *status_arc.lock().unwrap() = crate::lang::fmt2(lang.processing_files, &done.to_string(), &total_count.to_string());
                }
                ctx.request_repaint();
            }
        });
    }
}
impl eframe::App for OxyonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.cumulative_pass_nr() == 0 {
            self.load_config();
            self.apply_theme(ctx);
            self.verifier_deps();
        }
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.current_files = i.raw.dropped_files.iter().filter_map(|f| f.path.clone()).collect();
                if let Some(p) = self.current_files.first() {
                    self.current_stem = p.file_stem().unwrap_or_default().to_string_lossy().to_string();
                }
                #[cfg(feature = "api")]
                self.results_ui.lock().unwrap().clear();
                *self.status.lock().unwrap() = self.lang.files_loaded.replace("{}", &self.current_files.len().to_string());
            }
        });
        if let Some(ref mut c) = self.process {
            if let Ok(Some(_)) = c.try_wait() {
                *self.status.lock().unwrap() = self.lang.done.into();
                self.process = None;
            }
            ctx.request_repaint();
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading(format!("OXYON v{}", VERSION)));
            if !self.deps_manquantes.is_empty() {
                ui.colored_label(egui::Color32::RED, self.lang.missing.replace("{}", &self.deps_manquantes.join(", ")));
            }
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                let mut mods = vec![];
                #[cfg(feature = "api")] mods.push((ModuleType::Archive, "📦 Archive"));
                #[cfg(feature = "api")] mods.push((ModuleType::Audio, "🎵 Audio"));
                mods.push((ModuleType::Doc, "📄 Doc"));
                mods.push((ModuleType::Image, self.lang.tab_image));
                mods.push((ModuleType::Rename, self.lang.tab_rename));
                #[cfg(feature = "api")] mods.push((ModuleType::Scrapper, "🔍 Scrapper"));
                #[cfg(feature = "api")] mods.push((ModuleType::Tag, "🏷️ Tag"));
                #[cfg(feature = "api")] mods.push((ModuleType::Video, self.lang.tab_video));
                mods.push((ModuleType::Settings, self.lang.tab_settings));
                for (m, txt) in mods {
                    if ui.selectable_value(&mut self.module_actif, m, txt).clicked() {
                        self.load_config();
                    }
                }
            });
            ui.separator();
            match self.module_actif {
                #[cfg(feature = "api")]
                ModuleType::Archive => {
                    ui.horizontal(|ui| {
                        ui.label(self.lang.format_label);
                        egui::ComboBox::from_id_salt("arfmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                            for f in ["7z", "tar", "zip"] {
                                ui.selectable_value(&mut self.format_choisi, f.into(), f);
                            }
                        });
                    });
                    if ui.add(egui::Slider::new(&mut self.archive_niveau, 1..=9).text(self.lang.compression_slider)).changed() {
                        self.save_config();
                    }
                    if ui.checkbox(&mut self.save_archive_format, self.lang.save_format).changed() {
                        self.save_config();
                    }
                },
                ModuleType::Doc => {
                    ui.horizontal(|ui| {
                        ui.label(self.lang.action_label);
                        egui::ComboBox::from_id_salt("doc_action").selected_text(&self.doc_action).show_ui(ui, |ui| {
							ui.selectable_value(&mut self.doc_action, "Convert".into(), self.lang.doc_convert);
                            ui.selectable_value(&mut self.doc_action, "pdf_compress".into(), self.lang.doc_pdf_compress);
							ui.selectable_value(&mut self.doc_action, "pdf_crop".into(), self.lang.doc_pdf_crop);
                            ui.selectable_value(&mut self.doc_action, "pdf_delete_pages".into(), self.lang.doc_pdf_delete_pages);
							ui.selectable_value(&mut self.doc_action, "pdf_merge".into(), self.lang.doc_pdf_merge);
                            ui.selectable_value(&mut self.doc_action, "pdf_numbers".into(), self.lang.doc_pdf_number_pages);
                            ui.selectable_value(&mut self.doc_action, "pdf_organize".into(), self.lang.doc_pdf_organize);
							ui.selectable_value(&mut self.doc_action, "pdf_protect".into(), self.lang.doc_pdf_protect);
                            ui.selectable_value(&mut self.doc_action, "pdf_repair".into(), self.lang.doc_pdf_repair);
							ui.selectable_value(&mut self.doc_action, "pdf_rotate".into(), self.lang.doc_pdf_rotate);
							ui.selectable_value(&mut self.doc_action, "pdf_split".into(), self.lang.doc_pdf_split);
							ui.selectable_value(&mut self.doc_action, "pdf_unlock".into(), self.lang.doc_pdf_unlock);
                            ui.selectable_value(&mut self.doc_action, "pdf_watermark".into(), self.lang.doc_pdf_watermark);
                        });
                    });
                    ui.separator();
                    match self.doc_action.as_str() {
                        "Convert" => {
                            ui.horizontal(|ui| {
                                ui.label(self.lang.format_label);
                                egui::ComboBox::from_id_salt("dfmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["docx","epub","html","md","odt","pdf","rtf","txt"] {
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                            if ui.checkbox(&mut self.save_doc_format, self.lang.save_format).changed() {
                                self.save_config();
                            }
                        },
                        "pdf_split" => {
                            ui.label(self.lang.doc_split_hint1);
                            ui.label(self.lang.doc_split_hint2);
                        },
                        "pdf_merge" => {
                            ui.label(self.lang.doc_merge_hint1);
                            ui.label(self.lang.doc_merge_hint2);
                        },
                        "pdf_rotate" => {
                            ui.horizontal(|ui| {
                                ui.label(self.lang.angle_label);
                                egui::ComboBox::from_id_salt("pdf_rot").selected_text(format!("{}°", self.pdf_rotation_angle)).show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.pdf_rotation_angle, 90, "90°");
                                    ui.selectable_value(&mut self.pdf_rotation_angle, 180, "180°");
                                    ui.selectable_value(&mut self.pdf_rotation_angle, 270, "270°");
                                });
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.pages_hint);
                                ui.text_edit_singleline(&mut self.pdf_pages_spec);
                            });
                        },
                        "pdf_compress" => {
                            ui.label(self.lang.doc_compress_hint);
                        },
                        "pdf_crop" => {
                            ui.label(self.lang.doc_margins);
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.add(egui::Slider::new(&mut self.pdf_crop_x, 0.0..=100.0).fixed_decimals(1));
                                ui.label("Y:");
                                ui.add(egui::Slider::new(&mut self.pdf_crop_y, 0.0..=100.0).fixed_decimals(1));
                            });
                            ui.horizontal(|ui| {
                                ui.label("W:");
                                ui.add(egui::Slider::new(&mut self.pdf_crop_w, 1.0..=100.0).fixed_decimals(1));
                                ui.label("H:");
                                ui.add(egui::Slider::new(&mut self.pdf_crop_h, 1.0..=100.0).fixed_decimals(1));
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.pages_label);
                                ui.text_edit_singleline(&mut self.pdf_pages_spec);
                                ui.label(self.lang.pages_hint);
                            });
                        },
                        "pdf_organize" => {
                            ui.label(self.lang.doc_new_order);
                            ui.text_edit_singleline(&mut self.pdf_nouvel_ordre);
                        },
                        "pdf_delete_pages" => {
                            ui.label(self.lang.doc_delete_pages);
                            ui.text_edit_singleline(&mut self.pdf_pages_spec);
                        },
                        "pdf_numbers" => {
                            ui.horizontal(|ui| {
                                ui.label(self.lang.doc_start);
                                ui.add(egui::Slider::new(&mut self.pdf_num_debut, 1..=999));
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.doc_position);
                                egui::ComboBox::from_id_salt("pdf_numpos").selected_text(&self.pdf_num_position).show_ui(ui, |ui| {
                                    for pos in ["BasCentre","BasGauche","BasDroite","HautCentre","HautGauche","HautDroite"] {
                                        ui.selectable_value(&mut self.pdf_num_position, pos.into(), pos);
                                    }
                                });
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.doc_size);
                                ui.add(egui::Slider::new(&mut self.pdf_num_taille, 6.0..=36.0).fixed_decimals(0));
                            });
                        },
                        "pdf_protect" => {
                            ui.horizontal(|ui| {
                                ui.label(self.lang.doc_owner_password);
                                ui.add(egui::TextEdit::singleline(&mut self.pdf_owner_pass).password(true));
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.doc_user_password);
                                ui.add(egui::TextEdit::singleline(&mut self.pdf_user_pass).password(true));
                            });
                            ui.checkbox(&mut self.pdf_allow_print, self.lang.doc_allow_print);
                            ui.checkbox(&mut self.pdf_allow_copy, self.lang.doc_allow_copy);
                        },
                        "pdf_unlock" => {
                            ui.horizontal(|ui| {
                                ui.label(self.lang.doc_password);
                                ui.add(egui::TextEdit::singleline(&mut self.pdf_unlock_pass).password(true));
                            });
                        },
                        "pdf_repair" => {
                            ui.label(self.lang.doc_repair_hint1);
                            ui.label(self.lang.doc_repair_hint2);
                            ui.label(self.lang.doc_repair_hint3);
                        },
                        "pdf_watermark" => {
                            ui.label(self.lang.doc_watermark_hint1);
                            ui.label(self.lang.doc_repair_hint3);
                            ui.horizontal(|ui| {
                                ui.label(self.lang.doc_text);
                                ui.text_edit_singleline(&mut self.pdf_wm_texte);
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.doc_size);
                                ui.add(egui::Slider::new(&mut self.pdf_wm_taille, 12.0..=120.0).fixed_decimals(0));
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.doc_opacity);
                                ui.add(egui::Slider::new(&mut self.pdf_wm_opacite, 0.05..=1.0).fixed_decimals(2));
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.pages_label);
                                ui.text_edit_singleline(&mut self.pdf_pages_spec);
                                ui.label(self.lang.pages_hint);
                            });
                        },
                        _ => {}
                    }
                },
                ModuleType::Image => {
                    ui.horizontal(|ui| {
                        ui.label(self.lang.action_label);
                        egui::ComboBox::from_id_salt("img_action").selected_text(&self.image_action).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.image_action, "Convert".into(), self.lang.doc_convert);
							ui.selectable_value(&mut self.image_action, "crop".into(), self.lang.img_crop);
                            ui.selectable_value(&mut self.image_action, "resize".into(), self.lang.img_resize);
                            ui.selectable_value(&mut self.image_action, "rotate".into(), self.lang.img_rotate);
                        });
                    });
                    ui.separator();
                    match self.image_action.as_str() {
                        "Convert" => {
                            ui.horizontal(|ui| {
                                ui.label(self.lang.format_label);
                                egui::ComboBox::from_id_salt("ifmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["EXR","GIF","ICO","JPG","JXL","PNG","PSD","SVG","TIFF","WebP"] {
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                            if ui.checkbox(&mut self.save_image_format, self.lang.save_format).changed() {
                                self.save_config();
                            }
                            if ui.add(egui::Slider::new(&mut self.ratio_img, 1..=10).text(self.lang.img_quality_slider)).changed() {
                                self.save_config();
                            }
                        },
                        "resize" => {
                            ui.horizontal(|ui| {
                                ui.label(self.lang.format_label);
                                egui::ComboBox::from_id_salt("ifmt_resize").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["EXR","GIF","ICO","JPG","JXL","PNG","PSD","SVG","TIFF","WebP"] {
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                            if ui.checkbox(&mut self.save_image_format, self.lang.save_format).changed() {
                                self.save_config();
                            }
                            ui.separator();
                            ui.label(self.lang.img_resize_px);
                            ui.horizontal(|ui| {
                                ui.label(self.lang.img_width);
                                ui.text_edit_singleline(&mut self.resize_width);
                                ui.label(self.lang.img_height);
                                ui.text_edit_singleline(&mut self.resize_height);
                            });
                            ui.label(self.lang.img_andor);
                            ui.horizontal(|ui| {
                                ui.label(self.lang.img_max_size);
                                ui.text_edit_singleline(&mut self.resize_max_kb);
                            });
                        },
                        "rotate" => {
                            ui.horizontal(|ui| {
                                ui.label(self.lang.angle_label);
                                egui::ComboBox::from_id_salt("rot_angle").selected_text(format!("{}°", self.rotation_angle)).show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.rotation_angle, 90, "90°");
                                    ui.selectable_value(&mut self.rotation_angle, 180, "180°");
                                    ui.selectable_value(&mut self.rotation_angle, 270, "270°");
                                });
                            });
                        },
                        "crop" => {
                            ui.label(self.lang.img_coordinates);
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.add(egui::Slider::new(&mut self.crop_x, 0..=100));
                                ui.label("Y:");
                                ui.add(egui::Slider::new(&mut self.crop_y, 0..=100));
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.img_width);
                                ui.add(egui::Slider::new(&mut self.crop_width, 1..=100));
                                ui.label(self.lang.img_height);
                                ui.add(egui::Slider::new(&mut self.crop_height, 1..=100));
                            });
                        },
                        _ => {}
                    }
                },
                #[cfg(feature = "api")]
                ModuleType::Audio => {
                    ui.horizontal(|ui| {
                        ui.label(self.lang.action_label);
                        egui::ComboBox::from_id_salt("audio_action").selected_text(&self.audio_action).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.audio_action, "extract".into(), self.lang.audio_extract);
							ui.selectable_value(&mut self.audio_action, "Convert".into(), self.lang.doc_convert);                            
                        });
                    });
                    ui.separator();
                    match self.audio_action.as_str() {
                        "Convert" => {
                            // Détection codec au chargement de fichiers
                            if let Some(f) = self.current_files.first() {
                                if ui.button(self.lang.audio_detect_codec).clicked() {
                                    let codec = modules::audio::detecter_extension(f);
                                    let fmts = modules::audio::formats_compatibles(&codec);
                                    self.audio_formats_dispo = fmts.iter().map(|s| s.to_string()).collect();
                                    crate::log_info(&format!("Audio: codec detected='{}' | formats compatibles={:?}", codec, self.audio_formats_dispo));
                                }
                            }
                            ui.horizontal(|ui| {
                                ui.label(self.lang.format_label);
                                egui::ComboBox::from_id_salt("afmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in &self.audio_formats_dispo {
                                        ui.selectable_value(&mut self.format_choisi, f.clone(), f.as_str());
                                    }
                                });
                            });
                            if ui.add(egui::Slider::new(&mut self.audio_qualite, 0..=9).text(self.lang.audio_vbr_slider)).changed() {
                                self.save_config();
                            }
                            if ui.checkbox(&mut self.save_audio_format, self.lang.save_format).changed() {
                                self.save_config();
                            }
                        },
                        "extract" => {
                            ui.label(self.lang.audio_extract_hint1);
                            ui.label(self.lang.audio_extract_hint2);
                        },
                        _ => {}
                    }
                },
                #[cfg(feature = "api")]
                ModuleType::Video => {
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_salt("vfmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                            for f in ["mkv","mp4","webm"] {
                                ui.selectable_value(&mut self.format_choisi, f.into(), f);
                            }
                        });
                        if ui.checkbox(&mut self.copie_flux, self.lang.video_stream_copy).changed() { self.save_config(); }
                    });
                    if ui.add(egui::Slider::new(&mut self.video_speed, 0..=8).text(self.lang.video_quality_slider)).changed() {
                        self.save_config();
                    }
                    if ui.checkbox(&mut self.save_video_format, self.lang.save_format).changed() {
                        self.save_config();
                    }
                },
                #[cfg(feature = "api")]
                ModuleType::Scrapper => {
                    ui.horizontal(|ui| {
                        ui.label(self.lang.scrap_tmdb_key);
                        ui.add(egui::TextEdit::singleline(&mut self.tmdb_api_key).password(true));
                    });
                    ui.horizontal(|ui| {
                        ui.label(self.lang.scrap_fanart_key);
                        ui.add(egui::TextEdit::singleline(&mut self.fanart_api_key).password(true));
                    });
                    if ui.button(self.lang.scrap_save_keys).clicked() {
                        let content = format!(
                            "TMDB_API_KEY={}\nFANART_API_KEY={}\n",
                            self.tmdb_api_key, self.fanart_api_key
                        );
                        let _ = std::fs::write(".env", content);
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        let search = |is_series: bool, res_arc: Arc<Mutex<Vec<ScrapeEntry>>>, stem: String, ctx_c: egui::Context| {
                            res_arc.lock().unwrap().clear();
                            std::thread::spawn(move || {
                                if let Ok(results) = modules::scrap::search_tmdb(&stem, is_series) {
                                    for r in results {
                                        let tex = r.poster_path.as_ref()
                                        .and_then(|p| modules::scrap::download_image_bytes(p))
                                        .and_then(|b| image::load_from_memory(&b).ok())
                                        .map(|img| {
                                            let ci = egui::ColorImage::from_rgba_unmultiplied(
                                                [img.width() as usize, img.height() as usize],
                                                img.to_rgba8().as_flat_samples().as_slice()
                                            );
                                            ctx_c.load_texture(format!("p_{}", r.id), ci, Default::default())
                                        });
                                        res_arc.lock().unwrap().push(ScrapeEntry { data: r, texture: tex });
                                        ctx_c.request_repaint();
                                    }
                                }
                            });
                        };
                        if ui.button(self.lang.scrap_movie).clicked() {
                            search(false, Arc::clone(&self.results_ui), self.current_stem.clone(), ctx.clone());
                        }
                        if ui.button(self.lang.scrap_series).clicked() {
                            search(true, Arc::clone(&self.results_ui), self.current_stem.clone(), ctx.clone());
                        }
                    });
                    let entries = self.results_ui.lock().unwrap().clone();
                    for entry in entries {
                        ui.horizontal(|ui| {
                            if let Some(t) = &entry.texture { ui.image((t.id(), egui::vec2(50.0, 75.0))); }
                            ui.label(&entry.data.title);
                            if !self.current_files.is_empty() {
                                if ui.button(self.lang.scrap_choose).clicked() { modules::scrap::save_metadata(self.current_files[0].clone(), entry.data.clone()); }
                            }
                        });
                    }
                },
                #[cfg(feature = "api")]
                ModuleType::Tag => {
                    let path_opt = self.current_files.get(0).cloned();
                    ui.vertical(|ui| {
                        if ui.button(self.lang.tag_mark_watched).clicked() { if let Some(path) = &path_opt { let _ = modules::tag::marquer_vu(&path, &path.with_extension("nfo"), self.lang_id); } }
                        if ui.button(self.lang.tag_inject_nfo).clicked() { if let Some(path) = &path_opt { let _ = modules::tag::appliquer_tags(&path, &path.with_extension("nfo")); } }
                        if ui.button(self.lang.tag_add_poster).clicked() { if let Some(path) = &path_opt { let _ = modules::tag::ajouter_images_mkv(&path); } }
                        if ui.button(self.lang.tag_reset_tags).clicked() { if let Some(path) = &path_opt { let _ = modules::tag::supprimer_tous_tags(&path); } }
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.tag_edit_val);
                            if ui.button(self.lang.tag_edit_title).clicked() { if let Some(path) = &path_opt { let _ = modules::tag::modifier_tag(&path, "title", &self.tag_edit_val); } }
                        });
                    });
                },
                ModuleType::Rename => {
                    ui.vertical(|ui| {
                        self.rename_previews = modules::rename::preview(&self.current_files, &self.rename_cfg);

                        ui.heading(self.lang.tab_rename);
                        ui.separator();

                        // ── Find & Replace ──────────────────────────────────
                        ui.collapsing(self.lang.rename_find_replace, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(self.lang.rename_find);
                                ui.text_edit_singleline(&mut self.rename_cfg.find);
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.rename_replace_with);
                                ui.text_edit_singleline(&mut self.rename_cfg.replace_with);
                            });
                        });

                        // ── Insertion ───────────────────────────────────────
                        ui.collapsing(self.lang.rename_insert, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(self.lang.rename_text);
                                ui.text_edit_singleline(&mut self.rename_cfg.insert_text);
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.rename_at_pos);
                                ui.add(egui::DragValue::new(&mut self.rename_cfg.insert_pos).range(0..=999));
                            });
                        });

                        // ── Suppression de plage ────────────────────────────
                        ui.collapsing(self.lang.rename_delete_range, |ui| {
                            ui.checkbox(&mut self.rename_cfg.delete_enabled, self.lang.rename_enable);
                            ui.horizontal(|ui| {
                                ui.label(self.lang.rename_from);
                                ui.add(egui::DragValue::new(&mut self.rename_cfg.delete_from).range(0..=999));
                                ui.label(self.lang.rename_count);
                                ui.add(egui::DragValue::new(&mut self.rename_cfg.delete_count).range(0..=999));
                            });
                        });

                        // ── Numérotation ────────────────────────────────────
                        ui.collapsing(self.lang.rename_numbering, |ui| {
                            ui.checkbox(&mut self.rename_cfg.num_enabled, self.lang.rename_enable);
                            if self.rename_cfg.num_enabled {
                                ui.horizontal(|ui| {
                                    ui.label(self.lang.rename_start);
                                    ui.add(egui::DragValue::new(&mut self.rename_cfg.num_start).range(0..=99999));
                                    ui.label(self.lang.rename_step);
                                    ui.add(egui::DragValue::new(&mut self.rename_cfg.num_step).range(1..=100));
                                    ui.label(self.lang.rename_padding);
                                    ui.add(egui::DragValue::new(&mut self.rename_cfg.num_padding).range(0..=10));
                                });
                                ui.horizontal(|ui| {
                                    ui.label(self.lang.rename_separator);
                                    ui.add(egui::TextEdit::singleline(&mut self.rename_cfg.num_sep).desired_width(40.0));
                                    ui.label(self.lang.rename_position);
                                    egui::ComboBox::from_id_salt("num_pos")
                                        .selected_text(match self.rename_cfg.num_pos {
                                            modules::rename::NumPos::Prefix => self.lang.rename_prefix,
                                            modules::rename::NumPos::Suffix => self.lang.rename_suffix,
                                        })
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut self.rename_cfg.num_pos, modules::rename::NumPos::Prefix, self.lang.rename_prefix);
                                            ui.selectable_value(&mut self.rename_cfg.num_pos, modules::rename::NumPos::Suffix, self.lang.rename_suffix);
                                        });
                                });
                            }
                        });

                        // ── Casse ───────────────────────────────────────────
                        ui.collapsing(self.lang.rename_case, |ui| {
                            ui.horizontal(|ui| {
                                for mode in [
                                    modules::rename::CaseMode::Unchanged,
                                    modules::rename::CaseMode::Lower,
                                    modules::rename::CaseMode::Upper,
                                    modules::rename::CaseMode::Title,
                                    modules::rename::CaseMode::Sentence,
                                ] as [modules::rename::CaseMode; 5] {
                                    let label = mode.label();
                                    ui.selectable_value(&mut self.rename_cfg.case_mode, mode, label);
                                }
                            });
                        });

                        // ── Nettoyage ───────────────────────────────────────
                        ui.collapsing(self.lang.rename_clean, |ui| {
                            ui.checkbox(&mut self.rename_cfg.strip_trailing_spaces, self.lang.rename_trim_spaces);
                            ui.checkbox(&mut self.rename_cfg.strip_double_spaces, self.lang.rename_double_spaces);
                            ui.checkbox(&mut self.rename_cfg.strip_leading_dots, self.lang.rename_leading_dots);
                            ui.horizontal(|ui| {
                                ui.label(self.lang.rename_strip_chars);
                                ui.text_edit_singleline(&mut self.rename_cfg.strip_chars);
                            });
                        });

                        // ── Extension ───────────────────────────────────────
                        ui.collapsing(self.lang.rename_extension, |ui| {
                            ui.horizontal(|ui| {
                                for mode in [
                                    modules::rename::ExtMode::Unchanged,
                                    modules::rename::ExtMode::Lower,
                                    modules::rename::ExtMode::Upper,
                                    modules::rename::ExtMode::Replace,
                                    modules::rename::ExtMode::Remove,
                                ] as [modules::rename::ExtMode; 5] {
                                    let label = mode.label();
                                    ui.selectable_value(&mut self.rename_cfg.ext_mode, mode, label);
                                }
                            });
                            if self.rename_cfg.ext_mode == modules::rename::ExtMode::Replace {
                                ui.horizontal(|ui| {
                                    ui.label(self.lang.rename_new_ext);
                                    ui.text_edit_singleline(&mut self.rename_cfg.ext_new);
                                });
                            }
                        });

                        ui.separator();
                        if ui.button(self.lang.rename_reset).clicked() {
                            self.rename_cfg = modules::rename::RenameConfig::default();
                        }
                        ui.separator();

                        // ── Preview ─────────────────────────────────────────
                        if self.current_files.is_empty() {
                            ui.label(self.lang.drop_here);
                        } else {
                            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                                egui::Grid::new("rename_preview").striped(true).min_col_width(200.0).show(ui, |ui| {
                                    ui.strong(self.lang.rename_original);
                                    ui.strong(self.lang.rename_new_name);
                                    ui.end_row();
                                    for (orig, new_name) in &self.rename_previews {
                                        let orig_str = orig.file_name().and_then(|n| n.to_str()).unwrap_or("");
                                        if orig_str != new_name {
                                            ui.colored_label(egui::Color32::LIGHT_GREEN, orig_str);
                                            ui.colored_label(egui::Color32::LIGHT_GREEN, new_name);
                                        } else {
                                            ui.label(orig_str);
                                            ui.label(new_name);
                                        }
                                        ui.end_row();
                                    }
                                });
                            });
                            ui.separator();
                            if !self.rename_results.is_empty() {
                                let ok  = self.rename_results.iter().filter(|r| r.success).count();
                                let err = self.rename_results.iter().filter(|r| !r.success).count();
                                if err > 0 {
                                    ui.colored_label(egui::Color32::RED, format!("✅ {}  ⚠️ {} errors", ok, err));
                                    for r in self.rename_results.iter().filter(|r| !r.success) {
                                        if let Some(e) = &r.error {
                                            ui.colored_label(egui::Color32::RED, format!("  • {}: {}", r.new_name, e));
                                        }
                                    }
                                } else {
                                    ui.colored_label(egui::Color32::LIGHT_GREEN, format!("✅ {} {}", ok, self.lang.rename_done));
                                }
                            }
                            if ui.button(self.lang.rename_apply).clicked() {
                                self.rename_results = modules::rename::apply_renames(&self.rename_previews);
                                for r in &self.rename_results {
                                    if r.success {
                                        if let Some(pos) = self.current_files.iter().position(|f| *f == r.original) {
                                            let parent = r.original.parent().unwrap_or(std::path::Path::new(""));
                                            self.current_files[pos] = parent.join(&r.new_name);
                                        }
                                    }
                                }
                            }
                        }
                    });
                },
                ModuleType::Settings => {
                    ui.vertical(|ui| {
                        ui.heading(self.lang.settings_heading);
                        let old_theme = self.current_theme.clone();
                        ui.horizontal(|ui| {
                            ui.label(self.lang.settings_theme);
                            egui::ComboBox::from_id_salt("theme_sel").selected_text(&self.current_theme).show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.current_theme, "Auto".into(), "Auto");
                                ui.selectable_value(&mut self.current_theme, "Light".into(), "Light");
                                ui.selectable_value(&mut self.current_theme, "Dark".into(), "Dark");
                            });
                        });
                        if self.current_theme != old_theme {
                            self.apply_theme(ctx);
                            self.save_config();
                        }
                        ui.horizontal(|ui| {
                            ui.label("Language:");
                            let lang_label = if self.lang_id == "fr" { "Français" } else { "English" };
                            egui::ComboBox::from_id_salt("lang_sel").selected_text(lang_label).show_ui(ui, |ui| {
                                if ui.selectable_label(self.lang_id == "en", "English").clicked() {
                                    self.lang = &crate::lang::EN; self.lang_id = "en"; self.save_config();
                                }
                                if ui.selectable_label(self.lang_id == "fr", "Français").clicked() {
                                    self.lang = &crate::lang::FR; self.lang_id = "fr"; self.save_config();
                                }
                            });
                        });
                        ui.separator();
                        ui.heading(self.lang.settings_performance);
                        ui.horizontal(|ui| {
                            ui.label(self.lang.settings_max_jobs);
                            if ui.add(egui::Slider::new(&mut self.max_parallel_jobs, 1..=16).text("threads")).changed() {
                                self.save_config();
                            }
                        });
                        ui.label(self.lang.settings_jobs_hint);
                    });
                },
            }
            let mut hide_exec = self.module_actif == ModuleType::Settings || self.module_actif == ModuleType::Rename;
            #[cfg(feature = "api")]
            { hide_exec = hide_exec || self.module_actif == ModuleType::Scrapper || self.module_actif == ModuleType::Tag; }
            if !self.current_files.is_empty() && !hide_exec {
                ui.separator();
                if ui.button(self.lang.run_all).clicked() {
                    self.lancer_batch(ctx.clone());
                }
            }
            if self.current_files.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(self.lang.drop_here);
                        ui.add_space(5.0);
                        if ui.button(self.lang.browse).clicked() {
                            if let Some(paths) = rfd::FileDialog::new().pick_files() {
                                self.current_files = paths;
                                if let Some(p) = self.current_files.first() {
                                    self.current_stem = p.file_stem().unwrap_or_default().to_string_lossy().to_string();
                                }
                                #[cfg(feature = "api")]
                                self.results_ui.lock().unwrap().clear();
                                *self.status.lock().unwrap() = self.lang.files_loaded.replace("{}", &self.current_files.len().to_string());
                            }
                        }
                    });
                });
            }
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                let completed = *self.completed_jobs.lock().unwrap();
                let total = *self.total_jobs.lock().unwrap();
                if total > 0 && completed < total {
                    let active = *self.active_jobs.lock().unwrap();
                    let pct = (completed as f32 / total as f32 * 100.0).round() as u32;
                    ui.heading(crate::lang::fmt3(self.lang.processing_pct, &completed.to_string(), &total.to_string(), &pct.to_string()));
                    ui.add(egui::ProgressBar::new(completed as f32 / total as f32).animate(true));
                    ui.small(crate::lang::fmt2(self.lang.active_pending, &active.to_string(), &self.job_queue.lock().unwrap().len().to_string()));
                } else if total > 0 && completed >= total {
                    ui.heading(self.lang.done_processed.replace("{}", &total.to_string()));
                } else {
                    ui.heading(&*self.status.lock().unwrap());
                }
            });
            if !self.current_files.is_empty() { if ui.button(self.lang.clear_all).clicked() { self.current_files.clear(); } }
        });
    }
}
fn parse_pages_spec(spec: &str) -> Option<Vec<u32>> {
    let trimmed = spec.trim();
    if trimmed.is_empty() || trimmed == "1-end" {
        return None;
    }
    let pages: Vec<u32> = trimmed.split(',')
    .filter_map(|s| s.trim().parse::<u32>().ok())
    .collect();
    if pages.is_empty() { None } else { Some(pages) }
}
fn main() -> eframe::Result {
    log_info(&format!("=== OXYON v{} START ===", VERSION));
    let _ = modules::binaries::extraire_deps();
    let mut options = eframe::NativeOptions::default();
    #[cfg(target_os = "windows")]
    let icon_bytes: &[u8] = include_bytes!("../assets/Oxyon_icon.ico");
    #[cfg(not(target_os = "windows"))]
    let icon_bytes: &[u8] = include_bytes!("../assets/Oxyon_icon.png");
    if let Ok(icon_data) = image::load_from_memory(icon_bytes) {
        let icon_rgba = icon_data.to_rgba8();
        let (width, height) = icon_rgba.dimensions();
        options.viewport.icon = Some(std::sync::Arc::new(egui::IconData { rgba: icon_rgba.into_raw(), width, height }));
    }
    let result = eframe::run_native(
        &format!("oxyon v{}", VERSION),
        options,
        Box::new(|cc| {
            let mut app = OxyonApp::default();
            app.load_config();
            app.apply_theme(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    );
    log_info("=== OXYON FERMETURE ===");
    modules::binaries::cleanup();
    result
}