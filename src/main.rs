#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
const VERSION: &str = env!("CARGO_PKG_VERSION");
pub mod lang;
pub mod logging;
pub use logging::{log_info, log_warn, log_error};
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


#[cfg(feature = "api")]
#[derive(Clone)]
struct ScrapeEntry {
    data: modules::scrap::ScrapeResult,
    texture: Option<egui::TextureHandle>,
}
#[derive(PartialEq, Clone, Copy, Debug)]
enum ModuleType {
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
    Tools,
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
        rename_multi_find: String,
        rename_multi_replace: String,
        rename_ant_sets: Vec<String>,
        rename_ant_path: Option<PathBuf>,
        tmdb_api_key: String,
        fanart_api_key: String,
        save_doc_format: bool,
        save_image_format: bool,
        save_archive_format: bool,
        archive_niveau: u32,
        archive_action: String,
        archive_backup_source: String,
        archive_backup_dest: String,
        archive_backup_exclusions: String,
        archive_multi_source: String,
        #[cfg(feature = "api")]
        save_audio_format: bool,
        #[cfg(feature = "api")]
        audio_qualite: u32,
        #[cfg(feature = "api")]
        save_video_format: bool,
        #[cfg(feature = "api")]
        video_speed: u32,
        image_action: String,
        jxl_mode: String,
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
        tools_cfg: modules::tools::ToolsConfig,
        tools_new_name: String,
        tools_new_path: String,
        tools_new_folder: String,
        tools_result: String,
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
                save_archive_format: false,
                archive_niveau: 6,
                archive_action: "compress".into(),
                archive_backup_source: String::new(),
                archive_backup_dest: String::new(),
                archive_backup_exclusions: ".git, .github, target".into(),
                archive_multi_source: String::new(),
                #[cfg(feature = "api")]
                save_audio_format: false,
                #[cfg(feature = "api")]
                audio_qualite: 2,
                #[cfg(feature = "api")]
                save_video_format: false,
                #[cfg(feature = "api")]
                video_speed: 4,
                image_action: "Convert".into(),
                jxl_mode: "lossless".into(),
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
                tools_cfg: modules::tools::ToolsConfig::default(),
                tools_new_name: String::new(),
                tools_new_path: String::new(),
                tools_new_folder: String::new(),
                tools_result: String::new(),
                rename_cfg: modules::rename::RenameConfig::default(),
                rename_previews: Vec::new(),
                rename_results: Vec::new(),
                rename_multi_find: String::new(),
                rename_multi_replace: String::new(),
                rename_ant_sets: Vec::new(),
                rename_ant_path: None,
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
            ModuleType::Tools => {},
            #[cfg(feature = "api")]
            ModuleType::Video => self.format_choisi = String::new(),
            #[cfg(feature = "api")]
            ModuleType::Audio => self.format_choisi = String::new(),
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
                self.tools_cfg = modules::tools::ToolsConfig::load(&parsed);
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
                if let Some(arc) = parsed.get("archive") {
                    if let Some(fmt) = arc.get("format").and_then(|f| f.as_str()) {
                        if self.module_actif == ModuleType::Archive {
                            self.format_choisi = fmt.to_string();
                        }
                    }
                    if let Some(n) = arc.get("niveau").and_then(|v| v.as_integer()) {
                        self.archive_niveau = n as u32;
                    }
                    if let Some(s) = arc.get("backup_source").and_then(|v| v.as_str()) {
                        self.archive_backup_source = s.to_string();
                    }
                    if let Some(s) = arc.get("backup_dest").and_then(|v| v.as_str()) {
                        self.archive_backup_dest = s.to_string();
                    }
                    if let Some(s) = arc.get("backup_exclusions").and_then(|v| v.as_str()) {
                        self.archive_backup_exclusions = s.to_string();
                    }
                    if let Some(s) = arc.get("multi_source").and_then(|v| v.as_str()) {
                        self.archive_multi_source = s.to_string();
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
                    arc_table.insert("backup_source".to_string(), toml::Value::String(self.archive_backup_source.clone()));
                    arc_table.insert("backup_dest".to_string(), toml::Value::String(self.archive_backup_dest.clone()));
                    arc_table.insert("backup_exclusions".to_string(), toml::Value::String(self.archive_backup_exclusions.clone()));
                    arc_table.insert("multi_source".to_string(), toml::Value::String(self.archive_multi_source.clone()));
                }
            }
        #[cfg(feature = "api")]
        {
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
        self.tools_cfg.save(&mut parsed);
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
        let archive_niveau = self.archive_niveau;
        let archive_action = self.archive_action.clone();
        let img_action = self.image_action.clone();
        let jxl_mode = self.jxl_mode.clone();
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
                    ModuleType::Archive => {
                        match archive_action.as_str() {
                            "extract" => {
                                let dest = input.parent().unwrap().join(
                                    input.file_stem().unwrap_or_default().to_string_lossy().to_string()
                                );
                                let dest_str = dest.to_string_lossy().to_string();
                                log_info(&format!("Archive: extraction | {:?} -> {}", input, dest_str));
                                if modules::archive::extraire(&input, &dest_str) {
                                    Ok(())
                                } else {
                                    Err(format!("extraire() failed | file={:?}", input))
                                }
                            },
                            "convert" => {
                                log_info(&format!("Archive: convert fmt={} | {:?}", fmt, input));
                                if modules::archive::convertir(&input, &fmt) {
                                    Ok(())
                                } else {
                                    Err(format!("convertir() failed | fmt={} | file={:?}", fmt, input))
                                }
                            },
                            "multi" => {
                                // input = un sous-dossier, output = dossier.{fmt} à côté
                                let name = input.file_name().unwrap_or_default().to_string_lossy().to_string();
                                let multi_out = input.parent().unwrap().join(format!("{}.{}", name, fmt));
                                let multi_out_str = multi_out.to_string_lossy().to_string();
                                log_info(&format!("Archive multi: {} -> {}", name, multi_out_str));
                                if modules::archive::compresser(&input, &multi_out_str, &fmt, archive_niveau) {
                                    Ok(())
                                } else {
                                    Err(format!("multi compresser() failed | {} | fmt={}", name, fmt))
                                }
                            },
                            _ => {
                                log_info(&format!("Archive: compression fmt={} niveau={} | {:?}", fmt, archive_niveau, input));
                                if modules::archive::compresser(&input, &out_str, &fmt, archive_niveau) {
                                    Ok(())
                                } else {
                                    Err(format!("compresser() returned false | fmt={} | file={:?}", fmt, input))
                                }
                            },
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
                                // Si format JXL, dispatcher selon jxl_mode
                                if fmt.to_uppercase() == "JXL" {
                                    match jxl_mode.as_str() {
                                        "folder" => modules::pic::convertir_jxl_dossier(&input),
                                        "pivot" => modules::pic::convertir_jxl_pivot(&input),
                                        _ => modules::pic::convertir_jxl_lossless(&input),
                                    }
                                } else {
                                    if modules::pic::compresser(&input, &out_str, ratio) { Ok(()) }
                                    else { Err(format!("pic::compresser failed | fmt={} ratio={} | {:?}", fmt, ratio, input)) }
                                }
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
                mods.push((ModuleType::Archive, "📦 Archive"));
                #[cfg(feature = "api")] mods.push((ModuleType::Audio, "🎵 Audio"));
                mods.push((ModuleType::Doc, "📄 Doc"));
                mods.push((ModuleType::Image, self.lang.tab_image));
                mods.push((ModuleType::Rename, self.lang.tab_rename));
                #[cfg(feature = "api")] mods.push((ModuleType::Scrapper, "🔍 Scrapper"));
                #[cfg(feature = "api")] mods.push((ModuleType::Tag, "🏷️ Tag"));
                mods.push((ModuleType::Tools, "🛠 Tools"));
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
                ModuleType::Archive => {
                    ui.horizontal(|ui| {
                        ui.label(self.lang.action_label);
                        egui::ComboBox::from_id_salt("archive_action").selected_text(
                            match self.archive_action.as_str() {
                                "compress" => "Compress",
                                "extract" => "Extract",
                                "convert" => "Convert",
                                "backup" => "Backup",
                                "multi" => "Multi-compress",
                                _ => "Compress",
                            }
                        ).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.archive_action, "compress".into(), "Compress");
                            ui.selectable_value(&mut self.archive_action, "extract".into(), "Extract");
                            ui.selectable_value(&mut self.archive_action, "convert".into(), "Convert");
                            ui.selectable_value(&mut self.archive_action, "backup".into(), "Backup");
                            ui.selectable_value(&mut self.archive_action, "multi".into(), "Multi-compress");
                        });
                    });
                    ui.separator();
                    match self.archive_action.as_str() {
                        "compress" => {
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
                        "extract" => {
                            ui.label("Drop archive files above, they will be extracted next to the original.");
                        },
                        "convert" => {
                            ui.label("Drop archive files above to convert to another format.");
                            ui.horizontal(|ui| {
                                ui.label(self.lang.format_label);
                                egui::ComboBox::from_id_salt("arfmt_conv").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["7z", "tar", "zip"] {
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                        },
                        "backup" => {
                            ui.horizontal(|ui| {
                                ui.label("Source:");
                                ui.add(egui::TextEdit::singleline(&mut self.archive_backup_source).desired_width(250.0));
                                if ui.button("📂").clicked() {
                                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                        self.archive_backup_source = path.to_string_lossy().to_string();
                                        self.save_config();
                                    }
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label("Destination:");
                                ui.add(egui::TextEdit::singleline(&mut self.archive_backup_dest).desired_width(250.0));
                                if ui.button("📂").clicked() {
                                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                        self.archive_backup_dest = path.to_string_lossy().to_string();
                                        self.save_config();
                                    }
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label("Exclusions:");
                                if ui.add(egui::TextEdit::singleline(&mut self.archive_backup_exclusions).desired_width(250.0)).lost_focus() {
                                    self.save_config();
                                }
                            });
                            ui.small("Comma-separated folder names to exclude (e.g. .git, target, node_modules)");
                            ui.add_space(5.0);
                            let can_backup = !self.archive_backup_source.is_empty() && !self.archive_backup_dest.is_empty();
                            if ui.add_enabled(can_backup, egui::Button::new("▶ Run Backup")).clicked() {
                                let exclusions: Vec<&str> = self.archive_backup_exclusions
                                    .split(',')
                                    .map(|s| s.trim())
                                    .filter(|s| !s.is_empty())
                                    .collect();
                                let source = std::path::Path::new(&self.archive_backup_source);
                                match modules::archive::backup_zip(source, &self.archive_backup_dest, &exclusions) {
                                    Ok(path) => {
                                        *self.status.lock().unwrap() = format!("✅ Backup: {}", path);
                                    }
                                    Err(e) => {
                                        *self.status.lock().unwrap() = format!("⚠️ Backup: {}", e);
                                    }
                                }
                            }
                        },
                        "multi" => {
                            ui.horizontal(|ui| {
                                ui.label("Parent folder:");
                                ui.add(egui::TextEdit::singleline(&mut self.archive_multi_source).desired_width(250.0));
                                if ui.button("📂").clicked() {
                                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                        self.archive_multi_source = path.to_string_lossy().to_string();
                                        self.save_config();
                                    }
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label(self.lang.format_label);
                                egui::ComboBox::from_id_salt("arfmt_multi").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["7z", "tar", "zip"] {
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                            if ui.add(egui::Slider::new(&mut self.archive_niveau, 1..=9).text(self.lang.compression_slider)).changed() {
                                self.save_config();
                            }
                            ui.small("Each direct subfolder will be compressed as an individual archive.");
                            ui.add_space(5.0);
                            let can_multi = !self.archive_multi_source.is_empty();
                            if ui.add_enabled(can_multi, egui::Button::new("▶ Compress All Subfolders")).clicked() {
                                let parent = std::path::Path::new(&self.archive_multi_source);
                                match std::fs::read_dir(parent) {
                                    Ok(entries) => {
                                        self.current_files = entries
                                            .filter_map(|e| e.ok())
                                            .map(|e| e.path())
                                            .filter(|p| p.is_dir())
                                            .collect();
                                        if self.current_files.is_empty() {
                                            *self.status.lock().unwrap() = "No subfolders found.".into();
                                        } else {
                                            crate::log_info(&format!(
                                                "Archive multi: {} subfolders found in {:?}",
                                                self.current_files.len(), parent
                                            ));
                                            self.lancer_batch(ctx.clone());
                                        }
                                    }
                                    Err(e) => {
                                        *self.status.lock().unwrap() = format!("⚠️ {}", e);
                                    }
                                }
                            }
                        },
                        _ => {}
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
                            // Sous-options JXL
                            if self.format_choisi.to_uppercase() == "JXL" {
                                ui.separator();
                                ui.horizontal(|ui| {
                                    ui.label("JXL mode:");
                                    egui::ComboBox::from_id_salt("jxl_mode").selected_text(
                                        match self.jxl_mode.as_str() {
                                            "folder" => "Folder (separate dir)",
                                            "pivot" => "Pivot (via PNG)",
                                            _ => "Lossless (in-place)",
                                        }
                                    ).show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.jxl_mode, "lossless".into(), "Lossless (in-place)");
                                        ui.selectable_value(&mut self.jxl_mode, "folder".into(), "Folder (separate dir)");
                                        ui.selectable_value(&mut self.jxl_mode, "pivot".into(), "Pivot (via PNG)");
                                    });
                                });
                                match self.jxl_mode.as_str() {
                                    "folder" => { ui.small("Output in a \"{folder} jxl\" directory next to the source folder."); },
                                    "pivot" => { ui.small("Re-decode via PNG pivot for problematic files, output: {name}_pivot.jxl."); },
                                    _ => { ui.small("Lossless JXL next to the original, skips if .jxl already exists."); },
                                }
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
                                ui.selectable_value(&mut self.rename_cfg.multi_replace, false, "Simple"); // TODO lang
                                ui.selectable_value(&mut self.rename_cfg.multi_replace, true, "Multiple"); // TODO lang
                            });
                            if !self.rename_cfg.multi_replace {
                                // ── Mode simple ──
                                ui.horizontal(|ui| {
                                    ui.label(self.lang.rename_find);
                                    ui.text_edit_singleline(&mut self.rename_cfg.find);
                                });
                                ui.horizontal(|ui| {
                                    ui.label(self.lang.rename_replace_with);
                                    ui.text_edit_singleline(&mut self.rename_cfg.replace_with);
                                });
                            } else {
                                // ── Mode multiple ──
                                ui.horizontal(|ui| {
                                    ui.label(self.lang.rename_find);
                                    ui.add(egui::TextEdit::singleline(&mut self.rename_multi_find).desired_width(150.0));
                                    ui.label(self.lang.rename_replace_with);
                                    ui.add(egui::TextEdit::singleline(&mut self.rename_multi_replace).desired_width(150.0));
                                    if ui.button("➕").on_hover_text("Add rule").clicked() && !self.rename_multi_find.is_empty() { // TODO lang
                                        self.rename_cfg.replace_list.add(
                                            self.rename_multi_find.clone(),
                                            self.rename_multi_replace.clone(),
                                        );
                                        self.rename_multi_find.clear();
                                        self.rename_multi_replace.clear();
                                    }
                                });
                                // Import / Export (toujours visible en haut)
                                ui.horizontal(|ui| {
                                    if ui.button("💾 Save").clicked() { // TODO lang
                                        if let Some(path) = rfd::FileDialog::new()
                                            .add_filter("TSV", &["tsv"])
                                            .set_file_name("replace_rules.tsv")
                                            .save_file()
                                        {
                                            if let Err(e) = self.rename_cfg.replace_list.save(&path) {
                                                log_error(&format!("Save replace list: {}", e));
                                            }
                                        }
                                    }
                                    if ui.button("📂 Load").clicked() { // TODO lang
                                        if let Some(path) = rfd::FileDialog::new()
                                            .add_filter("TSV", &["tsv"])
                                            .pick_file()
                                        {
                                            match modules::rename::ReplaceList::load(&path) {
                                                Ok(list) => self.rename_cfg.replace_list = list,
                                                Err(e) => log_error(&format!("Load replace list: {}", e)),
                                            }
                                        }
                                    }
                                    if ui.button("📂 Ant Renamer").clicked() { // TODO lang
                                        if let Some(path) = rfd::FileDialog::new()
                                            .add_filter("XML", &["xml"])
                                            .pick_file()
                                        {
                                            match modules::rename::ReplaceList::list_ant_renamer_sets(&path) {
                                                Ok(sets) if sets.is_empty() => {
                                                    // Pas de sets nommés, charger CurrentList
                                                    match modules::rename::ReplaceList::load_ant_renamer_xml(&path, None) {
                                                        Ok(list) => self.rename_cfg.replace_list = list,
                                                        Err(e) => log_error(&format!("Import Ant Renamer: {}", e)),
                                                    }
                                                }
                                                Ok(sets) if sets.len() == 1 => {
                                                    // Un seul set, charger directement
                                                    match modules::rename::ReplaceList::load_ant_renamer_xml(&path, Some(&sets[0])) {
                                                        Ok(list) => self.rename_cfg.replace_list = list,
                                                        Err(e) => log_error(&format!("Import Ant Renamer: {}", e)),
                                                    }
                                                }
                                                Ok(sets) => {
                                                    // Plusieurs sets → afficher le sélecteur
                                                    self.rename_ant_sets = sets;
                                                    self.rename_ant_path = Some(path);
                                                }
                                                Err(e) => log_error(&format!("Import Ant Renamer: {}", e)),
                                            }
                                        }
                                    }
                                    if ui.button("🗑").on_hover_text("Clear all rules").clicked() { // TODO lang
                                        self.rename_cfg.replace_list.rules.clear();
                                    }
                                });
                                // Sélecteur de Set Ant Renamer (affiché si plusieurs sets disponibles)
                                if !self.rename_ant_sets.is_empty() {
                                    ui.horizontal(|ui| {
                                        ui.label("Set :"); // TODO lang
                                        for set_name in self.rename_ant_sets.clone() {
                                            if ui.button(&set_name).clicked() {
                                                if let Some(ref path) = self.rename_ant_path {
                                                    match modules::rename::ReplaceList::load_ant_renamer_xml(path, Some(&set_name)) {
                                                        Ok(list) => self.rename_cfg.replace_list = list,
                                                        Err(e) => log_error(&format!("Import Ant Renamer set '{}': {}", set_name, e)),
                                                    }
                                                }
                                                self.rename_ant_sets.clear();
                                                self.rename_ant_path = None;
                                            }
                                        }
                                        // Aussi proposer CurrentList
                                        if ui.button("CurrentList").clicked() {
                                            if let Some(ref path) = self.rename_ant_path {
                                                match modules::rename::ReplaceList::load_ant_renamer_xml(path, None) {
                                                    Ok(list) => self.rename_cfg.replace_list = list,
                                                    Err(e) => log_error(&format!("Import Ant Renamer CurrentList: {}", e)),
                                                }
                                            }
                                            self.rename_ant_sets.clear();
                                            self.rename_ant_path = None;
                                        }
                                        if ui.button("✖").clicked() {
                                            self.rename_ant_sets.clear();
                                            self.rename_ant_path = None;
                                        }
                                    });
                                }
                                // Tableau des règles (scrollable)
                                let mut to_remove: Option<usize> = None;
                                let mut to_move_up: Option<usize> = None;
                                let mut to_move_down: Option<usize> = None;
                                if !self.rename_cfg.replace_list.rules.is_empty() {
                                    egui::ScrollArea::vertical().max_height(200.0).id_salt("multi_replace_scroll").show(ui, |ui| {
                                        egui::Grid::new("multi_replace_grid").striped(true).show(ui, |ui| {
                                            ui.label(""); // checkbox col
                                            ui.strong(self.lang.rename_find);
                                            ui.strong(self.lang.rename_replace_with);
                                            ui.label(""); // actions
                                            ui.end_row();
                                            for (i, rule) in self.rename_cfg.replace_list.rules.iter_mut().enumerate() {
                                                ui.checkbox(&mut rule.enabled, "");
                                                ui.add(egui::TextEdit::singleline(&mut rule.find).desired_width(140.0));
                                                ui.add(egui::TextEdit::singleline(&mut rule.replace).desired_width(140.0));
                                                ui.horizontal(|ui| {
                                                    if ui.small_button("▲").clicked() { to_move_up = Some(i); }
                                                    if ui.small_button("▼").clicked() { to_move_down = Some(i); }
                                                    if ui.small_button("🗑").clicked() { to_remove = Some(i); }
                                                });
                                                ui.end_row();
                                            }
                                        });
                                    });
                                }
                                if let Some(i) = to_move_up { self.rename_cfg.replace_list.move_up(i); }
                                if let Some(i) = to_move_down { self.rename_cfg.replace_list.move_down(i); }
                                if let Some(i) = to_remove { self.rename_cfg.replace_list.remove(i); }
                            }
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
                                ui.selectable_value(&mut self.rename_cfg.insert_from_end, false, "↦ From start"); // TODO lang
                                ui.selectable_value(&mut self.rename_cfg.insert_from_end, true, "↤ From end"); // TODO lang
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
                                ui.selectable_value(&mut self.rename_cfg.delete_from_end, false, "↦ From start"); // TODO lang
                                ui.selectable_value(&mut self.rename_cfg.delete_from_end, true, "↤ From end"); // TODO lang
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
                ModuleType::Tools => {
                    ui.vertical(|ui| {
                        ui.heading("🛠 Tools");
                        ui.separator();

                        // ── Dossier de sortie ──
                        ui.horizontal(|ui| {
                            ui.label("Output folder:");
                            ui.add(egui::TextEdit::singleline(&mut self.tools_cfg.list_dir).desired_width(300.0));
                            if ui.button("📂").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    self.tools_cfg.list_dir = path.to_string_lossy().to_string();
                                    self.save_config();
                                }
                            }
                        });

                        ui.separator();

                        // ── File Sources ──
                        ui.collapsing("📁 File Sources (files listing)", |ui| {
                            // Tableau existant
                            let mut to_remove: Option<String> = None;
                            if !self.tools_cfg.file_sources.is_empty() {
                                egui::ScrollArea::vertical().max_height(150.0).id_salt("tools_files_scroll").show(ui, |ui| {
                                    egui::Grid::new("tools_files_grid").striped(true).show(ui, |ui| {
                                        ui.strong("Name");
                                        ui.strong("Path");
                                        ui.label("");
                                        ui.end_row();
                                        for (name, path) in &self.tools_cfg.file_sources {
                                            ui.label(name);
                                            ui.label(path);
                                            if ui.small_button("🗑").clicked() {
                                                to_remove = Some(name.clone());
                                            }
                                            ui.end_row();
                                        }
                                    });
                                });
                            }
                            if let Some(key) = to_remove {
                                self.tools_cfg.file_sources.remove(&key);
                                self.save_config();
                            }
                            // Ajouter
                            ui.horizontal(|ui| {
                                ui.label("Name:");
                                ui.add(egui::TextEdit::singleline(&mut self.tools_new_name).desired_width(100.0));
                                ui.label("Path:");
                                ui.add(egui::TextEdit::singleline(&mut self.tools_new_path).desired_width(200.0));
                                if ui.button("📂").clicked() {
                                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                        self.tools_new_path = path.to_string_lossy().to_string();
                                    }
                                }
                                if ui.button("➕").clicked() && !self.tools_new_name.is_empty() && !self.tools_new_path.is_empty() {
                                    self.tools_cfg.file_sources.insert(
                                        self.tools_new_name.clone(),
                                        self.tools_new_path.clone(),
                                    );
                                    self.tools_new_name.clear();
                                    self.tools_new_path.clear();
                                    self.save_config();
                                }
                            });
                        });

                        // ── Folder Sources ──
                        ui.collapsing("📂 Folder Sources (directory listing)", |ui| {
                            let mut to_remove_idx: Option<usize> = None;
                            if !self.tools_cfg.folder_sources.is_empty() {
                                egui::ScrollArea::vertical().max_height(120.0).id_salt("tools_folders_scroll").show(ui, |ui| {
                                    for (i, src) in self.tools_cfg.folder_sources.iter().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.label(src);
                                            if ui.small_button("🗑").clicked() {
                                                to_remove_idx = Some(i);
                                            }
                                        });
                                    }
                                });
                            }
                            if let Some(i) = to_remove_idx {
                                self.tools_cfg.folder_sources.remove(i);
                                self.save_config();
                            }
                            ui.horizontal(|ui| {
                                ui.label("Path:");
                                ui.add(egui::TextEdit::singleline(&mut self.tools_new_folder).desired_width(250.0));
                                if ui.button("📂").clicked() {
                                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                        self.tools_new_folder = path.to_string_lossy().to_string();
                                    }
                                }
                                if ui.button("➕").clicked() && !self.tools_new_folder.is_empty() {
                                    self.tools_cfg.folder_sources.push(self.tools_new_folder.clone());
                                    self.tools_new_folder.clear();
                                    self.save_config();
                                }
                            });
                        });

                        ui.separator();

                        // ── Boutons d'exécution ──
                        let can_run = !self.tools_cfg.list_dir.is_empty() && !self.tools_cfg.is_empty();
                        ui.horizontal(|ui| {
                            ui.set_enabled(can_run);
                            if ui.button("📋 List Files").clicked() {
                                let (ok, errs) = modules::tools::lister_fichiers(&self.tools_cfg);
                                if errs.is_empty() {
                                    self.tools_result = format!("✅ {} sources processed → {}", ok, self.tools_cfg.list_dir);
                                } else {
                                    self.tools_result = format!("✅ {} ok | ⚠️ {}\n{}", ok, errs.len(), errs.join("\n"));
                                }
                                crate::log_info(&format!("Tools lister_fichiers: ok={} err={}", ok, errs.len()));
                            }
                            if ui.button("📂 List Folders").clicked() {
                                let (ok, errs) = modules::tools::lister_dossiers(&self.tools_cfg);
                                if errs.is_empty() {
                                    self.tools_result = format!("✅ {} sources → multimedia.txt in {}", ok, self.tools_cfg.list_dir);
                                } else {
                                    self.tools_result = format!("✅ {} ok | ⚠️ {}\n{}", ok, errs.len(), errs.join("\n"));
                                }
                                crate::log_info(&format!("Tools lister_dossiers: ok={} err={}", ok, errs.len()));
                            }
                        });
                        if !can_run {
                            ui.colored_label(egui::Color32::YELLOW, "Configure output folder and at least one source above.");
                        }

                        // ── Résultat ──
                        if !self.tools_result.is_empty() {
                            ui.separator();
                            ui.label(&self.tools_result);
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
            let mut hide_exec = self.module_actif == ModuleType::Settings || self.module_actif == ModuleType::Rename || self.module_actif == ModuleType::Tools;
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
        &format!("Oxyon v{}", VERSION),
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
