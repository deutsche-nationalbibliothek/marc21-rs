use std::io::Write;

use crate::commands::check::record::Record;
use crate::commands::check::writer::Writer;
use crate::error::CliError;
use crate::utils;

pub(crate) struct TextWriter {
    wtr: utils::Writer,
}

impl TextWriter {
    /// Creates a new TextWriter from an already configured Writer.
    pub(crate) fn from_writer(
        wtr: utils::Writer,
    ) -> Result<Self, CliError> {
        Ok(Self { wtr })
    }

    /// Writes the record to the writer.
    pub(crate) fn write_record(
        &mut self,
        record: Record,
    ) -> Result<(), CliError> {
        writeln!(self.wtr, "{}", record.cn)?;
        Ok(())
    }

    /// Finish the underlying writer.
    pub(crate) fn finish(self) -> Result<(), CliError> {
        self.wtr.finish()?;
        Ok(())
    }
}

impl From<TextWriter> for Writer {
    fn from(wtr: TextWriter) -> Self {
        Writer::Text(Box::new(wtr))
    }
}
