use dvrf::Document;

use crate::commands::check::level::Level;
use crate::commands::check::record::Record;
use crate::commands::check::writer::Writer;
use crate::error::CliError;
use crate::utils;

pub(crate) struct DvrfWriter {
    wtr: utils::Writer,
    doc: Document,
}

impl DvrfWriter {
    /// Creates a new DvrfWriter from an already configured Writer.
    pub(crate) fn from_writer(
        wtr: utils::Writer,
    ) -> Result<Self, CliError> {
        Ok(Self {
            wtr,
            doc: Document::new(),
        })
    }

    /// Writes the record to the writer.
    pub(crate) fn write_record(
        &mut self,
        record: Record,
    ) -> Result<(), CliError> {
        self.doc.write_record(
            dvrf::Record::new()
                .with_position("id", &record.cn)
                .with_message(record.message)
                .with_level(record.level)
                .with_type(record.rule),
        );

        Ok(())
    }

    /// Finish the underlying writer.
    pub(crate) fn finish(mut self) -> Result<(), CliError> {
        self.doc.write_to(&mut self.wtr, true)?;
        self.wtr.finish()?;

        Ok(())
    }
}

impl From<DvrfWriter> for Writer {
    fn from(wtr: DvrfWriter) -> Self {
        Writer::Dvrf(Box::new(wtr))
    }
}

impl From<Level> for dvrf::Level {
    fn from(level: Level) -> Self {
        match level {
            Level::Error => dvrf::Level::Error,
            Level::Warning => dvrf::Level::Warning,
            Level::Info => dvrf::Level::Info,
        }
    }
}
