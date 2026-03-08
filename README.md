# Oxyon & Oxyon Desk

![Oxyon Logo](assets/oxyon_logo.png)

[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE.txt)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)

## What is Oxyon?

Oxyon is a portable desktop toolkit that centralizes a collection of everyday multimedia scripts into a single GUI application.
Built with Rust and [egui](https://github.com/emilide/egui).
Think of it as a Swiss army knife for file processing: conversion, renaming, tagging, scraping.
All in one place, with bundled binaries (ffmpeg, ffprobe, mkvpropedit) so there's nothing extra to install.

Available in English and French.

## What is Oxyon Desk?

Oxyon Desk was born from a real need.
In a professional environment where tools like Adobe Acrobat Pro aren't available,
the only option is often to upload sensitive documents to third-party websites (iLovePDF, iLoveIMG) — which never feels right.

Oxyon Desk brings those capabilities locally: document conversion, image processing, archiving, renaming.
All offline, with no external binaries, no API calls, and no data leaving your machine.
It's built for professional use where privacy and autonomy matter.

## Modules

| Module | Oxyon | Oxyon Desk |
|--------|:-----:|:----------:|
| Archives (7Z, ZIP, TAR) | ✅ | ✅ |
| Audio (MP3, FLAC, AAC, OGG) | ✅ | ❌ |
| Documents (DOCX, PDF, MD, ODT, HTML, LaTeX) | ✅ | ✅ |
| File renamer (find/replace, insert, numbering, case, extensions) | ✅ | ✅ |
| Pictures (15+ formats: AVIF, JXL, RAW, SVG, PSD, WebP, EXR…) | ✅ | ✅ |
| Scraper (TMDB, Fanart) | ✅ | ❌ |
| Tagger (MKV tagging) | ✅ | ❌ |
| Tools | ✅ | ✅ |
| Video (mkv, mp4, webm) | ✅ | ❌ |


## Key differences

| | Oxyon | Oxyon Desk |
|---|:---:|:---:|:---:|
| Purpose | Key differences | Swiss army knife for multimedia | Offline document & image processing |
| Bundled binaries |  ffmpeg, ffprobe, mkvpropedit | ✅ | ❌ |
| API keys required | FanArt, TMDB (for scraping only) | ✅ | ❌ |
| Internet access | for scraping only | ✅ | ❌ |
| Portable | - | ✅ | ✅ |

## Platforms

| | Linux ARM64 | Linux x64 | Mac ARM64 | Windows x64 |
|---|:---:|:---:|:---:|:---:|
| Oxyon | ✅ | ✅ | ✅ | ✅ |
| Oxyon Desk | ✅ | ✅ | ✅ | ✅ |

The source code is Mac ARM ready.
I don't have any, so I need to compile this with Github CI, which cost a lot of ratio compare to other platforms.
i will post sometimes.

## Quick Start

1. Download the latest release from [Releases](../../releases)
2. Run the executable — no installation needed
3. Drop your files or browse to select them
4. Choose your output format
5. Click "Execute"

## Building from source

```bash
# Full build (Oxyon)
cargo build --release

# Desk variant
cargo build --release --no-default-features --features bundled

# Optimized distribution build
cargo build --profile dist
```

## License

This project is licensed under the GNU General Public License v3.0 — see [LICENSE.txt](LICENSE.txt) for details.