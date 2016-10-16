
#[cfg(test)]
mod tests {

    extern crate opml;

    use std::fs::File;

    fn make_group(text: &str, key: &str, outlines: Vec<opml::Outline>) -> opml::Outline
    {
        opml::Outline::Group{text: text.to_string(), key: key.to_string(), outlines: outlines}
    }

    fn make_link(text: &str, url: &str, key: &str) -> opml::Outline
    {
        opml::Outline::Link(opml::Link{text: text.to_string(), url: url.to_string(), key: key.to_string()})
    }

    fn make_audio(text: &str, subtext: &str, url: &str, bitrate: u16, reliability: u16, format: opml::Format, item: &str, image: &str, guide_id: &str, genre_id: &str, now_playing_id: &str, preset_id: &str) -> opml::Outline
    {
        opml::Outline::Audio(opml::Audio{text: text.to_string(), subtext: subtext.to_string(), url: url.to_string(), bitrate: bitrate, reliability: reliability, format: format, item: item.to_string(), image: image.to_string(), guide_id: guide_id.to_string(), genre_id: genre_id.to_string(), now_playing_id: now_playing_id.to_string(), preset_id: preset_id.to_string()})
    }

    #[test]
    fn sample_1() {
        let document = opml::parse(File::open("tests/documents/sample_1.opml").unwrap()).unwrap();
        let expected_document = opml::Document {
            version: opml::Version{major: 1, minor: 0},
            head: opml::Head{title: "Browse".to_string(), status: Some(200)},
            outlines: vec![
                make_link("Local Radio", "http://opml.radiotime.com/Browse.ashx?c=local", "local"),
                make_link("Music", "http://opml.radiotime.com/Browse.ashx?c=music", "music"),
                make_link("Talk", "http://opml.radiotime.com/Browse.ashx?c=talk", "talk"),
                make_link("Sports", "http://opml.radiotime.com/Browse.ashx?c=sports", "sports"),
                make_link("By Location", "http://opml.radiotime.com/Browse.ashx?id=r0", "location"),
                make_link("By Language", "http://opml.radiotime.com/Browse.ashx?c=lang", "language"),
                make_link("Podcasts", "http://opml.radiotime.com/Browse.ashx?c=podcast", "podcast")
            ]
        };
        assert_eq!(document, expected_document);
    }

    #[test]
    fn sample_2() {
        let document = opml::parse(File::open("tests/documents/sample_2.opml").unwrap()).unwrap();
        let expected_document = opml::Document {
            version: opml::Version{major: 1, minor: 0},
            head: opml::Head{title: "Kraków".to_string(), status: Some(300)},
            outlines: vec![
                make_group("Stacje", "stations", vec![
                    make_audio("Anty Radio 101.3 (Rock)", "Rockowo Bezkompromisowi", "http://opml.radiotime.com/Tune.ashx?id=s76368", 96, 10, opml::Format::MP3, "station", "http://cdn-radiotime-logos.tunein.com/s9608q.png", "s76368", "g19", "s76368", "s76368"),
                    make_audio("KRK.FM 102.4 (Top 40-Pop)", "Polska", "http://opml.radiotime.com/Tune.ashx?id=s16527", 128, 10, opml::Format::MP3, "station", "http://cdn-radiotime-logos.tunein.com/s16527q.png", "s16527", "g61", "s16527", "s16527"),
                    make_audio("PR R Krakow Nowy Sacz 90.0 (Rock)", "Polska", "http://opml.radiotime.com/Tune.ashx?id=s103067", 32, 85, opml::Format::MP3, "station", "http://cdn-radiotime-logos.tunein.com/s103064q.png", "s103067", "g19", "s103067", "s103067"),
                    make_audio("PR R Krakow Tarnow 101.0 (Rock)", "Polska", "http://opml.radiotime.com/Tune.ashx?id=s103069", 32, 100, opml::Format::MP3, "station", "http://cdn-radiotime-logos.tunein.com/s103064q.png", "s103069", "g19", "s103069", "s103069")
                ])
            ]
        };
        assert_eq!(document, expected_document);
    }
}
