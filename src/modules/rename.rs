use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum CaseMode { Unchanged, Lower, Upper, Title, Sentence }
impl CaseMode {
    pub fn label(&self) -> &'static str {
        match self {
            CaseMode::Unchanged => "Unchanged", CaseMode::Lower => "lowercase",
            CaseMode::Upper => "UPPERCASE", CaseMode::Title => "Title Case",
            CaseMode::Sentence => "Sentence case",
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum NumPos { Prefix, Suffix }
#[derive(Debug, Clone, PartialEq)]
pub enum ExtMode { Unchanged, Lower, Upper, Replace, Remove }
impl ExtMode {
    pub fn label(&self) -> &'static str {
        match self {
            ExtMode::Unchanged => "Unchanged", ExtMode::Lower => "lowercase",
            ExtMode::Upper => "UPPERCASE", ExtMode::Replace => "Replace", ExtMode::Remove => "Remove",
        }
    }
}
#[derive(Debug, Clone)]
pub struct RenameConfig {
    pub find: String, pub replace_with: String, pub use_regex: bool,
    pub insert_text: String, pub insert_pos: usize,
    pub delete_from: usize, pub delete_count: usize, pub delete_enabled: bool,
    pub num_enabled: bool, pub num_start: usize, pub num_step: usize,
    pub num_padding: usize, pub num_pos: NumPos, pub num_sep: String,
    pub case_mode: CaseMode,
    pub strip_leading_dots: bool, pub strip_trailing_spaces: bool,
    pub strip_double_spaces: bool, pub strip_chars: String,
    pub ext_mode: ExtMode, pub ext_new: String,
}
impl Default for RenameConfig {
    fn default() -> Self {
        Self {
            find: String::new(), replace_with: String::new(), use_regex: false,
            insert_text: String::new(), insert_pos: 0,
            delete_from: 0, delete_count: 0, delete_enabled: false,
            num_enabled: false, num_start: 1, num_step: 1, num_padding: 2,
            num_pos: NumPos::Suffix, num_sep: " ".into(),
            case_mode: CaseMode::Unchanged,
            strip_leading_dots: false, strip_trailing_spaces: true,
            strip_double_spaces: true, strip_chars: String::new(),
            ext_mode: ExtMode::Unchanged, ext_new: String::new(),
        }
    }
}
pub fn preview(files: &[PathBuf], cfg: &RenameConfig) -> Vec<(PathBuf, String)> {
    files.iter().enumerate().map(|(i, p)| (p.clone(), compute_new_name(p, i, cfg))).collect()
}
fn compute_new_name(path: &Path, index: usize, cfg: &RenameConfig) -> String {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
    let ext_orig = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string();
    let mut name = stem;
    if !cfg.find.is_empty() { name = name.replace(&cfg.find, &cfg.replace_with); }
    if !cfg.insert_text.is_empty() {
        let pos = cfg.insert_pos.min(name.chars().count());
        let mut chars: Vec<char> = name.chars().collect();
        for (j, c) in cfg.insert_text.chars().enumerate() { chars.insert(pos + j, c); }
        name = chars.into_iter().collect();
    }
    if cfg.delete_enabled && cfg.delete_count > 0 {
        let chars: Vec<char> = name.chars().collect();
        let from = cfg.delete_from.min(chars.len());
        let to = (from + cfg.delete_count).min(chars.len());
        name = chars[..from].iter().chain(chars[to..].iter()).collect();
    }
    if !cfg.strip_chars.is_empty() { name = name.chars().filter(|c| !cfg.strip_chars.contains(*c)).collect(); }
    if cfg.strip_leading_dots { name = name.trim_start_matches('.').to_string(); }
    if cfg.strip_double_spaces { while name.contains("  ") { name = name.replace("  ", " "); } }
    if cfg.strip_trailing_spaces { name = name.trim().to_string(); }
    name = match &cfg.case_mode {
        CaseMode::Unchanged => name,
        CaseMode::Lower => name.to_lowercase(),
        CaseMode::Upper => name.to_uppercase(),
        CaseMode::Title => name.split_whitespace().map(|w| {
            let mut c = w.chars();
            match c.next() { None => String::new(), Some(f) => f.to_uppercase().to_string() + &c.as_str().to_lowercase() }
        }).collect::<Vec<_>>().join(" "),
        CaseMode::Sentence => {
            let mut c = name.chars();
            match c.next() { None => String::new(), Some(f) => f.to_uppercase().to_string() + &c.as_str().to_lowercase() }
        }
    };
    if cfg.num_enabled {
        let n = cfg.num_start + index * cfg.num_step;
        let num_str = if cfg.num_padding > 0 { format!("{:0>width$}", n, width = cfg.num_padding) } else { n.to_string() };
        name = match cfg.num_pos {
            NumPos::Prefix => format!("{}{}{}", num_str, cfg.num_sep, name),
            NumPos::Suffix => format!("{}{}{}", name, cfg.num_sep, num_str),
        };
    }
    let ext = match &cfg.ext_mode {
        ExtMode::Unchanged => if ext_orig.is_empty() { String::new() } else { format!(".{}", ext_orig) },
        ExtMode::Lower => if ext_orig.is_empty() { String::new() } else { format!(".{}", ext_orig.to_lowercase()) },
        ExtMode::Upper => if ext_orig.is_empty() { String::new() } else { format!(".{}", ext_orig.to_uppercase()) },
        ExtMode::Replace => if cfg.ext_new.is_empty() { String::new() } else { format!(".{}", cfg.ext_new.trim_start_matches('.')) },
        ExtMode::Remove => String::new(),
    };
    format!("{}{}", name, ext)
}
pub struct RenameResult {
    pub original: PathBuf, pub new_name: String, pub success: bool, pub error: Option<String>,
}
pub fn apply_renames(previews: &[(PathBuf, String)]) -> Vec<RenameResult> {
    previews.iter().map(|(original, new_name)| {
        let parent = original.parent().unwrap_or(Path::new(""));
        let dest = parent.join(new_name);
        if dest == *original { return RenameResult { original: original.clone(), new_name: new_name.clone(), success: true, error: None }; }
        if dest.exists() { return RenameResult { original: original.clone(), new_name: new_name.clone(), success: false, error: Some(format!("Already exists: {}", new_name)) }; }
        match std::fs::rename(original, &dest) {
            Ok(_)  => RenameResult { original: original.clone(), new_name: new_name.clone(), success: true,  error: None },
            Err(e) => RenameResult { original: original.clone(), new_name: new_name.clone(), success: false, error: Some(e.to_string()) },
        }
    }).collect()
}