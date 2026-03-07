// tools.rs — Lister fichiers et dossiers multimédia (remplace oxyfl.ps1 + oxyflf.ps1)

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;

// ─────────────────────────────────────────────
//  CONFIG
// ─────────────────────────────────────────────

/// Configuration du module Tools, persistée dans config.toml
#[derive(Clone, Debug)]
pub struct ToolsConfig {
    /// Dossier de sortie des fichiers .txt générés
    pub list_dir: String,
    /// Sources pour lister les fichiers : nom → chemin
    pub file_sources: BTreeMap<String, String>,
    /// Sources pour lister les dossiers
    pub folder_sources: Vec<String>,
}

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            list_dir: String::new(),
            file_sources: BTreeMap::new(),
            folder_sources: Vec::new(),
        }
    }
}

impl ToolsConfig {
    /// Charge depuis une table TOML déjà parsée
    pub fn load(parsed: &toml::Table) -> Self {
        let mut cfg = Self::default();

        if let Some(tools) = parsed.get("tools").and_then(|t| t.as_table()) {
            if let Some(dir) = tools.get("list_dir").and_then(|v| v.as_str()) {
                cfg.list_dir = dir.to_string();
            }
        }

        if let Some(tf) = parsed.get("tools_files").and_then(|t| t.as_table()) {
            for (name, val) in tf {
                if let Some(path) = val.as_str() {
                    cfg.file_sources.insert(name.clone(), path.to_string());
                }
            }
        }

        if let Some(td) = parsed.get("tools_folders").and_then(|t| t.as_table()) {
            if let Some(arr) = td.get("sources").and_then(|v| v.as_array()) {
                for item in arr {
                    if let Some(s) = item.as_str() {
                        cfg.folder_sources.push(s.to_string());
                    }
                }
            }
        }

        cfg
    }

    /// Sauvegarde dans la table TOML (merge)
    pub fn save(&self, parsed: &mut toml::Table) {
        // [tools]
        let tools = parsed
            .entry("tools")
            .or_insert(toml::Value::Table(toml::Table::new()));
        if let Some(t) = tools.as_table_mut() {
            t.insert(
                "list_dir".to_string(),
                toml::Value::String(self.list_dir.clone()),
            );
        }

        // [tools_files]
        let mut tf = toml::Table::new();
        for (name, path) in &self.file_sources {
            tf.insert(name.clone(), toml::Value::String(path.clone()));
        }
        parsed.insert("tools_files".to_string(), toml::Value::Table(tf));

        // [tools_folders]
        let mut td = toml::Table::new();
        let arr: Vec<toml::Value> = self
            .folder_sources
            .iter()
            .map(|s| toml::Value::String(s.clone()))
            .collect();
        td.insert("sources".to_string(), toml::Value::Array(arr));
        parsed.insert("tools_folders".to_string(), toml::Value::Table(td));
    }

    pub fn is_empty(&self) -> bool {
        self.list_dir.is_empty() && self.file_sources.is_empty() && self.folder_sources.is_empty()
    }
}

// ─────────────────────────────────────────────
//  LISTER FICHIERS (oxyfl.ps1)
// ─────────────────────────────────────────────

/// Liste les fichiers de chaque source et écrit un .txt par entrée dans list_dir.
/// Retourne (réussites, erreurs).
pub fn lister_fichiers(cfg: &ToolsConfig) -> (usize, Vec<String>) {
    let mut ok = 0usize;
    let mut erreurs: Vec<String> = Vec::new();

    let list_dir = Path::new(&cfg.list_dir);
    if !list_dir.exists() {
        if let Err(e) = fs::create_dir_all(list_dir) {
            erreurs.push(format!("Impossible de créer {}: {}", cfg.list_dir, e));
            return (ok, erreurs);
        }
    }

    for (name, dir_str) in &cfg.file_sources {
        let dir = Path::new(dir_str);
        let out_path = list_dir.join(format!("{}.txt", name));

        if !dir.exists() {
            erreurs.push(format!("{} → inaccessible", dir_str));
            continue;
        }

        match collect_files_recursive(dir) {
            Ok(files) if files.is_empty() => {
                // Pas de fichiers : supprimer le .txt s'il existait
                let _ = fs::remove_file(&out_path);
                ok += 1;
            }
            Ok(files) => {
                match write_lines(&out_path, &files) {
                    Ok(()) => ok += 1,
                    Err(e) => erreurs.push(format!("{} → écriture: {}", name, e)),
                }
            }
            Err(e) => erreurs.push(format!("{} → parcours: {}", name, e)),
        }
    }

    (ok, erreurs)
}

// ─────────────────────────────────────────────
//  LISTER DOSSIERS (oxyflf.ps1)
// ─────────────────────────────────────────────

/// Liste les dossiers de chaque source et écrit multimedia.txt dans list_dir.
/// Retourne (nombre de sources traitées, erreurs).
pub fn lister_dossiers(cfg: &ToolsConfig) -> (usize, Vec<String>) {
    let mut ok = 0usize;
    let mut erreurs: Vec<String> = Vec::new();

    let list_dir = Path::new(&cfg.list_dir);
    if !list_dir.exists() {
        if let Err(e) = fs::create_dir_all(list_dir) {
            erreurs.push(format!("Impossible de créer {}: {}", cfg.list_dir, e));
            return (ok, erreurs);
        }
    }

    let out_path = list_dir.join("multimedia.txt");
    let mut all_dirs: Vec<String> = Vec::new();

    for dir_str in &cfg.folder_sources {
        let dir = Path::new(dir_str);
        if !dir.exists() {
            all_dirs.push(format!("# Inaccessible : {}", dir_str));
            erreurs.push(format!("{} → inaccessible", dir_str));
            continue;
        }

        match collect_dirs_recursive(dir) {
            Ok(dirs) => {
                all_dirs.extend(dirs);
                ok += 1;
            }
            Err(e) => erreurs.push(format!("{} → parcours: {}", dir_str, e)),
        }
    }

    if all_dirs.is_empty() {
        let _ = fs::remove_file(&out_path);
    } else if let Err(e) = write_lines(&out_path, &all_dirs) {
        erreurs.push(format!("multimedia.txt → écriture: {}", e));
    }

    (ok, erreurs)
}

// ─────────────────────────────────────────────
//  HELPERS
// ─────────────────────────────────────────────

fn collect_files_recursive(dir: &Path) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    collect_files_inner(dir, &mut result)?;
    result.sort();
    Ok(result)
}

fn collect_files_inner(dir: &Path, out: &mut Vec<String>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {}", dir.display(), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_inner(&path, out)?;
        } else {
            out.push(path.to_string_lossy().to_string());
        }
    }
    Ok(())
}

fn collect_dirs_recursive(dir: &Path) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    collect_dirs_inner(dir, &mut result)?;
    result.sort();
    Ok(result)
}

fn collect_dirs_inner(dir: &Path, out: &mut Vec<String>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {}", dir.display(), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            out.push(path.to_string_lossy().to_string());
            collect_dirs_inner(&path, out)?;
        }
    }
    Ok(())
}

fn write_lines(path: &Path, lines: &[String]) -> Result<(), String> {
    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
    for line in lines {
        writeln!(file, "{}", line).map_err(|e| e.to_string())?;
    }
    Ok(())
}