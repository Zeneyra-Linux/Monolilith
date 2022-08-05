const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const LICENSE: &str = "ZPL-1.0";
const LICENSE_RICH: &str = "Zeneyra Pulic License 1.0";

use kagero::printer::Printer;

pub fn info(printer: &mut Printer) {
    // TODO: Print info about the app
}