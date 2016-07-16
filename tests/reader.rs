
#[cfg(test)]
mod tests {

    extern crate opml;

    use std::fs::File;
    
    fn make_link(text: &str, url: &str, key: &str) -> opml::Outline
    {
        opml::Outline::Link(opml::Link{text: text.to_string(), url: url.to_string(), key: key.to_string()})
    }

    #[test]
    fn sample_1() {
        let document = opml::parse(File::open("tests/documents/sample_1.opml").unwrap());
        let expected_document = opml::Document {
            version: opml::Version{major: 1, minor: 0},
            head: opml::Head::new(),
            outlines: vec![
                make_link("Local Radio", "http://opml.radiotime.com/Browse.ashx?c=local", "local"),
                make_link("Music", "http://opml.radiotime.com/Browse.ashx?c=music", "music"),
                make_link("Talk", "http://opml.radiotime.com/Browse.ashx?c=talk", "talk"),
                make_link("Sports", "http://opml.radiotime.com/Browse.ashx?c=sports", "sports"),
                make_link("By Location", "http://opml.radiotime.com/Browse.ashx?id=r0", "location"),
                make_link("By Language", "http://opml.radiotime.com/Browse.ashx?c=lang", "language"),
                make_link("Podcasts", "http://opml.radiotime.com/Browse.ashx?c=podcast", "podcast")
        ]};
        assert_eq!(document, expected_document);
    }
}
