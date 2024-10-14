use flate2::read::GzDecoder;
use std::{ffi::OsString, io::Read};
use tar::Archive;

pub fn extract_commit_hash(input: impl Read) -> Result<Option<String>, std::io::Error> {
    let decoder = GzDecoder::new(input);
    let mut a = Archive::new(decoder);

    let desired_filename = OsString::from(".cargo_vcs_info.json");

    for file in a.entries()? {
        let entry = file?;
        let path = entry.header().path()?;
        if path.file_name() != Some(&desired_filename) {
            continue;
        }
        // TODO: Use a proper struct instead of serde_json::Value
        let value = serde_json::from_reader::<_, serde_json::Value>(entry)?;
        return Ok(value["git"]["sha1"].as_str().map(|s| s.to_owned()));
    }
    Ok(None)
}
