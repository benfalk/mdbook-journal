use crate::prelude::*;

pub struct CliEntryGeneration {}

pub fn std_io() -> CliEntryGeneration {
    CliEntryGeneration {}
}

impl EntryGenerationTrait for CliEntryGeneration {
    fn collect_value(&self, variable: &Variable) -> Result<Option<MetaValue>> {
        use std::io::Write;
        let stdout = &mut std::io::stdout();
        let stdin = &mut std::io::stdin();
        let data = &mut String::with_capacity(64);
        write!(stdout, "({})❯ ", variable.key())?;
        stdout.flush()?;
        stdin.read_line(data)?;

        match data.trim() {
            "" => Ok(None),
            value => Ok(Some(MetaValue::String(value.to_owned()))),
        }
    }
}
