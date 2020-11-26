pub const BROWSE_URI: &str = "http://opml.radiotime.com/Browse.ashx";

pub fn search_uri<T: std::fmt::Display>(percent_encoded_query: &T) -> String {
    format!(
        "http://opml.radiotime.com/Search.ashx?query={}",
        percent_encoded_query
    )
}
