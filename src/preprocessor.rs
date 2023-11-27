use mdbook::errors::Error;
use mdbook::book::Book;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{Event, Parser, CowStr};
use regex::Regex;
use crate::types::State;

/// A image-size preprocessor.
#[derive(Default)]
pub struct ImageSize;

impl ImageSize {
    pub fn new() -> ImageSize {
        ImageSize
    }
}

impl Preprocessor for ImageSize {
    fn name(&self) -> &str {
        "image-size-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        let re = Regex::new(r#"\((?<url>\S+)\s+("(?<title>.+)"\s+)?=(?<width>\d+)?x(?<height>\d+)?\s*\)"#).unwrap();
        book.for_each_mut(|book| {
            if let mdbook::BookItem::Chapter(chapter) = book {
                let mut image_blocks = vec![];
                let events = Parser::new(&chapter.content);
                let mut state = State::None;
                let mut span_start = 0;
                let mut alt = "";
                for (event, span) in events.into_offset_iter() {
                    match event {
                        Event::Text(CowStr::Borrowed(text)) if text == "![" => match state  {
                            State::None => {
                                state = State::Start;
                                span_start = span.start;
                            },
                            _ => {
                                alt = "";
                                state = State::None;
                                continue;
                            }
                        },
                        Event::Text(CowStr::Borrowed(text)) if text == "]" => match state  {
                            State:: Start => state = State::AltClose,
                            _ => {
                                alt = "";
                                state = State::None;
                                continue;
                            }
                        },
                        Event::Text(CowStr::Borrowed(text)) => match state {
                            State::Start => alt = text,
                            State::AltClose => {
                                let Some(caps) = re.captures(text) else {
                                    alt = "";
                                    state = State::None;
                                    continue;
                                };
                                
                                let alt_attr = if alt == "" {
                                    String::from("")
                                } else {
                                    format!(" alt=\"{}\"", alt)
                                };
                                
                                let title = caps.name("title").map_or("", |m| m.as_str());
                                let title_attr = if title == "" {
                                    String::from("")
                                } else {
                                    format!(" title=\"{}\"", title)
                                };

                                let width = caps.name("width").map_or("", |m| m.as_str());
                                let height = caps.name("height").map_or("", |m| m.as_str());
                                
                                if width == "" && height == "" {
                                    alt = "";
                                    state = State::None;
                                    continue;
                                }

                                let width_attr = if width == "" {
                                    String::from("")
                                } else {
                                    format!(" width=\"{}\"", width)
                                };

                                let height_attr = if height == "" {
                                    String::from("")
                                } else {
                                    format!(" height=\"{}\"", height)
                                };

                                let img = format!("<img src=\"{}\"{}{}{}{}>",
                                    &caps["url"],
                                    alt_attr.as_str(),
                                    title_attr.as_str(),
                                    width_attr.as_str(),
                                    height_attr.as_str());
                                
                                image_blocks.push((span_start..span.start+&caps[0].len(), img));
                                alt = "";
                                state = State::None;
                                continue;
                            }
                            _ => continue

                        },
                        _ => {}
                    }
                }
                
                for(span, img) in image_blocks.iter().rev() {
                    let pre_content = &chapter.content[..span.start];
                    let post_content = &chapter.content[span.end..];
                    chapter.content = format!("{}{}{}", pre_content, img, post_content);
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn image_size_run() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "nop-preprocessor": {"blow-up": "a"}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n// \nfoo ![](images/2023-11-25-11-46-17.png =800x400) bar",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let expected_book = book.clone();
        let result = ImageSize::new().run(&ctx, book);
        assert!(result.is_ok());

        // The nop-preprocessor should not have made any changes to the book content.
        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }
}
