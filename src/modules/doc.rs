#![allow(dead_code)]
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::io::Read;

use lopdf::content::{Content, Operation};
use lopdf::encryption::crypt_filters::{Aes128CryptFilter, CryptFilter};
use lopdf::encryption::{EncryptionState, EncryptionVersion, Permissions};
use lopdf::{dictionary, Document, Object, ObjectId, SaveOptions, Stream};

// ════════════════════════════════════════════════════════════════════════
//  ENUMS FORMATS
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy)]
pub enum FormatEntree {
    Docx, Csv, Dotx, Json, Log, Md, Odt, Typst, Yaml, Html, Tex, Rst, Pdf, Txt,
}

#[derive(Debug, Clone, Copy)]
pub enum FormatSortie {
    Docx, Html, Md, Odt, Tex, Plain, Pdf,
}

pub fn detecter_format_entree(path: &Path) -> Option<FormatEntree> {
    path.extension()?.to_str().and_then(|ext| match ext.to_lowercase().as_str() {
        "docx" => Some(FormatEntree::Docx),
        "dotx" => Some(FormatEntree::Dotx),
        "csv"  => Some(FormatEntree::Csv),
        "json" => Some(FormatEntree::Json),
        "log"  => Some(FormatEntree::Log),
        "md" | "markdown" => Some(FormatEntree::Md),
        "odt"  => Some(FormatEntree::Odt),
        "typst" | "typ" => Some(FormatEntree::Typst),
        "yaml" | "yml" => Some(FormatEntree::Yaml),
        "html" | "htm" => Some(FormatEntree::Html),
        "tex"  => Some(FormatEntree::Tex),
        "rst"  => Some(FormatEntree::Rst),
        "pdf"  => Some(FormatEntree::Pdf),
        "txt" | "text" | "nfo" => Some(FormatEntree::Txt),
        _ => None,
    })
}

pub fn detecter_format_sortie(output: &str) -> Option<FormatSortie> {
    Path::new(output).extension()?.to_str().and_then(|ext| match ext.to_lowercase().as_str() {
        "docx" => Some(FormatSortie::Docx),
        "html" | "htm" => Some(FormatSortie::Html),
        "md" | "markdown" => Some(FormatSortie::Md),
        "odt"  => Some(FormatSortie::Odt),
        "tex"  => Some(FormatSortie::Tex),
        "txt"  => Some(FormatSortie::Plain),
        "pdf"  => Some(FormatSortie::Pdf),
        _ => None,
    })
}

// ════════════════════════════════════════════════════════════════════════
//  LECTEURS DE FORMATS (Rust pur)
// ════════════════════════════════════════════════════════════════════════

/// Lit un fichier texte brut
fn lire_texte(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("Erreur lecture {:?} : {}", path, e))
}

/// Markdown → HTML via pulldown-cmark
fn md_vers_html(texte: &str) -> String {
    let mut opts = pulldown_cmark::Options::empty();
    opts.insert(pulldown_cmark::Options::ENABLE_TABLES);
    opts.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    opts.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    let parser = pulldown_cmark::Parser::new_ext(texte, opts);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}

/// HTML → Markdown via html2md
fn html_vers_md(html: &str) -> String {
    html2md::parse_html(html)
}

/// HTML → texte brut (strip des tags)
fn html_vers_texte(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    let mut last_was_block = false;
    for ch in html.chars() {
        match ch {
            '<' => {
                in_tag = true;
            }
            '>' => {
                in_tag = false;
            }
            _ if !in_tag => {
                last_was_block = false;
                result.push(ch);
            }
            _ => {
                // à l'intérieur d'un tag, on détecte les balises block pour ajouter des newlines
                if !last_was_block && (ch == 'p' || ch == 'd' || ch == 'h' || ch == 'l' || ch == 'b') {
                    // heuristique simple, pas parfait
                }
            }
        }
    }
    // Décoder les entités HTML courantes
    result.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

/// Texte brut → HTML (wrap dans pre/p)
fn texte_vers_html(texte: &str) -> String {
    let escaped = texte
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    let paragraphs: Vec<String> = escaped.split("\n\n")
        .map(|p| format!("<p>{}</p>", p.replace('\n', "<br/>")))
        .collect();
    format!("<!DOCTYPE html>\n<html><body>\n{}\n</body></html>", paragraphs.join("\n"))
}

/// Extraire le texte d'un fichier DOCX (zip contenant word/document.xml)
fn lire_docx_texte(path: &Path) -> Result<String, String> {
    let file = std::fs::File::open(path)
        .map_err(|e| format!("Erreur ouverture DOCX : {}", e))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Erreur lecture ZIP DOCX : {}", e))?;

    let mut xml_content = String::new();
    {
        let mut doc_file = archive.by_name("word/document.xml")
            .map_err(|e| format!("word/document.xml introuvable : {}", e))?;
        doc_file.read_to_string(&mut xml_content)
            .map_err(|e| format!("Erreur lecture XML : {}", e))?;
    }

    extraire_texte_xml(&xml_content, &["w:t"])
}

/// Extraire le texte d'un fichier ODT (zip contenant content.xml)
fn lire_odt_texte(path: &Path) -> Result<String, String> {
    let file = std::fs::File::open(path)
        .map_err(|e| format!("Erreur ouverture ODT : {}", e))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Erreur lecture ZIP ODT : {}", e))?;

    let mut xml_content = String::new();
    {
        let mut content_file = archive.by_name("content.xml")
            .map_err(|e| format!("content.xml introuvable : {}", e))?;
        content_file.read_to_string(&mut xml_content)
            .map_err(|e| format!("Erreur lecture XML : {}", e))?;
    }

    extraire_texte_xml(&xml_content, &["text:p", "text:h", "text:span"])
}

/// Extraire le texte d'un XML en cherchant les balises spécifiées
fn extraire_texte_xml(xml: &str, balises_texte: &[&str]) -> Result<String, String> {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_str(xml);
    let mut texte = String::new();
    let mut dans_balise_texte = false;
    let mut profondeur_para: u32 = 0;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let name_bytes = e.name().into_inner().to_vec();
                let nom = std::str::from_utf8(&name_bytes).unwrap_or("");
                if balises_texte.iter().any(|b| *b == nom) {
                    dans_balise_texte = true;
                }
                // Détecter les paragraphes pour ajouter des sauts de ligne
                if nom == "w:p" || nom == "text:p" || nom == "text:h" {
                    profondeur_para += 1;
                    if !texte.is_empty() && !texte.ends_with('\n') {
                        texte.push('\n');
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let name_bytes = e.name().into_inner().to_vec();
                let nom = std::str::from_utf8(&name_bytes).unwrap_or("");
                if balises_texte.iter().any(|b| *b == nom) {
                    dans_balise_texte = false;
                }
                if nom == "w:p" || nom == "text:p" || nom == "text:h" {
                    profondeur_para = profondeur_para.saturating_sub(1);
                    if !texte.ends_with('\n') {
                        texte.push('\n');
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if dans_balise_texte || profondeur_para > 0 {
                    let bytes = e.into_inner();
                    let t = String::from_utf8_lossy(bytes.as_ref());
                    texte.push_str(&t);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Erreur parsing XML : {}", e)),
            _ => {}
        }
    }

    Ok(texte.trim().to_string())
}

// ════════════════════════════════════════════════════════════════════════
//  GÉNÉRATEUR PDF (lopdf pur — polices builtin)
// ════════════════════════════════════════════════════════════════════════

/// Génère un PDF à partir de texte brut, avec retour à la ligne et pagination
fn texte_vers_pdf(texte: &str, output: &str) -> Result<(), String> {
    let mut doc = Document::with_version("1.5");

    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
        "Encoding" => "WinAnsiEncoding",
    });

    let font_bold_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica-Bold",
        "Encoding" => "WinAnsiEncoding",
    });

    // A4 dimensions en points
    let page_w = 595.0_f64;
    let page_h = 842.0_f64;
    let marge_gauche = 50.0_f64;
    let marge_droite = 50.0_f64;
    let marge_haut = 50.0_f64;
    let marge_bas = 50.0_f64;
    let taille_police = 11.0_f64;
    let interligne = taille_police * 1.4;
    let largeur_utile = page_w - marge_gauche - marge_droite;
    // Approximation : ~0.5 * taille_police par caractère en Helvetica
    let chars_par_ligne = (largeur_utile / (taille_police * 0.5)) as usize;

    let lignes = decouper_texte(texte, chars_par_ligne);
    let lignes_par_page = ((page_h - marge_haut - marge_bas) / interligne) as usize;

    let pages_contenu: Vec<Vec<&str>> = lignes.chunks(lignes_par_page)
        .map(|c| c.iter().map(|s| s.as_str()).collect())
        .collect();

    if pages_contenu.is_empty() {
        // Document vide : une page blanche
        let content = Content { operations: vec![] };
        let content_bytes = content.encode().map_err(|e| format!("Erreur encodage : {}", e))?;
        let stream_id = doc.add_object(Stream::new(dictionary! {}, content_bytes));
        let resources = dictionary! {
            "Font" => dictionary! {
                "F1" => Object::Reference(font_id),
            },
        };
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "MediaBox" => vec![0.into(), 0.into(), page_w.into(), page_h.into()],
            "Resources" => resources,
            "Contents" => Object::Reference(stream_id),
        });
        let pages_id = doc.add_object(dictionary! {
            "Type" => "Pages",
            "Kids" => vec![Object::Reference(page_id)],
            "Count" => Object::Integer(1),
        });
        if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(page_id) {
            dict.set("Parent", Object::Reference(pages_id));
        }
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => Object::Reference(pages_id),
        });
        doc.trailer.set("Root", catalog_id);
        return sauvegarder(&mut doc, output);
    }

    let mut page_ids = Vec::new();
    for page_lignes in &pages_contenu {
        let mut ops = vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), taille_police.into()]),
            Operation::new("TL", vec![interligne.into()]),
            Operation::new("Td", vec![marge_gauche.into(), (page_h - marge_haut).into()]),
        ];

        for ligne in page_lignes {
            // Encoder en WinAnsi (remplacer les caractères non supportés)
            let encoded = encoder_winansi(ligne);
            ops.push(Operation::new("Tj", vec![Object::string_literal(encoded)]));
            ops.push(Operation::new("T*", vec![]));
        }

        ops.push(Operation::new("ET", vec![]));

        let content = Content { operations: ops };
        let content_bytes = content.encode().map_err(|e| format!("Erreur encodage : {}", e))?;
        let stream_id = doc.add_object(Stream::new(dictionary! {}, content_bytes));

        let resources = dictionary! {
            "Font" => dictionary! {
                "F1" => Object::Reference(font_id),
                "F2" => Object::Reference(font_bold_id),
            },
        };

        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "MediaBox" => vec![0.into(), 0.into(), page_w.into(), page_h.into()],
            "Resources" => resources,
            "Contents" => Object::Reference(stream_id),
        });
        page_ids.push(page_id);
    }

    let pages_id = doc.add_object(dictionary! {
        "Type" => "Pages",
        "Kids" => page_ids.iter().map(|id| Object::Reference(*id)).collect::<Vec<_>>(),
        "Count" => Object::Integer(page_ids.len() as i64),
    });

    for &pid in &page_ids {
        if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(pid) {
            dict.set("Parent", Object::Reference(pages_id));
        }
    }

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    });
    doc.trailer.set("Root", catalog_id);
    doc.compress();

    sauvegarder(&mut doc, output)
}

/// Découpe le texte en lignes en respectant une largeur max en caractères
fn decouper_texte(texte: &str, max_chars: usize) -> Vec<String> {
    let mut lignes = Vec::new();
    for ligne_brute in texte.lines() {
        if ligne_brute.is_empty() {
            lignes.push(String::new());
            continue;
        }
        let mut restant = ligne_brute;
        while !restant.is_empty() {
            if restant.len() <= max_chars {
                lignes.push(restant.to_string());
                break;
            }
            // Chercher le dernier espace avant la limite
            let point_coupe = restant[..max_chars]
                .rfind(' ')
                .unwrap_or(max_chars);
            lignes.push(restant[..point_coupe].to_string());
            restant = restant[point_coupe..].trim_start();
        }
    }
    lignes
}

/// Encode une chaîne pour WinAnsi (PDF Type1 builtin)
/// Remplace les caractères UTF-8 non supportés par '?'
fn encoder_winansi(texte: &str) -> Vec<u8> {
    texte.chars().map(|c| {
        match c as u32 {
            // ASCII standard
            0x20..=0x7E => c as u8,
            // WinAnsi 0x80-0x9F (cp1252 spécifiques)
            0x20AC => 0x80, // €
            0x201A => 0x82, // ‚
            0x0192 => 0x83, // ƒ
            0x201E => 0x84, // „
            0x2026 => 0x85, // …
            0x2020 => 0x86, // †
            0x2021 => 0x87, // ‡
            0x02C6 => 0x88, // ˆ
            0x2030 => 0x89, // ‰
            0x0160 => 0x8A, // Š
            0x2039 => 0x8B, // ‹
            0x0152 => 0x8C, // Œ
            0x017D => 0x8E, // Ž
            0x2018 => 0x91, // '
            0x2019 => 0x92, // '
            0x201C => 0x93, // "
            0x201D => 0x94, // "
            0x2022 => 0x95, // •
            0x2013 => 0x96, // –
            0x2014 => 0x97, // —
            0x02DC => 0x98, // ˜
            0x2122 => 0x99, // ™
            0x0161 => 0x9A, // š
            0x203A => 0x9B, // ›
            0x0153 => 0x9C, // œ
            0x017E => 0x9E, // ž
            0x0178 => 0x9F, // Ÿ
            // Latin-1 0xA0-0xFF (identiques en WinAnsi et Unicode)
            0xA0..=0xFF => c as u8,
            // Tab → espace
            0x09 => 0x20,
            // Tout le reste
            _ => b'?',
        }
    }).collect()
}

// ════════════════════════════════════════════════════════════════════════
//  CONVERSION PRINCIPALE (Rust pur — sans pandoc)
// ════════════════════════════════════════════════════════════════════════

/// Conversion générale : détecte les formats d'entrée/sortie et convertit
pub fn convertir(input: &Path, output: &str) -> bool {
    crate::log_info(&format!("doc::convertir | {:?} -> {}", input, output));

    let fmt_in = detecter_format_entree(input);
    let fmt_out = detecter_format_sortie(output);

    let result = match (fmt_in, fmt_out) {
        // ── Vers PDF ──
        (Some(FormatEntree::Md), Some(FormatSortie::Pdf)) => {
            lire_texte(input).and_then(|t| {
                let html = md_vers_html(&t);
                let texte = html_vers_texte(&html);
                texte_vers_pdf(&texte, output)
            })
        }
        (Some(FormatEntree::Html), Some(FormatSortie::Pdf)) => {
            lire_texte(input).and_then(|html| {
                let texte = html_vers_texte(&html);
                texte_vers_pdf(&texte, output)
            })
        }
        (Some(FormatEntree::Txt) | Some(FormatEntree::Log) | Some(FormatEntree::Csv) |
         Some(FormatEntree::Json) | Some(FormatEntree::Yaml), Some(FormatSortie::Pdf)) => {
            lire_texte(input).and_then(|t| texte_vers_pdf(&t, output))
        }
        (Some(FormatEntree::Docx) | Some(FormatEntree::Dotx), Some(FormatSortie::Pdf)) => {
            lire_docx_texte(input).and_then(|t| texte_vers_pdf(&t, output))
        }
        (Some(FormatEntree::Odt), Some(FormatSortie::Pdf)) => {
            lire_odt_texte(input).and_then(|t| texte_vers_pdf(&t, output))
        }

        // ── Vers HTML ──
        (Some(FormatEntree::Md), Some(FormatSortie::Html)) => {
            lire_texte(input).map(|t| {
                let html = md_vers_html(&t);
                let full = format!("<!DOCTYPE html>\n<html><head><meta charset=\"utf-8\"></head><body>\n{}\n</body></html>", html);
                std::fs::write(output, full)
                    .map_err(|e| format!("Erreur écriture : {}", e))
            }).and_then(|r| r)
        }
        (Some(FormatEntree::Txt) | Some(FormatEntree::Log) | Some(FormatEntree::Csv) |
         Some(FormatEntree::Json) | Some(FormatEntree::Yaml), Some(FormatSortie::Html)) => {
            lire_texte(input).and_then(|t| {
                let html = texte_vers_html(&t);
                std::fs::write(output, html).map_err(|e| format!("Erreur écriture : {}", e))
            })
        }
        (Some(FormatEntree::Docx) | Some(FormatEntree::Dotx), Some(FormatSortie::Html)) => {
            lire_docx_texte(input).and_then(|t| {
                let html = texte_vers_html(&t);
                std::fs::write(output, html).map_err(|e| format!("Erreur écriture : {}", e))
            })
        }
        (Some(FormatEntree::Odt), Some(FormatSortie::Html)) => {
            lire_odt_texte(input).and_then(|t| {
                let html = texte_vers_html(&t);
                std::fs::write(output, html).map_err(|e| format!("Erreur écriture : {}", e))
            })
        }

        // ── Vers Markdown ──
        (Some(FormatEntree::Html), Some(FormatSortie::Md)) => {
            lire_texte(input).and_then(|html| {
                let md = html_vers_md(&html);
                std::fs::write(output, md).map_err(|e| format!("Erreur écriture : {}", e))
            })
        }
        (Some(FormatEntree::Docx) | Some(FormatEntree::Dotx), Some(FormatSortie::Md)) => {
            lire_docx_texte(input).and_then(|t| {
                std::fs::write(output, t).map_err(|e| format!("Erreur écriture : {}", e))
            })
        }

        // ── Vers texte brut ──
        (Some(FormatEntree::Html), Some(FormatSortie::Plain)) => {
            lire_texte(input).and_then(|html| {
                let texte = html_vers_texte(&html);
                std::fs::write(output, texte).map_err(|e| format!("Erreur écriture : {}", e))
            })
        }
        (Some(FormatEntree::Md), Some(FormatSortie::Plain)) => {
            lire_texte(input).and_then(|md| {
                let html = md_vers_html(&md);
                let texte = html_vers_texte(&html);
                std::fs::write(output, texte).map_err(|e| format!("Erreur écriture : {}", e))
            })
        }
        (Some(FormatEntree::Docx) | Some(FormatEntree::Dotx), Some(FormatSortie::Plain)) => {
            lire_docx_texte(input).and_then(|t| {
                std::fs::write(output, t).map_err(|e| format!("Erreur écriture : {}", e))
            })
        }
        (Some(FormatEntree::Odt), Some(FormatSortie::Plain)) => {
            lire_odt_texte(input).and_then(|t| {
                std::fs::write(output, t).map_err(|e| format!("Erreur écriture : {}", e))
            })
        }

        // ── Vers DOCX (basique : texte dans un docx minimal) ──
        (_, Some(FormatSortie::Docx)) => {
            let texte = match fmt_in {
                Some(FormatEntree::Md) => lire_texte(input).map(|t| { let h = md_vers_html(&t); html_vers_texte(&h) }),
                Some(FormatEntree::Html) => lire_texte(input).map(|h| html_vers_texte(&h)),
                Some(FormatEntree::Docx) | Some(FormatEntree::Dotx) => lire_docx_texte(input),
                Some(FormatEntree::Odt) => lire_odt_texte(input),
                _ => lire_texte(input),
            };
            texte.and_then(|t| ecrire_docx_simple(&t, output))
        }

        // ── Copie directe si même format ou inconnu ──
        _ => {
            crate::log_warn(&format!("doc::convertir | conversion non supportée {:?} -> {:?}, copie directe", fmt_in, fmt_out));
            std::fs::copy(input, output)
                .map(|_| ())
                .map_err(|e| format!("Erreur copie : {}", e))
        }
    };

    match result {
        Ok(()) => {
            crate::log_info(&format!("doc::convertir OK | {:?} -> {}", input, output));
            true
        }
        Err(e) => {
            crate::log_error(&format!("doc::convertir ÉCHEC | {}", e));
            false
        }
    }
}

/// Conversion avec formats explicites
pub fn convertir_avec_formats(
    input: &Path, output: &str,
    _format_entree: Option<FormatEntree>,
    _format_sortie: Option<FormatSortie>,
) -> bool {
    // Délègue à convertir() qui détecte les formats par extension
    convertir(input, output)
}

/// Extraire le texte brut d'un document
pub fn extraire_texte(input: &Path, output: &str) -> bool {
    let result = match detecter_format_entree(input) {
        Some(FormatEntree::Docx) | Some(FormatEntree::Dotx) => lire_docx_texte(input),
        Some(FormatEntree::Odt) => lire_odt_texte(input),
        Some(FormatEntree::Html) => lire_texte(input).map(|h| html_vers_texte(&h)),
        Some(FormatEntree::Md) => lire_texte(input).map(|t| {
            let html = md_vers_html(&t);
            html_vers_texte(&html)
        }),
        _ => lire_texte(input),
    };
    match result {
        Ok(texte) => {
            std::fs::write(output, texte).is_ok()
        }
        Err(e) => {
            crate::log_error(&format!("doc::extraire_texte ÉCHEC | {}", e));
            false
        }
    }
}

/// Écrire un DOCX minimal (un seul paragraphe de texte)
fn ecrire_docx_simple(texte: &str, output: &str) -> Result<(), String> {
    use std::io::Write;
    let file = std::fs::File::create(output)
        .map_err(|e| format!("Erreur création DOCX : {}", e))?;
    let mut zip_writer = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // [Content_Types].xml
    zip_writer.start_file("[Content_Types].xml", options)
        .map_err(|e| format!("Erreur ZIP : {}", e))?;
    zip_writer.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#).map_err(|e| format!("Erreur écriture : {}", e))?;

    // _rels/.rels
    zip_writer.start_file("_rels/.rels", options)
        .map_err(|e| format!("Erreur ZIP : {}", e))?;
    zip_writer.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#).map_err(|e| format!("Erreur écriture : {}", e))?;

    // word/_rels/document.xml.rels
    zip_writer.start_file("word/_rels/document.xml.rels", options)
        .map_err(|e| format!("Erreur ZIP : {}", e))?;
    zip_writer.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
</Relationships>"#).map_err(|e| format!("Erreur écriture : {}", e))?;

    // word/document.xml
    zip_writer.start_file("word/document.xml", options)
        .map_err(|e| format!("Erreur ZIP : {}", e))?;

    let mut doc_xml = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>"#);

    for ligne in texte.lines() {
        let escaped = ligne
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        doc_xml.push_str(&format!("\n    <w:p><w:r><w:t xml:space=\"preserve\">{}</w:t></w:r></w:p>", escaped));
    }

    doc_xml.push_str("\n  </w:body>\n</w:document>");

    zip_writer.write_all(doc_xml.as_bytes())
        .map_err(|e| format!("Erreur écriture : {}", e))?;

    zip_writer.finish()
        .map_err(|e| format!("Erreur finalisation ZIP : {}", e))?;

    Ok(())
}

// Fonctions de compatibilité (dead code mais gardées pour l'API)
pub fn convertir_csv(input: &Path, output: &str, _fmt: FormatSortie) -> bool { convertir(input, output) }
pub fn traiter_log(input: &Path, output: &str) -> bool { convertir(input, output) }
pub fn convertir_yaml(input: &Path, output: &str, _fmt: FormatSortie) -> bool { convertir(input, output) }
pub fn convertir_typst(input: &Path, output: &str, _fmt: FormatSortie) -> bool { convertir(input, output) }
pub fn convertir_pdf(input: &Path, output: &str, _fmt: FormatSortie) -> bool { convertir(input, output) }
pub fn convertir_vers_pdf(input: &Path, _format_entree: Option<FormatEntree>) -> Result<String, String> {
    let output = input.with_extension("pdf");
    let output_str = output.to_str().ok_or("Chemin de sortie invalide")?;
    if convertir(input, output_str) {
        Ok(output_str.to_string())
    } else {
        Err("Conversion vers PDF échouée".into())
    }
}

// ════════════════════════════════════════════════════════════════════════
//  HELPERS — conversion intermédiaire pour opérations PDF
// ════════════════════════════════════════════════════════════════════════

fn est_pdf(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}

/// Convertit un fichier non-PDF en PDF temporaire (Rust pur)
fn vers_pdf_temp(input: &Path) -> Result<String, String> {
    let tmp = std::env::temp_dir().join(format!(
        "oxyon_tmp_{}.pdf",
        input.file_stem().unwrap_or_default().to_string_lossy()
    ));
    let tmp_str = tmp.to_str().ok_or("Chemin temp invalide")?;
    crate::log_info(&format!("vers_pdf_temp | {:?} -> {}", input, tmp_str));
    if convertir(input, tmp_str) {
        Ok(tmp_str.to_string())
    } else {
        Err(format!("Conversion vers PDF temporaire échouée pour {}", input.display()))
    }
}

/// Reconvertit un PDF temporaire vers le format original
fn depuis_pdf_temp(pdf_path: &str, output: &str) -> Result<(), String> {
    let fmt_out = detecter_format_sortie(output);
    if fmt_out.is_none() || matches!(fmt_out, Some(FormatSortie::Pdf)) {
        std::fs::copy(pdf_path, output)
            .map_err(|e| format!("Erreur copie {} → {} : {}", pdf_path, output, e))?;
        return Ok(());
    }
    if convertir(Path::new(pdf_path), output) {
        Ok(())
    } else {
        Err(format!("Reconversion depuis PDF échouée pour {}", output))
    }
}

fn nettoyer_temp(path: &str) {
    let _ = std::fs::remove_file(path);
}

fn appliquer_operation_doc<F>(input: &Path, output: &str, op_pdf: F) -> Result<(), String>
where
    F: FnOnce(&Path, &str) -> Result<(), String>,
{
    if est_pdf(input) {
        crate::log_info(&format!("appliquer_operation_doc | PDF direct | {:?} -> {}", input, output));
        return op_pdf(input, output);
    }
    crate::log_info(&format!("appliquer_operation_doc | non-PDF, conversion intermédiaire | {:?}", input));
    let pdf_in = vers_pdf_temp(input)?;
    let pdf_out = format!("{}_out.pdf", pdf_in.trim_end_matches(".pdf"));
    let result = op_pdf(Path::new(&pdf_in), &pdf_out);
    nettoyer_temp(&pdf_in);
    if result.is_err() {
        nettoyer_temp(&pdf_out);
        return result;
    }
    let reconvert = depuis_pdf_temp(&pdf_out, output);
    nettoyer_temp(&pdf_out);
    reconvert
}

// ════════════════════════════════════════════════════════════════════════
//  HELPERS PDF INTERNES
// ════════════════════════════════════════════════════════════════════════

fn obtenir_pages_ordonnees(doc: &Document) -> Vec<ObjectId> {
    let pages = doc.get_pages();
    let mut page_list: Vec<(u32, ObjectId)> = pages.into_iter().collect();
    page_list.sort_by_key(|(num, _)| *num);
    page_list.into_iter().map(|(_, id)| id).collect()
}

fn obtenir_mediabox(doc: &Document, page_id: ObjectId) -> Option<[f64; 4]> {
    let page_dict = doc.get_dictionary(page_id).ok()?;

    let mediabox_obj = if let Ok(mb) = page_dict.get(b"MediaBox") {
        mb.clone()
    } else if let Ok(Object::Reference(parent_id)) = page_dict.get(b"Parent") {
        let parent_dict = doc.get_dictionary(*parent_id).ok()?;
        parent_dict.get(b"MediaBox").ok()?.clone()
    } else {
        return None;
    };

    if let Object::Array(ref arr) = mediabox_obj {
        if arr.len() == 4 {
            let vals: Vec<f64> = arr.iter().filter_map(|o| match o {
                Object::Integer(i) => Some(*i as f64),
                Object::Real(r) => Some((*r).into()),
                _ => None,
            }).collect();
            if vals.len() == 4 {
                return Some([vals[0], vals[1], vals[2], vals[3]]);
            }
        }
    }
    None
}

fn collecter_references(doc: &Document, obj: &Object, ids: &mut Vec<ObjectId>) {
    match obj {
        Object::Reference(id) => {
            if !ids.contains(id) {
                ids.push(*id);
                if let Ok(referenced) = doc.get_object(*id) {
                    collecter_references(doc, referenced, ids);
                }
            }
        }
        Object::Array(arr) => {
            for item in arr { collecter_references(doc, item, ids); }
        }
        Object::Dictionary(dict) => {
            for (_, val) in dict.iter() { collecter_references(doc, val, ids); }
        }
        Object::Stream(stream) => {
            for (_, val) in stream.dict.iter() { collecter_references(doc, val, ids); }
        }
        _ => {}
    }
}

/// Helper : ajouter un overlay (contenu + police + optionnel graphic state) à une page
fn ajouter_overlay_page(
    doc: &mut Document,
    page_id: ObjectId,
    content_bytes: Vec<u8>,
    font_name: &str,
    font_id: ObjectId,
    extra_gs: Option<(&str, ObjectId)>,
) -> Result<(), String> {
    let stream_id = doc.add_object(Stream::new(dictionary! {}, content_bytes));

    // Phase 1 : lire resources et contents (emprunt immutable via get_object)
    let (res_dict, font_ref_id, existing_contents) = {
        let page_obj = doc.get_object(page_id)
            .map_err(|e| format!("Page introuvable : {}", e))?;
        let dict = match page_obj {
            Object::Dictionary(d) => d,
            _ => return Err("L'objet page n'est pas un dictionnaire".into()),
        };

        let res = match dict.get(b"Resources") {
            Ok(Object::Dictionary(r)) => r.clone(),
            Ok(Object::Reference(_id)) => { lopdf::Dictionary::default() }  // marqueur : on résoudra après
            _ => lopdf::Dictionary::default(),
        };
        let res_ref = match dict.get(b"Resources") {
            Ok(Object::Reference(id)) => Some(*id),
            _ => None,
        };

        let contents = match dict.get(b"Contents") {
            Ok(Object::Array(arr)) => {
                let mut arr = arr.clone();
                arr.push(Object::Reference(stream_id));
                Object::Array(arr)
            }
            Ok(Object::Reference(ref_id)) => {
                Object::Array(vec![Object::Reference(*ref_id), Object::Reference(stream_id)])
            }
            _ => Object::Reference(stream_id),
        };

        (res, res_ref, contents)
    };

    // Résoudre les Resources si c'était une référence
    let mut resources = if let Some(ref_id) = font_ref_id {
        doc.get_dictionary(ref_id).cloned().unwrap_or(res_dict)
    } else {
        res_dict
    };

    // Résoudre Font
    let font_ref = match resources.get(b"Font") {
        Ok(Object::Reference(id)) => Some(*id),
        _ => None,
    };
    let mut fonts = match resources.get(b"Font") {
        Ok(Object::Dictionary(f)) => f.clone(),
        _ => lopdf::Dictionary::default(),
    };
    if let Some(fid) = font_ref {
        fonts = doc.get_dictionary(fid).cloned().unwrap_or(fonts);
    }
    fonts.set(font_name, Object::Reference(font_id));
    resources.set("Font", Object::Dictionary(fonts));

    // Résoudre ExtGState si nécessaire
    if let Some((gs_name, gs_id)) = extra_gs {
        let gs_ref = match resources.get(b"ExtGState") {
            Ok(Object::Reference(id)) => Some(*id),
            _ => None,
        };
        let mut ext_gstate = match resources.get(b"ExtGState") {
            Ok(Object::Dictionary(g)) => g.clone(),
            _ => lopdf::Dictionary::default(),
        };
        if let Some(gid) = gs_ref {
            ext_gstate = doc.get_dictionary(gid).cloned().unwrap_or(ext_gstate);
        }
        ext_gstate.set(gs_name, Object::Reference(gs_id));
        resources.set("ExtGState", Object::Dictionary(ext_gstate));
    }

    // Phase 2 : écrire (emprunt mutable)
    if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(page_id) {
        dict.set("Resources", Object::Dictionary(resources));
        dict.set("Contents", existing_contents);
    }

    Ok(())
}

/// Helper : doc.save() retourne Result<File, _> en 0.38, on le mappe en Result<(), _>
fn sauvegarder(doc: &mut Document, output: &str) -> Result<(), String> {
    doc.save(output).map(|_| ()).map_err(|e| format!("Erreur sauvegarde : {}", e))
}

// ════════════════════════════════════════════════════════════════════════
//  PDF SPLIT
// ════════════════════════════════════════════════════════════════════════

fn pdf_split_interne(input: &Path, output_dir: &str) -> Result<Vec<String>, String> {
    std::fs::create_dir_all(output_dir)
        .map_err(|e| format!("Impossible de créer le dossier : {}", e))?;

    let doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;
    let pages = obtenir_pages_ordonnees(&doc);

    if pages.is_empty() {
        return Err("Le PDF ne contient aucune page".into());
    }

    crate::log_info(&format!("pdf_split_interne | {:?} | {} pages -> {}", input, pages.len(), output_dir));

    let base_name = input.file_stem().unwrap_or_default().to_string_lossy();
    let mut fichiers = Vec::new();

    for (i, &page_id) in pages.iter().enumerate() {
        let mut new_doc = Document::with_version(&doc.version);

        let mut objets = vec![page_id];
        if let Ok(page_obj) = doc.get_object(page_id) {
            collecter_references(&doc, page_obj, &mut objets);
        }

        let mut id_map: BTreeMap<ObjectId, ObjectId> = BTreeMap::new();
        for &old_id in &objets {
            if !id_map.contains_key(&old_id) {
                if let Ok(obj) = doc.get_object(old_id) {
                    let new_id = new_doc.add_object(obj.clone());
                    id_map.insert(old_id, new_id);
                }
            }
        }

        let new_page_id = id_map.get(&page_id).copied().unwrap_or(page_id);
        let pages_id = new_doc.new_object_id();

        if let Ok(Object::Dictionary(dict)) = new_doc.get_object_mut(new_page_id) {
            dict.set("Parent", Object::Reference(pages_id));
        }

        let pages_dict = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![Object::Reference(new_page_id)],
            "Count" => Object::Integer(1),
        };
        new_doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

        let catalog_id = new_doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        new_doc.trailer.set("Root", catalog_id);
        new_doc.compress();

        let output_path = format!("{}/{}_page_{:04}.pdf", output_dir, base_name, i + 1);
        sauvegarder(&mut new_doc, &output_path)?;
        fichiers.push(output_path);
    }

    Ok(fichiers)
}

/// Split : fonctionne sur PDF et autres formats (convertit d'abord en PDF)
pub fn pdf_split(input: &Path, output_dir: &str) -> Result<Vec<String>, String> {
    if est_pdf(input) {
        return pdf_split_interne(input, output_dir);
    }
    // Non-PDF : convertir d'abord
    let pdf_tmp = vers_pdf_temp(input)?;
    let result = pdf_split_interne(Path::new(&pdf_tmp), output_dir);
    nettoyer_temp(&pdf_tmp);
    result
}

// ════════════════════════════════════════════════════════════════════════
//  PDF MERGE
// ════════════════════════════════════════════════════════════════════════

pub fn pdf_merge(inputs: &[&Path], output: &str) -> Result<(), String> {
    if inputs.is_empty() {
        return Err("Aucun fichier à fusionner".into());
    }
    crate::log_info(&format!("pdf_merge | {} fichier(s) -> {}", inputs.len(), output));
    for (i, p) in inputs.iter().enumerate() {
        crate::log_info(&format!("  [{}] {:?}", i+1, p));
    }

    // Convertir les non-PDF en PDF temporaire
    let mut documents: Vec<Document> = Vec::new();
    let mut temps: Vec<String> = Vec::new();
    for path in inputs {
        if est_pdf(path) {
            documents.push(
                Document::load(path)
                    .map_err(|e| format!("Erreur chargement {} : {}", path.display(), e))?
            );
        } else {
            let tmp = vers_pdf_temp(path)?;
            documents.push(
                Document::load(&tmp)
                    .map_err(|e| format!("Erreur chargement temp {} : {}", tmp, e))?
            );
            temps.push(tmp);
        }
    }

    let mut max_id = 1;
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut merged = Document::with_version("1.5");

    for mut doc in documents {
        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc.get_pages().into_iter().map(|(_, object_id)| {
                (object_id, doc.get_object(object_id).unwrap().to_owned())
            }).collect::<BTreeMap<ObjectId, Object>>()
        );
        documents_objects.extend(doc.objects);
    }

    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in documents_objects.iter() {
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                catalog_object = Some((
                    catalog_object.map_or(*object_id, |(id, _)| id),
                    object.clone(),
                ));
            }
            b"Pages" => {
                if let Ok(dict) = object.as_dict() {
                    let mut dict = dict.clone();
                    if let Some((_, ref old)) = pages_object {
                        if let Ok(old_dict) = old.as_dict() {
                            dict.extend(old_dict);
                        }
                    }
                    pages_object = Some((
                        pages_object.map_or(*object_id, |(id, _)| id),
                        Object::Dictionary(dict),
                    ));
                }
            }
            b"Page" | b"Outlines" | b"Outline" => {}
            _ => { merged.objects.insert(*object_id, object.clone()); }
        }
    }

    let pages_obj = pages_object.ok_or("Nœud Pages introuvable")?;
    let catalog_obj = catalog_object.ok_or("Catalogue introuvable")?;

    for (object_id, object) in documents_pages.iter() {
        if let Ok(dict) = object.as_dict() {
            let mut dict = dict.clone();
            dict.set("Parent", pages_obj.0);
            merged.objects.insert(*object_id, Object::Dictionary(dict));
        }
    }

    if let Ok(dict) = pages_obj.1.as_dict() {
        let mut dict = dict.clone();
        dict.set("Count", documents_pages.len() as u32);
        dict.set("Kids", documents_pages.keys()
            .map(|id| Object::Reference(*id))
            .collect::<Vec<_>>());
        merged.objects.insert(pages_obj.0, Object::Dictionary(dict));
    }

    if let Ok(dict) = catalog_obj.1.as_dict() {
        let mut dict = dict.clone();
        dict.set("Pages", pages_obj.0);
        dict.remove(b"Outlines");
        merged.objects.insert(catalog_obj.0, Object::Dictionary(dict));
    }

    merged.trailer.set("Root", catalog_obj.0);
    merged.max_id = merged.objects.len() as u32;
    merged.renumber_objects();
    merged.compress();

    let result = sauvegarder(&mut merged, output);
    for t in &temps { nettoyer_temp(t); }
    result
}

// ════════════════════════════════════════════════════════════════════════
//  PDF ROTATE
// ════════════════════════════════════════════════════════════════════════

fn pdf_rotate_interne(input: &Path, output: &str, rotation: u16, pages_cibles: Option<&[u32]>) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    if !matches!(rotation, 90 | 180 | 270) {
        return Err(format!("Rotation invalide : {}. Utilisez 90, 180 ou 270.", rotation));
    }

    let pages = obtenir_pages_ordonnees(&doc);
    for (i, &page_id) in pages.iter().enumerate() {
        let page_num = (i + 1) as u32;
        if !pages_cibles.map_or(true, |c| c.contains(&page_num)) { continue; }

        if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(page_id) {
            let actuel = dict.get(b"Rotate").ok()
                .and_then(|r| if let Object::Integer(i) = r { Some(*i) } else { None })
                .unwrap_or(0);
            dict.set("Rotate", Object::Integer((actuel + rotation as i64) % 360));
        }
    }

    sauvegarder(&mut doc, output)
}

pub fn pdf_rotate(input: &Path, output: &str, rotation: u16, pages_cibles: Option<&[u32]>) -> Result<(), String> {
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_rotate_interne(pdf_in, pdf_out, rotation, pages_cibles)
    })
}

// ════════════════════════════════════════════════════════════════════════
//  PDF COMPRESS
// ════════════════════════════════════════════════════════════════════════

fn pdf_compresser_interne(input: &Path, output: &str) -> Result<u64, String> {
    let taille_avant = std::fs::metadata(input).map(|m| m.len()).unwrap_or(0);

    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    doc.delete_zero_length_streams();
    doc.prune_objects();
    doc.renumber_objects();
    doc.compress();

    let options = SaveOptions::builder()
        .use_object_streams(true)
        .use_xref_streams(true)
        .compression_level(9)
        .build();

    let mut file = std::fs::File::create(output)
        .map_err(|e| format!("Erreur création fichier : {}", e))?;
    doc.save_with_options(&mut file, options)
        .map_err(|e| format!("Erreur sauvegarde : {}", e))?;

    let taille_apres = std::fs::metadata(output).map(|m| m.len()).unwrap_or(0);
    Ok(taille_avant.saturating_sub(taille_apres))
}

pub fn pdf_compresser(input: &Path, output: &str) -> Result<u64, String> {
    crate::log_info(&format!("pdf_compresser | {:?} -> {}", input, output));
    if est_pdf(input) {
        let result = pdf_compresser_interne(input, output);
        if let Ok(bytes_gagnés) = &result {
            crate::log_info(&format!("pdf_compresser OK | {} octets économisés", bytes_gagnés));
        }
        return result;
    }
    let pdf_tmp = vers_pdf_temp(input)?;
    let result = pdf_compresser_interne(Path::new(&pdf_tmp), output);
    nettoyer_temp(&pdf_tmp);
    result
}

// ════════════════════════════════════════════════════════════════════════
//  PDF CROP
// ════════════════════════════════════════════════════════════════════════

fn pdf_crop_interne(
    input: &Path, output: &str,
    x_pct: f64, y_pct: f64, w_pct: f64, h_pct: f64,
    pages_cibles: Option<&[u32]>,
) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    let pages = obtenir_pages_ordonnees(&doc);
    for (i, &page_id) in pages.iter().enumerate() {
        let page_num = (i + 1) as u32;
        if !pages_cibles.map_or(true, |c| c.contains(&page_num)) { continue; }

        let [mb_x, mb_y, mb_w, mb_h] = obtenir_mediabox(&doc, page_id)
            .ok_or_else(|| format!("MediaBox introuvable page {}", page_num))?;

        let largeur = mb_w - mb_x;
        let hauteur = mb_h - mb_y;

        let new_llx = mb_x + (largeur * x_pct / 100.0);
        let new_lly = mb_y + (hauteur * y_pct / 100.0);
        let new_urx = new_llx + (largeur * w_pct / 100.0);
        let new_ury = new_lly + (hauteur * h_pct / 100.0);

        if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(page_id) {
            dict.set("CropBox", Object::Array(vec![
                Object::Real(format!("{:.2}", new_llx).parse().unwrap_or(0.0)),
                Object::Real(format!("{:.2}", new_lly).parse().unwrap_or(0.0)),
                Object::Real(format!("{:.2}", new_urx).parse().unwrap_or(0.0)),
                Object::Real(format!("{:.2}", new_ury).parse().unwrap_or(0.0)),
            ]));
        }
    }

    sauvegarder(&mut doc, output)
}

pub fn pdf_crop(
    input: &Path, output: &str,
    x_pct: f64, y_pct: f64, w_pct: f64, h_pct: f64,
    pages_cibles: Option<&[u32]>,
) -> Result<(), String> {
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_crop_interne(pdf_in, pdf_out, x_pct, y_pct, w_pct, h_pct, pages_cibles)
    })
}

// ════════════════════════════════════════════════════════════════════════
//  PDF ORGANIZE — Réorganise / supprime des pages
// ════════════════════════════════════════════════════════════════════════

fn pdf_organiser_interne(input: &Path, output: &str, nouvel_ordre: &[u32]) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    let pages = obtenir_pages_ordonnees(&doc);
    let total = pages.len() as u32;

    for &num in nouvel_ordre {
        if num == 0 || num > total {
            return Err(format!("Page invalide : {}. Le PDF a {} pages.", num, total));
        }
    }

    let new_kids: Vec<Object> = nouvel_ordre.iter()
        .map(|&n| Object::Reference(pages[n as usize - 1]))
        .collect();
    let new_count = new_kids.len() as i64;

    let catalog = doc.catalog()
        .map_err(|e| format!("Catalogue introuvable : {}", e))?;
    let pages_id = match catalog.get(b"Pages") {
        Ok(Object::Reference(id)) => *id,
        _ => return Err("Référence Pages introuvable".into()),
    };

    if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(pages_id) {
        dict.set("Kids", Object::Array(new_kids));
        dict.set("Count", Object::Integer(new_count));
    }

    sauvegarder(&mut doc, output)
}

pub fn pdf_organiser(input: &Path, output: &str, nouvel_ordre: &[u32]) -> Result<(), String> {
    let ordre = nouvel_ordre.to_vec();
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_organiser_interne(pdf_in, pdf_out, &ordre)
    })
}

pub fn pdf_supprimer_pages(input: &Path, output: &str, pages_a_supprimer: &[u32]) -> Result<(), String> {
    // On a besoin du nombre total de pages → charger d'abord
    let total = if est_pdf(input) {
        let doc = Document::load(input).map_err(|e| format!("Erreur : {}", e))?;
        doc.get_pages().len() as u32
    } else {
        let pdf_tmp = vers_pdf_temp(input)?;
        let doc = Document::load(&pdf_tmp).map_err(|e| format!("Erreur : {}", e))?;
        let total = doc.get_pages().len() as u32;
        nettoyer_temp(&pdf_tmp);
        total
    };

    let pages_a_garder: Vec<u32> = (1..=total)
        .filter(|n| !pages_a_supprimer.contains(n))
        .collect();

    if pages_a_garder.is_empty() {
        return Err("Impossible de supprimer toutes les pages".into());
    }

    pdf_organiser(input, output, &pages_a_garder)
}

// ════════════════════════════════════════════════════════════════════════
//  PDF PAGE NUMBERS
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy)]
pub enum PositionNumero {
    BasCentre, BasGauche, BasDroite,
    HautCentre, HautGauche, HautDroite,
}

fn pdf_numeroter_interne(
    input: &Path, output: &str,
    debut: u32,
    position: PositionNumero,
    taille_police: f64,
) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    let font_id = doc.add_object(font_dict);

    let pages = obtenir_pages_ordonnees(&doc);

    for (i, &page_id) in pages.iter().enumerate() {
        let numero = format!("{}", debut + i as u32);
        let mediabox = obtenir_mediabox(&doc, page_id).unwrap_or([0.0, 0.0, 595.0, 842.0]);
        let largeur = mediabox[2] - mediabox[0];
        let hauteur = mediabox[3] - mediabox[1];

        let (x, y) = match position {
            PositionNumero::BasCentre  => (largeur / 2.0 - 10.0, 30.0),
            PositionNumero::BasGauche  => (40.0, 30.0),
            PositionNumero::BasDroite  => (largeur - 60.0, 30.0),
            PositionNumero::HautCentre => (largeur / 2.0 - 10.0, hauteur - 30.0),
            PositionNumero::HautGauche => (40.0, hauteur - 30.0),
            PositionNumero::HautDroite => (largeur - 60.0, hauteur - 30.0),
        };

        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["Fnum".into(), taille_police.into()]),
                Operation::new("Td", vec![x.into(), y.into()]),
                Operation::new("Tj", vec![Object::string_literal(numero)]),
                Operation::new("ET", vec![]),
            ],
        };
        let content_bytes = content.encode()
            .map_err(|e| format!("Erreur encodage contenu : {}", e))?;

        ajouter_overlay_page(&mut doc, page_id, content_bytes, "Fnum", font_id, None)?;
    }

    sauvegarder(&mut doc, output)
}

pub fn pdf_numeroter(
    input: &Path, output: &str,
    debut: u32,
    position: PositionNumero,
    taille_police: f64,
) -> Result<(), String> {
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_numeroter_interne(pdf_in, pdf_out, debut, position, taille_police)
    })
}

// ════════════════════════════════════════════════════════════════════════
//  PDF PROTECT — AES-128, V4
// ════════════════════════════════════════════════════════════════════════

fn pdf_proteger_interne(
    input: &Path, output: &str,
    mot_de_passe_owner: &str,
    mot_de_passe_user: &str,
    autoriser_impression: bool,
    autoriser_copie: bool,
) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    let mut perms = Permissions::empty();
    if autoriser_impression {
        perms |= Permissions::PRINTABLE;
        perms |= Permissions::PRINTABLE_IN_HIGH_QUALITY;
    }
    if autoriser_copie {
        perms |= Permissions::COPYABLE;
        perms |= Permissions::COPYABLE_FOR_ACCESSIBILITY;
    }

    let crypt_filter: Arc<dyn CryptFilter> = Arc::new(Aes128CryptFilter);
    let version = EncryptionVersion::V4 {
        document: &doc,
        encrypt_metadata: true,
        crypt_filters: BTreeMap::from([(b"StdCF".to_vec(), crypt_filter)]),
        stream_filter: b"StdCF".to_vec(),
        string_filter: b"StdCF".to_vec(),
        owner_password: mot_de_passe_owner,
        user_password: mot_de_passe_user,
        permissions: perms,
    };

    let state = EncryptionState::try_from(version)
        .map_err(|e| format!("Erreur création chiffrement : {}", e))?;

    doc.encrypt(&state)
        .map_err(|e| format!("Erreur chiffrement : {}", e))?;

    sauvegarder(&mut doc, output)
}

pub fn pdf_proteger(
    input: &Path, output: &str,
    mot_de_passe_owner: &str,
    mot_de_passe_user: &str,
    autoriser_impression: bool,
    autoriser_copie: bool,
) -> Result<(), String> {
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_proteger_interne(pdf_in, pdf_out, mot_de_passe_owner, mot_de_passe_user, autoriser_impression, autoriser_copie)
    })
}

// ════════════════════════════════════════════════════════════════════════
//  PDF UNLOCK
// ════════════════════════════════════════════════════════════════════════

pub fn pdf_dechiffrer(input: &Path, output: &str, mot_de_passe: &str) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    if !doc.is_encrypted() {
        return sauvegarder(&mut doc, output);
    }

    doc.decrypt(mot_de_passe)
        .map_err(|e| format!("Mot de passe incorrect ou erreur déchiffrement : {}", e))?;

    doc.trailer.remove(b"Encrypt");

    sauvegarder(&mut doc, output)
}

// ════════════════════════════════════════════════════════════════════════
//  PDF REPAIR
// ════════════════════════════════════════════════════════════════════════

fn pdf_reparer_interne(input: &Path, output: &str) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF (fichier trop corrompu ?) : {}", e))?;

    doc.delete_zero_length_streams();
    doc.prune_objects();
    doc.renumber_objects();
    doc.compress();

    sauvegarder(&mut doc, output)
}

pub fn pdf_reparer(input: &Path, output: &str) -> Result<(), String> {
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_reparer_interne(pdf_in, pdf_out)
    })
}

// ════════════════════════════════════════════════════════════════════════
//  PDF WATERMARK
// ════════════════════════════════════════════════════════════════════════

fn pdf_watermark_interne(
    input: &Path, output: &str,
    texte: &str,
    taille_police: f64,
    opacite: f64,
    pages_cibles: Option<&[u32]>,
) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    let gs_dict = dictionary! {
        "Type" => "ExtGState",
        "CA"  => opacite,
        "ca"  => opacite,
    };
    let gs_id = doc.add_object(gs_dict);

    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica-Bold",
    };
    let font_id = doc.add_object(font_dict);

    let pages = obtenir_pages_ordonnees(&doc);

    for (i, &page_id) in pages.iter().enumerate() {
        let page_num = (i + 1) as u32;
        if !pages_cibles.map_or(true, |c| c.contains(&page_num)) { continue; }

        let mediabox = obtenir_mediabox(&doc, page_id).unwrap_or([0.0, 0.0, 595.0, 842.0]);
        let cx = (mediabox[2] - mediabox[0]) / 2.0;
        let cy = (mediabox[3] - mediabox[1]) / 2.0;

        let angle: f64 = 45.0_f64.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let content = Content {
            operations: vec![
                Operation::new("q", vec![]),
                Operation::new("gs", vec!["GSwm".into()]),
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["Fwm".into(), taille_police.into()]),
                Operation::new("rg", vec![0.7.into(), 0.7.into(), 0.7.into()]),
                Operation::new("Tm", vec![
                    cos_a.into(), sin_a.into(),
                    (-sin_a).into(), cos_a.into(),
                    cx.into(), cy.into(),
                ]),
                Operation::new("Tj", vec![Object::string_literal(texte.to_string())]),
                Operation::new("ET", vec![]),
                Operation::new("Q", vec![]),
            ],
        };
        let content_bytes = content.encode()
            .map_err(|e| format!("Erreur encodage watermark : {}", e))?;

        ajouter_overlay_page(
            &mut doc, page_id, content_bytes,
            "Fwm", font_id,
            Some(("GSwm", gs_id)),
        )?;
    }

    sauvegarder(&mut doc, output)
}

pub fn pdf_watermark(
    input: &Path, output: &str,
    texte: &str,
    taille_police: f64,
    opacite: f64,
    pages_cibles: Option<&[u32]>,
) -> Result<(), String> {
    let texte = texte.to_string();
    let opacite = opacite;
    let taille = taille_police;
    let pages = pages_cibles.map(|p| p.to_vec());
    appliquer_operation_doc(input, output, move |pdf_in, pdf_out| {
        pdf_watermark_interne(pdf_in, pdf_out, &texte, taille, opacite, pages.as_deref())
    })
}