use crate::ac::{SymbolDoc, DocParameter};

lazy_static! {
    pub static ref VARIABLES: [SymbolDoc; 17] = [
        SymbolDoc::var("title", "Get the Title frame from tag.\n\n  Used tags:<br> **MP3**: `TIT2`<br> **FLAC**: `TITLE`<br> **MP4**: `©nam`"),
        SymbolDoc::var("artist", "Get the Artists frame from tag.\n\n  Used tags:<br> **MP3**: `TPE1`<br> **FLAC**: `ARTIST`<br> **MP4**: `©ART`"),
        SymbolDoc::var("artists", "Get the Artists frame from tag.\n\n  Used tags:<br> **MP3**: `TPE1`<br> **FLAC**: `ARTIST`<br> **MP4**: `©ART`"),
        SymbolDoc::var("album", "Get the Album Artists frame from tag.\n\n  Used tags:<br> **MP3**: `TPE2`<br> **FLAC**: `ALBUMARTIST`<br> **MP4**: `aART`"),
        SymbolDoc::var("albumartist", "Get the Album frame from tag.\n\n  Used tags:<br> **MP3**: `TALB`<br> **FLAC**: `ALBUM`<br> **MP4**: `©alb`"),
        SymbolDoc::var("albumartists", "Get the Album frame from tag.\n\n  Used tags:<br> **MP3**: `TALB`<br> **FLAC**: `ALBUM`<br> **MP4**: `©alb`"),
        SymbolDoc::var("key", "Get the Key frame from tag.\n\n  Used tags:<br> **MP3**: `TKEY`<br> **FLAC**: `INITIALKEY`<br> **MP4**: `tmpo`"),
        SymbolDoc::var("bpm", "Get the BPM frame from tag.\n\n  Used tags:<br> **MP3**: `TBPM`<br> **FLAC**: `BPM`<br> **MP4**: `©gen`"),
        SymbolDoc::var("genre", "Get the Genre frame from tag.\n\n  Used tags:<br> **MP3**: `TCON`<br> **FLAC**: `GENRE`<br> **MP4**: `com.apple.iTunes:LABEL`"),
        SymbolDoc::var("style", "Get the Label frame from tag.\n\n  Used tags:<br> **MP3**: `TPUB`<br> **FLAC**: `LABEL`<br> **MP4**: `com.apple.iTunes:ISRC`"),
        SymbolDoc::var("label", "Get the Style frame from tag.\n\n  Used tags:<br> **MP3**: `STYLE`<br> **FLAC**: `STYLE`<br> **MP4**: `com.apple.iTunes:CATALOGNUMBER`"),
        SymbolDoc::var("isrc", "Get the ISRC frame from tag.\n\n  Used tags:<br> **MP3**: `TSRC`<br> **FLAC**: `ISRC`<br> **MP4**: `desc`"),
        SymbolDoc::var("catalognumber", "Get the Catalog Number frame from tag.\n\n  Used tags:<br> **MP3**: `CATALOGNUMBER`<br> **FLAC**: `CATALOGNUMBER`<br> **MP4**: `trkn`"),
        SymbolDoc::var("version", "Get the Version frame from tag.\n\n  Used tags:<br> **MP3**: `TIT3`<br> **FLAC**: `SUBTITLE`<br> **MP4**: `com.apple.iTunes:REMIXER`"),
        SymbolDoc::var("tracknumber", "Get the Track Number frame from tag.\n\n  Used tags:<br> **MP3**: `TRCK`<br> **FLAC**: `TRACKNUMBER`<br> **MP4**: `com.apple.iTunes:KEY`"),
        SymbolDoc::var("duration", "Get the Duration frame from tag.\n\n  Used tags:<br> **MP3**: `TLEN`<br> **FLAC**: `LENGTH`<br> **MP4**: `com.apple.iTunes:STYLE`"),
        SymbolDoc::var("remixer", "Get the Remixer frame from tag.\n\n  Used tags:<br> **MP3**: `TPE4`<br> **FLAC**: `REMIXE`<br> **MP4**: `com.apple.iTunes:LENGTH`"),
    ];

    pub static ref PROPERTIES: [SymbolDoc; 2] = [
        SymbolDoc::prop("first", "Get the first item in an array"),
        SymbolDoc::prop("last", "Get the last item in an array"),
    ];

    pub static ref FUNCTIONS: [SymbolDoc; 12] = [
        SymbolDoc::f("lower", "Convert all to lowercase", vec![]),
        SymbolDoc::f("lowercase", "Convert all to lowercase", vec![]),
        SymbolDoc::f("upper", "Convert all to uppercase", vec![]),
        SymbolDoc::f("uppercase", "Convert all to uppercase", vec![]),
        SymbolDoc::f("slice", "Take a range out of array or substring", vec![DocParameter::n("start", true), DocParameter::n("end", false)]),
        SymbolDoc::f("range", "Take a range out of array or substring", vec![DocParameter::n("start", true), DocParameter::n("end", false)]),
        SymbolDoc::f("capitalize", "Convert first letter to uppercase", vec![]),
        SymbolDoc::f("replace", "Replace text", vec![DocParameter::s("from", true), DocParameter::s("to", true)]),
        SymbolDoc::f("pad", "Pad on the left side with given character to reach given length", vec![DocParameter::s("char", true), DocParameter::n("length", true)]),
        SymbolDoc::f("sort", "Sort the array alphabetically", vec![]),
        SymbolDoc::f("reverse", "Reverse the array", vec![]),
        SymbolDoc::f("join", "Join array into string with custom separator", vec![DocParameter::s("separator", true)]),
    ];
}
