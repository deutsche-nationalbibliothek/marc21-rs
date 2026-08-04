use std::path::PathBuf;

use crate::commands::check::record::Record;
use crate::commands::check::writer::csv::CsvWriter;
use crate::commands::check::writer::dvrf::DvrfWriter;
use crate::commands::check::writer::text::TextWriter;
use crate::error::CliError;
use crate::utils::WriterBuilder;

mod csv;
mod dvrf;
mod text;

pub(crate) enum Writer {
    Text(Box<TextWriter>),
    Csv(Box<CsvWriter>),
    Dvrf(Box<DvrfWriter>),
}

impl Writer {
    pub(crate) fn try_from_path_or_stdout(
        output: Option<PathBuf>,
    ) -> Result<Self, CliError> {
        let wtr = WriterBuilder::default()
            .try_from_path_or_stdout(output.clone())?;

        let filename = output
            .clone()
            .and_then(|path| path.to_str().map(|s| s.to_owned()))
            .unwrap_or_default();

        let writer = if output.is_none()
            || filename.ends_with(".csv.gz")
            || filename.ends_with(".csv")
        {
            CsvWriter::from_writer(wtr)?.into()
        } else if filename.ends_with(".txt.gz")
            || filename.ends_with(".txt")
        {
            TextWriter::from_writer(wtr)?.into()
        } else if filename.ends_with(".json.gz")
            || filename.ends_with(".json")
        {
            DvrfWriter::from_writer(wtr)?.into()
        } else {
            // Use DVRF output by default
            DvrfWriter::from_writer(wtr)?.into()
        };

        Ok(writer)
    }

    pub(crate) fn write_record(
        &mut self,
        record: Record,
    ) -> Result<(), CliError> {
        match self {
            Self::Text(wtr) => wtr.write_record(record),
            Self::Csv(wtr) => wtr.write_record(record),
            Self::Dvrf(wtr) => wtr.write_record(record),
        }
    }

    pub(crate) fn finish(self) -> Result<(), CliError> {
        match self {
            Self::Text(wtr) => wtr.finish(),
            Self::Csv(wtr) => wtr.finish(),
            Self::Dvrf(wtr) => wtr.finish(),
        }
    }
}
