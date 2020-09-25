

use epub::doc::EpubDoc;
use std::path::Path;

#[test]
fn doc_open() {
    let doc = EpubDoc::new("test.epub");
    assert!(doc.is_ok());
    let doc = doc.unwrap();
    assert_eq!(Path::new("OEBPS"), doc.root_base);
    assert_eq!(Path::new("OEBPS/content.opf"), doc.root_file);

    assert_eq!(23, doc.resources.len());
    {
        let tpage = doc.resources.get("titlepage.xhtml");
        assert_eq!(tpage.unwrap().0, Path::new("OEBPS/Text/titlepage.xhtml"));
    }

    {
        assert_eq!(17, doc.spine.len());
        assert_eq!("titlepage.xhtml", doc.spine[0]);
    }

    {
        let title = doc.mdata("title");
        assert_eq!(title.unwrap(), "Todo es mío");
    }

    {
        let cover = doc.get_cover_id();
        assert_eq!(cover.unwrap(), "portada.png");
    }
}

#[test]
fn toc_test() {
    let doc = EpubDoc::new("test.epub");
    assert!(doc.is_ok());
    let doc = doc.unwrap();

    assert!(doc.toc.len() > 0);
    for nav in doc.toc.iter() {
        let chapter = doc.resource_uri_to_chapter(&nav.content);
        assert!(chapter.is_some());
        assert_eq!(nav.play_order, chapter.unwrap());
    }
}
