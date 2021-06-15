use druid::{im, Data};

#[derive(Data, Debug, Clone)]
struct Header {
    filename: String,
    last_saved_ts: u128,
}

// idea: :md command to render as markdown, for 'reading mode' :)
