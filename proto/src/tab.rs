use std::sync::Arc;
use snafu::prelude::*;

use notation_model::prelude::*;

#[derive(Debug, Snafu)]
pub enum LoadTabError {
    #[snafu(display("Unable to decode tab"))]
    DecodeRon { source: ron::error::SpannedError },
    #[snafu(display("Unable to parse tab"))]
    ParseTab { source: ParseError },
}

pub type LoadTabResult = Result<Arc<Tab>, LoadTabError>;

pub fn load_tab(bytes: &[u8]) -> LoadTabResult {
    let proto_tab = ron::de::from_bytes::<ProtoTab>(bytes)
        .context(DecodeRonSnafu {})?;
    let tab = Tab::try_parse_arc(proto_tab, false, None)
        .context(ParseTabSnafu {})?;
    Ok(tab)
}