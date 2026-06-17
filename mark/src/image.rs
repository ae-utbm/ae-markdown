use crate::image::ImgDimension::{Percent, Px};
use comrak::html::{ChildRendering, Context, dangerous_url};
use comrak::nodes::NodeLink;
use regex::Regex;
use std::fmt;
use std::fmt::{Display, Write};
use std::sync::LazyLock;

const DIMENSION_PATTERN: &str = r"^(?P<width>\d+(%|px)?)(x(?P<height>\d+(%|px)?))?$";
static DIMENSION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(DIMENSION_PATTERN).unwrap());

enum ImgDimension<T: Display> {
    Px(T),
    Percent(T),
}

impl<'a> TryFrom<&'a str> for ImgDimension<&'a str> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if s.ends_with('%') {
            Ok(Percent(s.strip_suffix('%').unwrap()))
        } else if s.ends_with("px") {
            Ok(Px(s.strip_suffix("px").unwrap()))
        } else if s.chars().all(|c| c.is_ascii_digit()) {
            Ok(Px(s))
        } else {
            Err(())
        }
    }
}

impl<T: Display> Display for ImgDimension<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Px(n) => write!(f, "{n}px"),
            Percent(n) => write!(f, "{n}%"),
        }
    }
}

struct SithImg<'a> {
    url: &'a str,
    width: Option<ImgDimension<&'a str>>,
    height: Option<ImgDimension<&'a str>>,
}

impl<'a> From<&'a str> for SithImg<'a> {
    fn from(s: &'a str) -> Self {
        // if the url contained dimension instructions, remove the query part
        // else, leave the url untouched
        if let Some((url, query)) = s.rsplit_once('?')
            && let Some(dimensions) = DIMENSION_RE.captures(query)
        {
            Self {
                url,
                width: dimensions
                    .name("width")
                    .and_then(|i| i.as_str().try_into().ok()),
                height: dimensions
                    .name("height")
                    .and_then(|i| i.as_str().try_into().ok()),
            }
        } else {
            Self {
                url: s,
                width: None,
                height: None,
            }
        }
    }
}

/// Render an image, with eventual size modifiers.
///
/// This is basically a copy-paste of the comrack `render_image` function,
/// with some code added
pub(crate) fn render_image<T>(
    context: &mut Context<T>,
    entering: bool,
    nl: &NodeLink,
) -> Result<ChildRendering, fmt::Error> {
    if entering {
        let img_data = SithImg::from(nl.url.as_str());
        if context.options.render.figure_with_caption {
            context.write_str("<figure>")?;
        }
        context.write_str("<img src=\"")?;
        if !dangerous_url(img_data.url) {
            context.escape_href(img_data.url)?;
        }
        if img_data.width.is_some() || img_data.height.is_some() {
            context.write_str("\" style=\"")?;
            if let Some(width) = img_data.width {
                write!(context, "width:{}", width)?;
            }
            if let Some(height) = img_data.height {
                write!(context, ";height:{}", height)?;
            }
        }
        context.write_str("\" alt=\"")?;
        return Ok(ChildRendering::Plain);
    } else {
        if !nl.title.is_empty() {
            context.write_str("\" title=\"")?;
            context.escape(&nl.title)?;
        }
        context.write_str("\" />")?;
        if context.options.render.figure_with_caption {
            if !nl.title.is_empty() {
                context.write_str("<figcaption>")?;
                context.escape(&nl.title)?;
                context.write_str("</figcaption>")?;
            }
            context.write_str("</figure>")?;
        }
    }

    Ok(ChildRendering::HTML)
}
