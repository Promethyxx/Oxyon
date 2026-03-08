const VERSION: &str = env!("CARGO_PKG_VERSION");
pub mod lang;
pub mod logging;
pub use logging::{log_info, log_warn, log_error};
pub mod modules;
mod cli;

use clap::Parser;
use cli::*;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Cli::parse();
    let lang_id = args.lang.as_str();

    eprintln!("Oxyon CLI v{}", VERSION);

    // Extraire les binaires bundled (ffmpeg, mkvpropedit, ffprobe)
    let _ = modules::binaries::extraire_deps();

    let result = match args.command {
        Command::Pic { action } => run_pic(action),
        Command::Doc { action } => run_doc(action),
        #[cfg(feature = "api")]
        Command::Tag { action } => run_tag(action, lang_id),
        Command::Rename {
            files, find, replace, regex, list, ant, ant_set,
            case, number, num_start, num_step, num_pad, num_sep,
            ext, ext_new, dry_run,
        } => run_rename(
            files, find, replace, regex, list, ant, ant_set,
            case, number, num_start, num_step, num_pad, num_sep,
            ext, ext_new, dry_run,
        ),
        Command::Archive { action } => run_archive(action),
        Command::Tools { action } => run_tools(action),
    };

    modules::binaries::cleanup();

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ERREUR: {}", e);
            ExitCode::FAILURE
        }
    }
}

// ─── Helpers ────────────────────────────────────────────────────

fn make_output(input: &Path, ext: &str) -> String {
    let parent = input.parent().unwrap_or(Path::new("."));
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    parent.join(format!("{}_oxyon.{}", stem, ext)).to_string_lossy().to_string()
}

fn parse_pages(spec: &Option<String>) -> Option<Vec<u32>> {
    spec.as_ref().and_then(|s| {
        let pages: Vec<u32> = s.split(',').filter_map(|p| p.trim().parse().ok()).collect();
        if pages.is_empty() { None } else { Some(pages) }
    })
}

fn ok_or_err(success: bool, context: &str) -> Result<(), String> {
    if success { Ok(()) } else { Err(format!("{} failed", context)) }
}

// ─── PIC ────────────────────────────────────────────────────────

fn run_pic(action: PicAction) -> Result<(), String> {
    match action {
        PicAction::Convert { files, to, quality, jxl_mode } => {
            for f in &files {
                eprintln!("  pic convert: {:?} → {}", f, to);
                if to.eq_ignore_ascii_case("jxl") {
                    match jxl_mode.as_str() {
                        "folder" => modules::pic::convertir_jxl_dossier(f)?,
                        "pivot" => modules::pic::convertir_jxl_pivot(f)?,
                        _ => modules::pic::convertir_jxl_lossless(f)?,
                    }
                } else {
                    let out = make_output(f, &to);
                    ok_or_err(modules::pic::compresser(f, &out, quality), "pic::compresser")?;
                    eprintln!("    → {}", out);
                }
            }
            Ok(())
        }
        PicAction::Resize { files, width, height, max_kb } => {
            for f in &files {
                let ext = f.extension().unwrap_or_default().to_string_lossy().to_string();
                let out = make_output(f, &ext);
                eprintln!("  pic resize: {:?}", f);
                let w = width.unwrap_or(0);
                let h = height.unwrap_or(0);
                let kb = max_kb.unwrap_or(0);
                if w > 0 && h > 0 && kb > 0 {
                    let temp = format!("{}_temp.{}", out, ext);
                    ok_or_err(modules::pic::redimensionner_pixels(f, &temp, w, h), "resize pixels")?;
                    ok_or_err(modules::pic::redimensionner_poids(Path::new(&temp), &out, kb), "resize weight")?;
                    let _ = std::fs::remove_file(&temp);
                } else if w > 0 && h > 0 {
                    ok_or_err(modules::pic::redimensionner_pixels(f, &out, w, h), "resize pixels")?;
                } else if kb > 0 {
                    ok_or_err(modules::pic::redimensionner_poids(f, &out, kb), "resize weight")?;
                } else {
                    return Err("Specify --width/--height and/or --max-kb".into());
                }
                eprintln!("    → {}", out);
            }
            Ok(())
        }
        PicAction::Rotate { files, angle } => {
            for f in &files {
                let ext = f.extension().unwrap_or_default().to_string_lossy().to_string();
                let out = make_output(f, &ext);
                ok_or_err(modules::pic::pivoter(f, &out, angle), "pic::pivoter")?;
                eprintln!("  rotated {:?} → {}", f, out);
            }
            Ok(())
        }
        PicAction::Crop { files, x, y, width, height } => {
            for f in &files {
                let ext = f.extension().unwrap_or_default().to_string_lossy().to_string();
                let out = make_output(f, &ext);
                ok_or_err(modules::pic::recadrer(f, &out, x, y, width, height), "pic::recadrer")?;
                eprintln!("  cropped {:?} → {}", f, out);
            }
            Ok(())
        }
        PicAction::Exif { files } => {
            for f in &files {
                let tags = modules::pic::lire_exif(f);
                if tags.is_empty() {
                    println!("{}: no EXIF data", f.display());
                } else {
                    println!("{}:", f.display());
                    for t in &tags { println!("  {}", t); }
                }
            }
            Ok(())
        }
        PicAction::StripExif { files } => {
            for f in &files {
                let ext = f.extension().unwrap_or_default().to_string_lossy().to_string();
                let out = make_output(f, &ext);
                ok_or_err(modules::pic::supprimer_exif(f, &out), "pic::supprimer_exif")?;
                eprintln!("  stripped EXIF {:?} → {}", f, out);
            }
            Ok(())
        }
    }
}

// ─── DOC ────────────────────────────────────────────────────────

fn run_doc(action: DocAction) -> Result<(), String> {
    match action {
        DocAction::Convert { files, to } => {
            for f in &files {
                let out = make_output(f, &to);
                let fmt_in = modules::doc::detecter_format_entree(f);
                let fmt_out = modules::doc::detecter_format_sortie(&out);
                ok_or_err(modules::doc::convertir_avec_formats(f, &out, fmt_in, fmt_out), "doc::convertir")?;
                eprintln!("  converted {:?} → {}", f, out);
            }
            Ok(())
        }
        DocAction::PdfSplit { file } => {
            let output_dir = file.parent().unwrap_or(Path::new(".")).join(
                format!("{}_pages", file.file_stem().unwrap_or_default().to_string_lossy())
            );
            std::fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;
            let pages = modules::doc::pdf_split(&file, output_dir.to_str().unwrap())?;
            eprintln!("  split {:?} → {} pages in {:?}", file, pages.len(), output_dir);
            Ok(())
        }
        DocAction::PdfMerge { files, output } => {
            let paths: Vec<&Path> = files.iter().map(|p| p.as_path()).collect();
            modules::doc::pdf_merge(&paths, &output)?;
            eprintln!("  merged {} files → {}", files.len(), output);
            Ok(())
        }
        DocAction::PdfRotate { file, angle, pages } => {
            let out = make_output(&file, "pdf");
            let p = parse_pages(&pages);
            modules::doc::pdf_rotate(&file, &out, angle, p.as_deref())?;
            eprintln!("  rotated {:?} → {}", file, out);
            Ok(())
        }
        DocAction::PdfCompress { file } => {
            let out = make_output(&file, "pdf");
            let saved = modules::doc::pdf_compresser(&file, &out)?;
            eprintln!("  compressed {:?} → {} (saved {} bytes)", file, out, saved);
            Ok(())
        }
        DocAction::PdfCrop { file, x, y, width, height, pages } => {
            let out = make_output(&file, "pdf");
            let p = parse_pages(&pages);
            modules::doc::pdf_crop(&file, &out, x, y, width, height, p.as_deref())?;
            eprintln!("  cropped {:?} → {}", file, out);
            Ok(())
        }
        DocAction::PdfOrganize { file, order } => {
            let out = make_output(&file, "pdf");
            let ordre: Vec<u32> = order.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            if ordre.is_empty() { return Err("Invalid page order".into()); }
            modules::doc::pdf_organiser(&file, &out, &ordre)?;
            eprintln!("  reordered {:?} → {}", file, out);
            Ok(())
        }
        DocAction::PdfDelete { file, pages } => {
            let out = make_output(&file, "pdf");
            let p: Vec<u32> = pages.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            if p.is_empty() { return Err("No valid pages specified".into()); }
            modules::doc::pdf_supprimer_pages(&file, &out, &p)?;
            eprintln!("  deleted pages {:?} from {:?} → {}", p, file, out);
            Ok(())
        }
        DocAction::PdfNumber { file, start, position, size } => {
            let out = make_output(&file, "pdf");
            let pos = match position.as_str() {
                "BasGauche"  => modules::doc::PositionNumero::BasGauche,
                "BasDroite"  => modules::doc::PositionNumero::BasDroite,
                "HautCentre" => modules::doc::PositionNumero::HautCentre,
                "HautGauche" => modules::doc::PositionNumero::HautGauche,
                "HautDroite" => modules::doc::PositionNumero::HautDroite,
                _            => modules::doc::PositionNumero::BasCentre,
            };
            modules::doc::pdf_numeroter(&file, &out, start, pos, size)?;
            eprintln!("  numbered {:?} → {}", file, out);
            Ok(())
        }
        DocAction::PdfProtect { file, owner_pass, user_pass, allow_print, allow_copy } => {
            let out = make_output(&file, "pdf");
            modules::doc::pdf_proteger(&file, &out, &owner_pass, &user_pass, allow_print, allow_copy)?;
            eprintln!("  protected {:?} → {}", file, out);
            Ok(())
        }
        DocAction::PdfUnlock { file, password } => {
            let out = make_output(&file, "pdf");
            modules::doc::pdf_dechiffrer(&file, &out, &password)?;
            eprintln!("  unlocked {:?} → {}", file, out);
            Ok(())
        }
        DocAction::PdfRepair { file } => {
            let out = make_output(&file, "pdf");
            modules::doc::pdf_reparer(&file, &out)?;
            eprintln!("  repaired {:?} → {}", file, out);
            Ok(())
        }
        DocAction::PdfWatermark { file, text, size, opacity, pages } => {
            let out = make_output(&file, "pdf");
            let p = parse_pages(&pages);
            modules::doc::pdf_watermark(&file, &out, &text, size, opacity, p.as_deref())?;
            eprintln!("  watermarked {:?} → {}", file, out);
            Ok(())
        }
    }
}

// ─── TAG ────────────────────────────────────────────────────────

#[cfg(feature = "api")]
fn run_tag(action: TagAction, lang_id: &str) -> Result<(), String> {
    match action {
        TagAction::MarquerVu { files } => {
            for f in &files {
                // marquer_vu attend un nfo_path mais ne l'utilise pas (paramètre legacy)
                modules::tag::marquer_vu(f, f, lang_id)?;
                eprintln!("  marked watched: {:?}", f);
            }
            Ok(())
        }
        TagAction::Edit { file, tag, value } => {
            modules::tag::modifier_tag(&file, &tag, &value)?;
            eprintln!("  set {}={} on {:?}", tag, value, file);
            Ok(())
        }
        TagAction::Nfo { file, nfo } => {
            modules::tag::appliquer_tags(&file, &nfo)?;
            eprintln!("  injected NFO {:?} → {:?}", nfo, file);
            Ok(())
        }
        TagAction::Images { files } => {
            for f in &files {
                modules::tag::ajouter_images_mkv(f)?;
                eprintln!("  attached images to {:?}", f);
            }
            Ok(())
        }
        TagAction::Reset { files } => {
            for f in &files {
                modules::tag::supprimer_tous_tags(f)?;
                eprintln!("  reset all tags: {:?}", f);
            }
            Ok(())
        }
    }
}

// ─── RENAME ─────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn run_rename(
    files: Vec<std::path::PathBuf>,
    find: Option<String>,
    replace: String,
    use_regex: bool,
    list: Option<std::path::PathBuf>,
    ant: Option<std::path::PathBuf>,
    ant_set: Option<String>,
    case: Option<String>,
    number: Option<String>,
    num_start: usize,
    num_step: usize,
    num_pad: usize,
    num_sep: String,
    ext: Option<String>,
    ext_new: Option<String>,
    dry_run: bool,
) -> Result<(), String> {
    use modules::rename::*;

    let mut cfg = RenameConfig::default();

    // Find/replace simple
    if let Some(f) = find {
        cfg.find = f;
        cfg.replace_with = replace;
        cfg.use_regex = use_regex;
    }

    // Liste de remplacements multiples
    if let Some(list_path) = list {
        cfg.multi_replace = true;
        cfg.replace_list = ReplaceList::load(&list_path)?;
        eprintln!("  loaded {} replace rules from {:?}", cfg.replace_list.rules.len(), list_path);
    }

    // Import Ant Renamer
    if let Some(ant_path) = ant {
        cfg.multi_replace = true;
        cfg.replace_list = ReplaceList::load_ant_renamer_xml(&ant_path, ant_set.as_deref())?;
        eprintln!("  loaded {} Ant Renamer rules from {:?}", cfg.replace_list.rules.len(), ant_path);
    }

    // Case
    if let Some(c) = case {
        cfg.case_mode = match c.to_lowercase().as_str() {
            "lower" => CaseMode::Lower,
            "upper" => CaseMode::Upper,
            "title" => CaseMode::Title,
            "sentence" => CaseMode::Sentence,
            _ => CaseMode::Unchanged,
        };
    }

    // Numbering
    if let Some(pos) = number {
        cfg.num_enabled = true;
        cfg.num_start = num_start;
        cfg.num_step = num_step;
        cfg.num_padding = num_pad;
        cfg.num_sep = num_sep;
        cfg.num_pos = match pos.to_lowercase().as_str() {
            "prefix" => NumPos::Prefix,
            _ => NumPos::Suffix,
        };
    }

    // Extension
    if let Some(e) = ext {
        cfg.ext_mode = match e.to_lowercase().as_str() {
            "lower" => ExtMode::Lower,
            "upper" => ExtMode::Upper,
            "replace" => ExtMode::Replace,
            "remove" => ExtMode::Remove,
            _ => ExtMode::Unchanged,
        };
        if cfg.ext_mode == ExtMode::Replace {
            cfg.ext_new = ext_new.unwrap_or_default();
        }
    }

    let previews = preview(&files, &cfg);

    if dry_run {
        eprintln!("  DRY RUN — no files renamed");
        for (orig, new_name) in &previews {
            let old_name = orig.file_name().unwrap_or_default().to_string_lossy();
            if old_name != *new_name {
                println!("  {} → {}", old_name, new_name);
            } else {
                println!("  {} (unchanged)", old_name);
            }
        }
    } else {
        let results = apply_renames(&previews);
        let ok = results.iter().filter(|r| r.success).count();
        let fail = results.iter().filter(|r| !r.success).count();
        for r in &results {
            if r.success {
                eprintln!("  ✓ {} → {}", r.original.display(), r.new_name);
            } else {
                eprintln!("  ✗ {} → {} : {}", r.original.display(), r.new_name, r.error.as_deref().unwrap_or("?"));
            }
        }
        eprintln!("  {} renamed, {} failed", ok, fail);
    }

    Ok(())
}

// ─── ARCHIVE ────────────────────────────────────────────────────

fn run_archive(action: ArchiveAction) -> Result<(), String> {
    match action {
        ArchiveAction::Compress { files, to, level } => {
            for f in &files {
                let out = make_output(f, &to);
                ok_or_err(modules::archive::compresser(f, &out, &to, level), "archive::compresser")?;
                eprintln!("  compressed {:?} → {}", f, out);
            }
            Ok(())
        }
        ArchiveAction::Extract { files, dest } => {
            for f in &files {
                let destination = dest.clone().unwrap_or_else(|| {
                    let parent = f.parent().unwrap_or(Path::new("."));
                    let stem = f.file_stem().unwrap_or_default().to_string_lossy();
                    parent.join(stem.as_ref()).to_string_lossy().to_string()
                });
                ok_or_err(modules::archive::extraire(f, &destination), "archive::extraire")?;
                eprintln!("  extracted {:?} → {}", f, destination);
            }
            Ok(())
        }
        ArchiveAction::Convert { files, to } => {
            for f in &files {
                ok_or_err(modules::archive::convertir(f, &to), "archive::convertir")?;
                eprintln!("  converted {:?} → {}", f, to);
            }
            Ok(())
        }
        ArchiveAction::Backup { source, dest, exclude } => {
            let exclusions: Vec<&str> = exclude.split(',').map(|s| s.trim()).collect();
            let result = modules::archive::backup_zip(&source, &dest, &exclusions)?;
            eprintln!("  backup: {}", result);
            Ok(())
        }
    }
}

// ─── TOOLS ──────────────────────────────────────────────────────

fn run_tools(action: ToolsAction) -> Result<(), String> {
    use modules::tools::ToolsConfig;
    use std::collections::BTreeMap;

    match action {
        ToolsAction::ListFiles { output, source } => {
            let mut file_sources = BTreeMap::new();
            for s in &source {
                let parts: Vec<&str> = s.splitn(2, '=').collect();
                if parts.len() != 2 {
                    return Err(format!("Invalid source format '{}'. Expected name=path", s));
                }
                file_sources.insert(parts[0].to_string(), parts[1].to_string());
            }
            let cfg = ToolsConfig {
                list_dir: output,
                file_sources,
                folder_sources: Vec::new(),
            };
            let (ok, errs) = modules::tools::lister_fichiers(&cfg);
            eprintln!("  list-files: {} sources processed", ok);
            for e in &errs { eprintln!("  ⚠ {}", e); }
            if errs.is_empty() { Ok(()) } else { Err(format!("{} errors", errs.len())) }
        }
        ToolsAction::ListFolders { output, source } => {
            let cfg = ToolsConfig {
                list_dir: output,
                file_sources: BTreeMap::new(),
                folder_sources: source,
            };
            let (ok, errs) = modules::tools::lister_dossiers(&cfg);
            eprintln!("  list-folders: {} sources processed", ok);
            for e in &errs { eprintln!("  ⚠ {}", e); }
            if errs.is_empty() { Ok(()) } else { Err(format!("{} errors", errs.len())) }
        }
    }
}