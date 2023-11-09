use tracing::error;

#[tracing::instrument(skip(reader))]
pub fn read_csv<
    Reader: std::io::Read,
    Input: for<'de> serde::Deserialize<'de>,
    Output: std::convert::From<Input>,
>(
    reader: Reader,
) -> anyhow::Result<Vec<Output>> {
    let mut csv_reader = csv::Reader::from_reader(reader);
    Ok(csv_reader
        .deserialize::<Input>()
        .filter_map(|r| match r {
            Ok(row) => Some(row),
            Err(err) => {
                error!("{:?}", err);
                None
            }
        })
        .map(|r| r.into())
        .collect())
}
