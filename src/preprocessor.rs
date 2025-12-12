use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::errors::{Error, Result as MdbookResult};
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};

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
            if let BookItem::Chapter(chapter) = item {
                let _ = preprocess(&chapter.content).map(|c| {
                    chapter.content = c;
                });
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> MdbookResult<bool> {
        Ok(renderer != "not-supported")
    }
}
