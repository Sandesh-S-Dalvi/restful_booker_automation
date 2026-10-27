use crate::reporting::metrics::TestSuite;
use anyhow::Result;
use std::fs::File;
use std::io::Write;

pub struct JsonReporter;

impl JsonReporter {
    pub fn write(suite: &TestSuite, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(suite)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
