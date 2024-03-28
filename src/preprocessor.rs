use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use crate::markdown::preprocess;

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
        book.for_each_mut(|item| {
            if let mdbook::BookItem::Chapter(chapter) = item {
                let _ = preprocess(&chapter.content).map(|c| {
                    chapter.content = c;
                });
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
