use std::path::PathBuf;

/// Sous-commandes CLI d'Oxyon
#[derive(clap::Parser)]
#[command(name = "oxyon-cli", version = env!("CARGO_PKG_VERSION"), about = "Oxyon CLI — multimedia processing toolbox")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Langue (en / fr)
    #[arg(long, default_value = "en", global = true)]
    pub lang: String,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Image processing (convert, resize, rotate, crop)
    Pic {
        #[command(subcommand)]
        action: PicAction,
    },
    /// Document conversion & PDF tools
    Doc {
        #[command(subcommand)]
        action: DocAction,
    },
    /// MKV tagging (mark watched, edit tags, inject NFO, attach images, reset)
    #[cfg(feature = "api")]
    Tag {
        #[command(subcommand)]
        action: TagAction,
    },
    /// Batch rename files
    Rename {
        /// Files to rename
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Find string
        #[arg(long)]
        find: Option<String>,

        /// Replace string
        #[arg(long, default_value = "")]
        replace: String,

        /// Use regex for find/replace
        #[arg(long)]
        regex: bool,

        /// Load a replace list file (TSV)
        #[arg(long)]
        list: Option<PathBuf>,

        /// Load an Ant Renamer XML file
        #[arg(long)]
        ant: Option<PathBuf>,

        /// Ant Renamer set name (if --ant is used)
        #[arg(long)]
        ant_set: Option<String>,

        /// Case transform: lower, upper, title, sentence
        #[arg(long)]
        case: Option<String>,

        /// Add numbering (prefix or suffix)
        #[arg(long)]
        number: Option<String>,

        /// Numbering start
        #[arg(long, default_value = "1")]
        num_start: usize,

        /// Numbering step
        #[arg(long, default_value = "1")]
        num_step: usize,

        /// Numbering zero-padding width
        #[arg(long, default_value = "2")]
        num_pad: usize,

        /// Numbering separator
        #[arg(long, default_value = " ")]
        num_sep: String,

        /// Extension mode: lower, upper, replace, remove
        #[arg(long)]
        ext: Option<String>,

        /// New extension (with --ext replace)
        #[arg(long)]
        ext_new: Option<String>,

        /// Dry run (preview only, no rename)
        #[arg(long)]
        dry_run: bool,
    },
    /// Archive (compress, extract, convert, backup)
    Archive {
        #[command(subcommand)]
        action: ArchiveAction,
    },
    /// Tools (list files, list folders)
    Tools {
        #[command(subcommand)]
        action: ToolsAction,
    },
}

// ─── PIC ────────────────────────────────────────────────────────

#[derive(clap::Subcommand)]
pub enum PicAction {
    /// Convert image(s) to another format
    Convert {
        /// Input file(s)
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Output format (png, jpg, webp, bmp, tiff, jxl…)
        #[arg(long, short)]
        to: String,
        /// Quality ratio (1-10, default 2)
        #[arg(long, default_value = "2")]
        quality: u32,
        /// JXL mode: lossless, folder, pivot
        #[arg(long, default_value = "lossless")]
        jxl_mode: String,
    },
    /// Resize image(s)
    Resize {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Target width in pixels
        #[arg(long)]
        width: Option<u32>,
        /// Target height in pixels
        #[arg(long)]
        height: Option<u32>,
        /// Max file size in KB
        #[arg(long)]
        max_kb: Option<u32>,
    },
    /// Rotate image(s)
    Rotate {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Angle: 90, 180, 270
        #[arg(long, default_value = "90")]
        angle: u32,
    },
    /// Crop image(s) (percentages)
    Crop {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        #[arg(long)] x: u32,
        #[arg(long)] y: u32,
        #[arg(long, short)] width: u32,
        #[arg(long, short = 'H')] height: u32,
    },
    /// Read EXIF metadata
    Exif {
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
    /// Strip EXIF metadata
    StripExif {
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
}

// ─── DOC ────────────────────────────────────────────────────────

#[derive(clap::Subcommand)]
pub enum DocAction {
    /// Convert document (md, html, txt, docx, odt → pdf, md, html, txt)
    Convert {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Output format (pdf, md, html, txt)
        #[arg(long, short)]
        to: String,
    },
    /// Split PDF into individual pages
    PdfSplit {
        #[arg(required = true)]
        file: PathBuf,
    },
    /// Merge multiple PDFs
    PdfMerge {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Output file
        #[arg(long, short)]
        output: String,
    },
    /// Rotate PDF pages
    PdfRotate {
        #[arg(required = true)]
        file: PathBuf,
        /// Angle: 90, 180, 270
        #[arg(long, default_value = "90")]
        angle: u16,
        /// Pages to rotate (e.g. "1,3,5"). Omit for all pages.
        #[arg(long)]
        pages: Option<String>,
    },
    /// Compress a PDF
    PdfCompress {
        #[arg(required = true)]
        file: PathBuf,
    },
    /// Crop PDF pages (percentages)
    PdfCrop {
        #[arg(required = true)]
        file: PathBuf,
        #[arg(long)] x: f64,
        #[arg(long)] y: f64,
        #[arg(long, short)] width: f64,
        #[arg(long, short = 'H')] height: f64,
        #[arg(long)]
        pages: Option<String>,
    },
    /// Reorder PDF pages
    PdfOrganize {
        #[arg(required = true)]
        file: PathBuf,
        /// New page order (e.g. "3,1,2,4")
        #[arg(long)]
        order: String,
    },
    /// Delete pages from a PDF
    PdfDelete {
        #[arg(required = true)]
        file: PathBuf,
        /// Pages to delete (e.g. "2,5")
        #[arg(long)]
        pages: String,
    },
    /// Add page numbers to a PDF
    PdfNumber {
        #[arg(required = true)]
        file: PathBuf,
        /// Starting number
        #[arg(long, default_value = "1")]
        start: u32,
        /// Position: BasCentre, BasGauche, BasDroite, HautCentre, HautGauche, HautDroite
        #[arg(long, default_value = "BasCentre")]
        position: String,
        /// Font size
        #[arg(long, default_value = "10")]
        size: f64,
    },
    /// Protect a PDF with passwords
    PdfProtect {
        #[arg(required = true)]
        file: PathBuf,
        #[arg(long)] owner_pass: String,
        #[arg(long)] user_pass: String,
        #[arg(long, default_value = "true")]
        allow_print: bool,
        #[arg(long, default_value = "true")]
        allow_copy: bool,
    },
    /// Unlock a protected PDF
    PdfUnlock {
        #[arg(required = true)]
        file: PathBuf,
        #[arg(long)] password: String,
    },
    /// Repair a PDF
    PdfRepair {
        #[arg(required = true)]
        file: PathBuf,
    },
    /// Add a watermark to a PDF
    PdfWatermark {
        #[arg(required = true)]
        file: PathBuf,
        /// Watermark text
        #[arg(long)]
        text: String,
        /// Font size
        #[arg(long, default_value = "60")]
        size: f64,
        /// Opacity (0.0 - 1.0)
        #[arg(long, default_value = "0.15")]
        opacity: f64,
        #[arg(long)]
        pages: Option<String>,
    },
}

// ─── TAG ────────────────────────────────────────────────────────

#[cfg(feature = "api")]
#[derive(clap::Subcommand)]
pub enum TagAction {
    /// Mark MKV file(s) as watched (cumulative playcount)
    MarquerVu {
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
    /// Edit a single MKV tag
    Edit {
        #[arg(required = true)]
        file: PathBuf,
        /// Tag name (e.g. TITLE, COMMENT)
        #[arg(long)]
        tag: String,
        /// Tag value
        #[arg(long)]
        value: String,
    },
    /// Inject tags from NFO file into MKV
    Nfo {
        /// MKV file
        #[arg(required = true)]
        file: PathBuf,
        /// NFO file path
        #[arg(long)]
        nfo: PathBuf,
    },
    /// Attach poster/fanart/logo images to MKV
    Images {
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
    /// Remove all tags and attachments from MKV (reset)
    Reset {
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
}

// ─── ARCHIVE ────────────────────────────────────────────────────

#[derive(clap::Subcommand)]
pub enum ArchiveAction {
    /// Compress file(s) or folder(s)
    Compress {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Output format (zip, 7z, tar, gz)
        #[arg(long, short)]
        to: String,
        /// Compression level (1-9)
        #[arg(long, default_value = "6")]
        level: u32,
    },
    /// Extract archive(s)
    Extract {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Destination folder (default: same folder, named after archive)
        #[arg(long)]
        dest: Option<String>,
    },
    /// Convert archive to another format
    Convert {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Target format
        #[arg(long, short)]
        to: String,
    },
    /// Backup a folder (zip with exclusions)
    Backup {
        /// Source folder
        #[arg(required = true)]
        source: PathBuf,
        /// Destination folder
        #[arg(long)]
        dest: String,
        /// Exclusions (comma-separated, e.g. ".git,target")
        #[arg(long, default_value = ".git,.github,target")]
        exclude: String,
    },
}

// ─── TOOLS ──────────────────────────────────────────────────────

#[derive(clap::Subcommand)]
pub enum ToolsAction {
    /// List all files from sources into .txt files
    ListFiles {
        /// Output directory for .txt files
        #[arg(long)]
        output: String,
        /// Sources as name=path pairs (e.g. "movies=/media/movies")
        #[arg(long, required = true)]
        source: Vec<String>,
    },
    /// List all folders from sources into multimedia.txt
    ListFolders {
        /// Output directory
        #[arg(long)]
        output: String,
        /// Source folder paths
        #[arg(long, required = true)]
        source: Vec<String>,
    },
}