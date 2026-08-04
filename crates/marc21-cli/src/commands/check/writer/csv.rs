use csv::WriterBuilder;

use crate::commands::check::record::Record;
use crate::commands::check::writer::Writer;
use crate::error::CliError;
use crate::utils;

pub(crate) struct CsvWriter {
    wtr: csv::Writer<utils::Writer>,
}

impl CsvWriter {
    /// Creates a new TextWriter from an already configured Writer.
    pub(crate) fn from_writer(
        wtr: utils::Writer,
    ) -> Result<Self, CliError> {
        let mut wtr = WriterBuilder::default().from_writer(wtr);
        wtr.write_record(["cn", "rule", "message"])?;

        Ok(Self { wtr })
    }

    /// Writes the record to the writer.
    pub(crate) fn write_record(
        &mut self,
        record: Record,
    ) -> Result<(), CliError> {
        self.wtr.write_record(&[
            record.cn,
            record.level.to_string(),
            record.message,
        ])?;

        Ok(())
    }

    /// Finish the underlying writer.
    pub(crate) fn finish(mut self) -> Result<(), CliError> {
        self.wtr.flush()?;

        if let Ok(inner) = self.wtr.into_inner() {
            inner.finish()?;
        }

        Ok(())
    }
}

impl From<CsvWriter> for Writer {
    fn from(wtr: CsvWriter) -> Self {
        Writer::Csv(Box::new(wtr))
    }
}
