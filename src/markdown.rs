use std::borrow::Cow;

use mdbook::errors::Result as MdbookResult;
use pulldown_cmark::{Event, Parser, CowStr};
use crate::types::State;
use regex::{Regex, Captures};
use once_cell::sync::Lazy;

static RE_IMAGE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\((?<url>\S+)\s+("(?<title>.+)"\s+)?=(?<width>\d+)?x(?<height>\d+)?\s*\)"#).unwrap()
});

pub fn preprocess(content: &str) -> MdbookResult<String> {
    let mut image_blocks = vec![];
    let mut alt = "";
    let mut state = State::None;
    let mut span_start = 0;

    let text_events = Parser::new(content).into_offset_iter()
        .filter_map(|(event, span)| match event {
            Event::Text(CowStr::Borrowed(text)) => Some((text, span)),
            _ => None,
        });

    for (text, span) in text_events {
        if text == "![" {
            if let State::None = state {
                state = State::Start;
                span_start = span.start;
            } else {
                alt = "";
                state = State::Start;
                continue;
            }
        }
        else if text == "]" {
            if let State::Start = state {
                state = State::AltClose;
            } else {
                alt = "";
                state = State::None;
                continue;
            }
        } else if let State::Start = state {
            alt = text;
        } else if let State::AltClose = state {
            let Some(caps) = RE_IMAGE.captures(text) else {
                alt = "";
                state = State::None;
                continue;
            };
            
            let alt_attr = get_attr(alt, "alt");
            
            let (_, title_attr) = get_name_attr(&caps, "title");
            let (width, width_attr) = get_name_attr(&caps, "width");
            let (height, height_attr) = get_name_attr(&caps, "height");

            if width == "" && height == "" {
                alt = "";
                state = State::None;
                continue;
            }

            let img = format!("<img src=\"{}\"{}{}{}{}>",
                &caps["url"],
                alt_attr,
                title_attr,
                width_attr,
                height_attr);
            
            image_blocks.push((span_start..span.start+&caps[0].len(), img));
            alt = "";
            state = State::None;
            continue;
        }
    }
    
    let mut content = content.to_string();
    for(span, img) in image_blocks.iter().rev() {
        let pre_content = &content[..span.start];
        let post_content = &content[span.end..];
        content = format!("{}{}{}", pre_content, img, post_content);
    }

    Ok(content)
}

fn get_attr<'a>(attr: &str, name: &str) -> Cow<'a, str> {
    match attr {
        "" => "".into(),
        _ => format!(" {}=\"{}\"", name, attr).into(),
    }
}

fn get_name_attr<'a>(caps: &'a Captures, name: &'a str) 
    -> (&'a str, Cow<'a, str>) 
{
    let attr_v= caps.name(name).map_or("", |m| m.as_str());
    (attr_v, get_attr(attr_v, name))
}

#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    fn preprocess_width_height() {
        let content = "# Chapter 1\n// \nfoo ![](images/2023-11-25-11-46-17.png =800x400) bar";
        let result = preprocess(content).unwrap();
        assert_eq!(result, "# Chapter 1\n// \nfoo <img src=\"images/2023-11-25-11-46-17.png\" width=\"800\" height=\"400\"> bar");
    }

    #[test]
    fn preprocess_width() {
        let content = "# Chapter 1\n// \nfoo ![](images/2023-11-25-11-46-17.png =800x) bar";
        let result = preprocess(content).unwrap();
        assert_eq!(result, "# Chapter 1\n// \nfoo <img src=\"images/2023-11-25-11-46-17.png\" width=\"800\"> bar");
    }

    #[test]
    fn preprocess_height() {
        let content = "# Chapter 1\n// \nfoo ![](images/2023-11-25-11-46-17.png =x400) bar";
        let result = preprocess(content).unwrap();
        assert_eq!(result, "# Chapter 1\n// \nfoo <img src=\"images/2023-11-25-11-46-17.png\" height=\"400\"> bar");
    }
}
