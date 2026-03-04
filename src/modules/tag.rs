use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::modules::binaries;
/// Lit un fichier NFO et retourne un HashMap
pub fn lire_nfo(nfo_path: &Path) -> Result<HashMap<String, String>, String> {
    let file = File::open(nfo_path).map_err(|e| e.to_string())?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    let mut buf = Vec::new();
    let mut data: HashMap<String, String> = HashMap::new();
    let mut current_tag = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                current_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
            }
            Ok(Event::Text(e)) => {
                let text = e.decode().unwrap_or_default().trim().to_string();
                if !current_tag.is_empty() && !text.is_empty() {
                    data.insert(current_tag.clone(), text);
                }
            }
            Ok(Event::End(_)) => {
                current_tag.clear();
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Erreur lecture NFO: {}", e)),
            _ => {}
        }
        buf.clear();
    }
    Ok(data)
}
/// Crée le fichier XML temporaire pour mkvpropedit
fn creer_xml_tags(tags: &HashMap<String, String>) -> String {
    let mut xml = String::from("<?xml version=\"1.0\"?>\n<Tags>\n  <Tag>\n    <Targets><TargetTypeValue>50</TargetTypeValue></Targets>\n");
    for (cle, valeur) in tags {
        if !valeur.is_empty() {
            xml.push_str("    <Simple>\n");
            xml.push_str(&format!("      <Name>{}</Name>\n", cle.to_uppercase()));
            xml.push_str(&format!("      <String>{}</String>\n", valeur.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")));
            xml.push_str("    </Simple>\n");
        }
    }
    xml.push_str("  </Tag>\n</Tags>");
    xml
}
/// Lit les tags Matroska existants d'un MKV via ffprobe
fn lire_tags_mkv(mkv_path: &Path) -> HashMap<String, String> {
    let mut tags = HashMap::new();
    let output = binaries::silent_cmd(binaries::get_ffprobe())
        .args([
            "-v", "quiet",
            "-print_format", "json",
            "-show_entries", "format_tags",
            mkv_path.to_str().unwrap(),
        ])
        .output();
    if let Ok(out) = output {
        let json_str = String::from_utf8_lossy(&out.stdout);
        // Parse basique des paires "KEY": "VALUE" dans le bloc tags
        // Format attendu: "format": { "tags": { "KEY": "VALUE", ... } }
        if let Some(tags_start) = json_str.find("\"tags\"") {
            let bloc = &json_str[tags_start..];
            if let Some(brace) = bloc.find('{') {
                let inner = &bloc[brace + 1..];
                if let Some(end) = inner.find('}') {
                    let content = &inner[..end];
                    // Extraire chaque paire "key": "value"
                    let mut rest = content;
                    while let Some(q1) = rest.find('"') {
                        rest = &rest[q1 + 1..];
                        if let Some(q2) = rest.find('"') {
                            let key = rest[..q2].to_string();
                            rest = &rest[q2 + 1..];
                            // Chercher la valeur après le ":"
                            if let Some(colon) = rest.find(':') {
                                rest = &rest[colon + 1..];
                                if let Some(v1) = rest.find('"') {
                                    rest = &rest[v1 + 1..];
                                    if let Some(v2) = rest.find('"') {
                                        let val = rest[..v2].to_string();
                                        tags.insert(key.to_uppercase(), val);
                                        rest = &rest[v2 + 1..];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    tags
}

/// 1. Marquer une vidéo comme 'VU' (cumul avec les tags existants, ne touche pas au NFO)
pub fn marquer_vu(mkv_path: &Path, _nfo_path: &Path, lang_id: &str) -> Result<(), String> {
    // Lire les tags existants du MKV via ffprobe
    let mut tags = lire_tags_mkv(mkv_path);

    // Incrémenter playcount (cumul)
    let pc = tags.get("PLAYCOUNT")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0) + 1;
    tags.insert("PLAYCOUNT".to_string(), pc.to_string());
    tags.insert("WATCHED".to_string(), "true".to_string());
    let watched_label = if lang_id == "fr" { "VU" } else { "WATCHED" };
    tags.insert("KEYWORDS".to_string(), watched_label.to_string());

    // Réinjecter tous les tags (existants + VU) dans le MKV
    let xml_content = creer_xml_tags(&tags);
    let temp_xml = "temp_vu.xml";
    std::fs::write(temp_xml, xml_content).map_err(|e| e.to_string())?;
    let status = binaries::silent_cmd(binaries::get_mkvpropedit())
        .args([mkv_path.to_str().unwrap(), "--tags", &format!("global:{}", temp_xml)])
        .status().map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(temp_xml);
    if status.success() { Ok(()) } else { Err("Erreur marquage VU complet".into()) }
}
/// 2. Modification directe
pub fn modifier_tag(mkv_path: &Path, tag: &str, valeur: &str) -> Result<(), String> {
    let status = binaries::silent_cmd(binaries::get_mkvpropedit())
        .args([
            mkv_path.to_str().unwrap(),
            "--edit", "info",
            "--set", &format!("{}={}", tag, valeur),
        ])
        .status().map_err(|e| e.to_string())?;
    if status.success() { Ok(()) } else { Err("Erreur modification tag".into()) }
}
/// 3. Injection complète depuis NFO
pub fn appliquer_tags(mkv_path: &Path, nfo_path: &Path) -> Result<(), String> {
    let mut tags = lire_nfo(nfo_path)?;
    // On supprime le statut de lecture pour ne pas l'écraser
    tags.remove("playcount");
    tags.remove("watched");
    tags.remove("watchedstatus");
    tags.remove("KEYWORDS"); // Statut VU géré séparément
    // Rating dans COMMENT
    if let Some(rating) = tags.get("value").cloned() {
        if let Ok(r) = rating.parse::<f64>() {
            let rounded = (r * 10.0).round() / 10.0;
            tags.insert("COMMENT".to_string(), format!("{} / 10", rounded));
        }
    }
    // Year → RELEASETIME (copie, year reste aussi)
    if let Some(year) = tags.get("year").cloned() {
        if !year.is_empty() {
            tags.insert("RELEASETIME".to_string(), year);
        }
    }
    // premiered → DATE_RELEASED (année seulement)
    if let Some(premiered) = tags.remove("premiered") {
    if !premiered.is_empty() {
        let annee = premiered.split('-').next().unwrap_or(&premiered).to_string();
        tags.insert("DATE_RELEASED".to_string(), annee);
    }
}
    let xml_content = creer_xml_tags(&tags);
    let temp_xml = "temp_meta.xml";
    std::fs::write(temp_xml, xml_content).map_err(|e| e.to_string())?;
    let status = binaries::silent_cmd(binaries::get_mkvpropedit())
        .args([mkv_path.to_str().unwrap(), "--tags", &format!("global:{}", temp_xml)])
        .status().map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(temp_xml);
    if status.success() { Ok(()) } else { Err("Erreur injection métadonnées".into()) }
}
/// 4. Injection Poster / Fanart / Logo
pub fn ajouter_images_mkv(mkv_path: &Path) -> Result<(), String> {
    let parent = mkv_path.parent().ok_or("Dossier parent introuvable")?;
    let stem_mkv = mkv_path.file_stem().unwrap().to_string_lossy().to_lowercase();
    let mut command = binaries::silent_cmd(binaries::get_mkvpropedit());
    command.arg(mkv_path);
    let mut found = false;
    for entry in std::fs::read_dir(parent).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let name = path.file_name().unwrap().to_string_lossy().to_lowercase();
        if name.contains(&stem_mkv) && (name.contains("poster") || name.contains("fanart") || name.contains("clearlogo")) {
            let attachment_name = if name.contains("poster") { "cover" }
                                 else if name.contains("fanart") { "fanart" }
                                 else { "clearlogo" };
            let mime = if name.ends_with(".png") { "image/png" } else { "image/jpeg" };
            command.args(["--attachment-name", attachment_name, "--attachment-mime-type", mime, "--add-attachment", path.to_str().unwrap()]);
            found = true;
        }
    }
    if found {
        let status = command.status().map_err(|e| e.to_string())?;
        if status.success() { return Ok(()); }
    }
    Ok(())
}
/// 5. Supprimer TOUS les tags et TOUTES les pièces jointes (Reset total)
pub fn supprimer_tous_tags(mkv_path: &Path) -> Result<(), String> {
    let xml_vide = "<?xml version=\"1.0\"?>\n<Tags>\n</Tags>";
    let temp_xml = "temp_reset.xml";
    std::fs::write(temp_xml, xml_vide).map_err(|e| e.to_string())?;
    let status = binaries::silent_cmd(binaries::get_mkvpropedit())
        .args([
            mkv_path.to_str().unwrap(),
            "--tags", &format!("global:{}", temp_xml),
            "--edit", "info", "--set", "title=",
            "--delete-attachment", "mime-type:image/jpeg",
            "--delete-attachment", "mime-type:image/png",
        ])
        .status()
        .map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(temp_xml);
    if status.success() { Ok(()) } else { Err("Erreur reset".into()) }
}