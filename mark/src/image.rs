use comrak::html::{ChildRendering, Context, dangerous_url};
use comrak::nodes::NodeLink;
use std::fmt;
use std::fmt::Write;
use std::sync::LazyLock;
use std::time::Instant;
use regex::Regex;

const DIMENSION_PATTERN: &str = r"^(?P<width>\d+(%|px)?)(x(?P<height>\d+(%|px)?))?$";
static DIMENSION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(DIMENSION_PATTERN).unwrap());

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
        let start = Instant::now();
        let split = nl.url.rsplit_once('?');
        let dimensions = match split {
            Some((_, query)) => DIMENSION_RE.captures(query),
            None => None,
        };
        let dur = start.elapsed();
        println!("{dur:?} {}", nl.url);
        if context.options.render.figure_with_caption {
            context.write_str("<figure>")?;
        }
        context.write_str("<img src=\"")?;
        let url = match dimensions {
            // if the url contained dimension instructions, remove the query part
            Some(_) => split.unwrap().0,
            None => nl.url.as_str(), // else, leave the url untouched
        };
        if !dangerous_url(url) {
            context.escape_href(url)?;
        }
        if let Some(dimensions) = dimensions {
            context.write_str("\" style=\"")?;

            for dim in ["width", "height"] {
                if let Some(val) = dimensions.name(dim) {
                    context.write_str(dim)?;
                    context.write_char(':')?;
                    context.write_str(val.as_str())?;
                    if !val.as_str().ends_with('%') && !val.as_str().ends_with("px") {
                        context.write_str("px")?;
                    }
                    context.write_char(';')?;
                }
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
