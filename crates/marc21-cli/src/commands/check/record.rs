use crate::commands::check::level::Level;

#[derive(Debug)]
pub(crate) struct Record {
    pub(crate) cn: String,
    pub(crate) level: Level,
    pub(crate) message: String,
    pub(crate) rule: String,
}
