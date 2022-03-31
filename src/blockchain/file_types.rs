use serde::{ Serialize, Deserialize };
use serde_json;


#[derive(Serialize, Deserialize)]
enum FileType {
    AAC,
    ABW,
    ARC,
    AVIF,
    AVI,
    AZW,
    BIN,
    BMP,
    BZ,
    BZ2,
    CDA,
    CSH,
    CSS,
    CSV,
    DOC,
    DOCX,
    EOT,
    EPUB,
    GZ,
    GIF,
    HTML,
    ICO,
    ICS,
    JAR,
    JPG,
    JS,
    JSON,
    JSONLD,
    MIDI,
    MJS,
    MP3,
    MP4,
    MPEG,
    MPKG,
    ODP,
    ODS,
    ODT,
    OGA,
    OGV,
    OGX,
    OPUS,
    OTF,
    PNG,
    PDF,
    PHP,
    PPT,
    PPTX,
    RAR,
    RTF,
    SH,
    SVG,
    SWF,
    TAR,
    TIF,
    TIFF,
    TS,
    TTF,
    TXT,
    VSD,
    WAV,
    WEBM,
    WEBP,
    WOFF,
    WOFF2,
    XHTML,
    XML,
    XSL,
    XSLX,
    XUL,
    ZIP,
}

impl FileType {
    pub fn from_str(file_type: &str) -> Option<Self> {
        let final_ext = file_type.to_ascii_lowercase();
        let as_enum: Result<Self, _> = serde_json::from_str(&final_ext);
        drop(final_ext);
        match as_enum {
            Ok(final_enum) => {
                return Some(final_enum);
            },
            Err(err) => {
                println!("{}", &err);
                return None;
            }
        }
    } 
    
    pub fn to_mime_type(file_type: FileType) -> String {

    }
}