use serde::Serialize;
use std::io::Write;

use crate::documents::*;
use crate::xml_builder::*;
use footnote_id::generate_footnote_id;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Footnote {
    pub id: usize,
    pub content: Vec<Paragraph>,
}

impl Default for Footnote {
    fn default() -> Self {
        Footnote {
            id: 1,
            content: vec![],
        }
    }
}

impl Footnote {
    pub fn new() -> Self {
        Self {
            id: generate_footnote_id(),
            ..Default::default()
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn add_content(&mut self, p: Paragraph) -> Self {
        self.content.push(p);
        self.clone()
    }
}
impl From<&FootnoteReference> for Footnote {
    fn from(reference: &FootnoteReference) -> Self {
        Footnote {
            id: reference.id,
            content: reference.content.clone(),
        }
    }
}

impl BuildXML for Footnote {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        // To ensure docx compatible XML serialization for footnotes, we default to an empty paragraph.
        let mut footnote = self.clone();
        if self.content == vec![] {
            eprintln!("{:?}", footnote);
            footnote.add_content(Paragraph::new());
        }

        XMLBuilder::from(stream)
            .open_footnote(&format!("{}", self.id))?
            .add_children(&footnote.content)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_footnote_build_default() {
        let b = Footnote::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:footnote w:id="1"><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr></w:p></w:footnote>"#
        );
    }

    #[test]
    fn test_footnote_build_with_paragraph() {
        let b = Footnote::new()
            .add_content(Paragraph::new().add_run(Run::new().add_text("hello")))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:footnote w:id="1"><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">hello</w:t></w:r></w:p></w:footnote>"#
        );
    }
}
