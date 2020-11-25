use std::fs::File;
use std::io;

#[cfg(test)]

fn make_group(text: &str, key: &str, outlines: Vec<tunein::Outline>) -> tunein::Outline {
    tunein::Outline::Group(tunein::Group {
        text: text.into(),
        key: key.into(),
        outlines: outlines,
    })
}

fn make_link(text: &str, url: &str, key: &str, guide_id: &str) -> tunein::Outline {
    tunein::Outline::Link(tunein::Link {
        text: text.into(),
        url: url.into(),
        key: key.into(),
        guide_id: guide_id.into(),
    })
}

fn make_audio(
    text: &str,
    subtext: &str,
    url: &str,
    bitrate: u16,
    reliability: u16,
    format: tunein::Format,
    item: &str,
    image: &str,
    guide_id: &str,
    genre_id: &str,
    now_playing_id: &str,
    preset_id: &str,
) -> tunein::Outline {
    tunein::Outline::Audio(tunein::Audio {
        text: text.into(),
        subtext: subtext.into(),
        url: url.into(),
        bitrate: bitrate,
        reliability: reliability,
        format: format,
        item: item.into(),
        image: image.into(),
        guide_id: guide_id.into(),
        genre_id: genre_id.into(),
        now_playing_id: now_playing_id.into(),
        preset_id: preset_id.into(),
    })
}

#[test]
fn empty() {
    assert!(tunein::read(io::empty()).is_err());
}

#[test]
fn sample_1() {
    let document = tunein::read(File::open("tests/documents/sample_1.opml").unwrap()).unwrap();
    let expected_document = tunein::Document {
        version: tunein::Version { major: 1, minor: 0 },
        head: tunein::Head {
            title: "Browse".into(),
            status: Some(200),
        },
        outlines: vec![
            make_link(
                "Local Radio",
                "http://opml.radiotime.com/Browse.ashx?c=local",
                "local",
                "",
            ),
            make_link(
                "Music",
                "http://opml.radiotime.com/Browse.ashx?c=music",
                "music",
                "",
            ),
            make_link(
                "Talk",
                "http://opml.radiotime.com/Browse.ashx?c=talk",
                "talk",
                "",
            ),
            make_link(
                "Sports",
                "http://opml.radiotime.com/Browse.ashx?c=sports",
                "sports",
                "",
            ),
            make_link(
                "By Location",
                "http://opml.radiotime.com/Browse.ashx?id=r0",
                "location",
                "",
            ),
            make_link(
                "By Language",
                "http://opml.radiotime.com/Browse.ashx?c=lang",
                "language",
                "",
            ),
            make_link(
                "Podcasts",
                "http://opml.radiotime.com/Browse.ashx?c=podcast",
                "podcast",
                "",
            ),
        ],
    };
    assert_eq!(document, expected_document);
}

#[test]
fn sample_2() {
    let document = tunein::read(File::open("tests/documents/sample_2.opml").unwrap()).unwrap();
    let expected_document = tunein::Document {
        version: tunein::Version { major: 1, minor: 0 },
        head: tunein::Head {
            title: "Kraków".into(),
            status: Some(300),
        },
        outlines: vec![make_group(
            "Stacje",
            "stations",
            vec![
                make_audio(
                    "Anty Radio 101.3 (Rock)",
                    "Rockowo Bezkompromisowi",
                    "http://opml.radiotime.com/Tune.\
                                                   ashx?id=s76368",
                    96,
                    10,
                    tunein::Format::MP3,
                    "station",
                    "http://cdn-radiotime-logos.tunein.\
                                                   com/s9608q.png",
                    "s76368",
                    "g19",
                    "s76368",
                    "s76368",
                ),
                make_audio(
                    "KRK.FM 102.4 (Top 40-Pop)",
                    "Polska",
                    "http://opml.radiotime.com/Tune.\
                                                   ashx?id=s16527",
                    128,
                    10,
                    tunein::Format::MP3,
                    "station",
                    "http://cdn-radiotime-logos.tunein.\
                                                   com/s16527q.png",
                    "s16527",
                    "g61",
                    "s16527",
                    "s16527",
                ),
                make_audio(
                    "PR R Krakow Nowy Sacz 90.0 (Rock)",
                    "Polska",
                    "http://opml.radiotime.com/Tune.\
                                                   ashx?id=s103067",
                    32,
                    85,
                    tunein::Format::MP3,
                    "station",
                    "http://cdn-radiotime-logos.tunein.\
                                                   com/s103064q.png",
                    "s103067",
                    "g19",
                    "s103067",
                    "s103067",
                ),
                make_audio(
                    "PR R Krakow Tarnow 101.0 (Rock)",
                    "Polska",
                    "http://opml.radiotime.com/Tune.\
                                                   ashx?id=s103069",
                    32,
                    100,
                    tunein::Format::MP3,
                    "station",
                    "http://cdn-radiotime-logos.tunein.\
                                                   com/s103064q.png",
                    "s103069",
                    "g19",
                    "s103069",
                    "s103069",
                ),
            ],
        )],
    };
    assert_eq!(document, expected_document);
}

#[test]
fn sample_3() {
    let document = tunein::read(File::open("tests/documents/sample_3.opml").unwrap()).unwrap();
    let expected_document = tunein::Document {
        version: tunein::Version { major: 1, minor: 0 },
        head: tunein::Head {
            title: "Angola".into(),
            status: Some(200),
        },
        outlines: vec![
            make_group(
                "All Stations",
                "stations",
                vec![make_audio(
                    "L.A.C FM 95.5 (Portuguese Music)",
                    "Luanda Antena Comercial",
                    "http://opml.radiotime.com/Tune.ashx?id=s6228",
                    128,
                    91,
                    tunein::Format::MP3,
                    "station",
                    "http://cdn-radiotime-logos.tunein.com/s6228q.png",
                    "s6228",
                    "g158",
                    "s6228",
                    "s6228",
                )],
            ),
            make_link(
                "Luanda",
                "http://opml.radiotime.com/Browse.ashx?id=r100526",
                "",
                "r100526",
            ),
        ],
    };
    assert_eq!(document, expected_document);
}

#[test]
fn broken_1() {
    let input = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><opml version=\"1\"><head>".as_bytes();
    assert!(tunein::read(input).is_err());
}

#[test]
fn minimal_ok() {
    let input = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><opml version=\"1\"></opml>".as_bytes();
    assert!(tunein::read(input).is_ok());
}
