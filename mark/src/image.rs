use comrak::html::{ChildRendering, Context, dangerous_url};
use comrak::nodes::NodeLink;
use nom::branch::alt;
use nom::bytes::tag;

use nom::Parser;
use nom::character::complete::digit1;
use nom::combinator::{eof, map_res, opt};
use std::fmt;
use std::fmt::{Display, Write};

enum ImgDimension {
    Px(u32),
    Percent(u32),
}

impl Display for ImgDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Px(n) => write!(f, "{n}px"),
            Self::Percent(n) => write!(f, "{n}%"),
        }
    }
}

struct SithImg<'a> {
    url: &'a str,
    width: Option<ImgDimension>,
    height: Option<ImgDimension>,
}

struct ParseDimensionError;

/// Given a querystring that may contain image dimensions,
/// parse it and return the result
///
/// ## Note
///
/// If the dimension string is valid, it will always contain the image width.
/// However, the height is optional.
fn parse_dimensions(s: &str) -> Result<(ImgDimension, Option<ImgDimension>), ParseDimensionError> {
    fn parse_dim(dim: &str) -> Result<(&str, ImgDimension), ParseDimensionError> {
        let parsed = (
            map_res(digit1::<_, (_, _)>, str::parse),
            opt(alt((eof, tag("%"), tag("px")))),
        )
            .parse(dim);
        let Ok((remaining, (val, unit))) = parsed else {
            return Err(ParseDimensionError);
        };
        match unit {
            Some("%") => Ok((remaining, ImgDimension::Percent(val))),
            None | Some("px") | Some("") => Ok((remaining, ImgDimension::Px(val))),
            _ => Err(ParseDimensionError),
        }
    }
    let (remaining, width) = parse_dim(s)?;
    let height = if let Some(stripped) = remaining.strip_prefix("x") {
        Some(parse_dim(stripped)?.1)
    } else {
        None
    };

    Ok((width, height))
}

impl<'a> From<&'a str> for SithImg<'a> {
    fn from(s: &'a str) -> Self {
        // if the url contained dimension instructions, remove the query part
        // else, leave the url untouched
        if let Some((url, query)) = s.rsplit_once('?')
            && let Ok((width, height)) = parse_dimensions(query)
        {
            Self {
                url,
                width: Some(width),
                height,
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
