use anyhow::bail;

#[tracing::instrument(skip(reader))]
pub fn read_csv<
    Reader: std::io::Read,
    Input: for<'de> serde::Deserialize<'de>,
    Output: std::convert::From<Input>,
>(
    reader: Reader,
) -> anyhow::Result<Vec<Output>> {
    let mut output = vec![];
    for row in csv::Reader::from_reader(reader).deserialize::<Input>() {
        match row {
            Ok(parsed) => output.push(parsed.into()),
            Err(err) => bail!("parsing failed: {}", err),
        }
    }

    Ok(output)
}
