use comrak::html::{ChildRendering, Context, dangerous_url};
use comrak::nodes::{NodeLink, NodeValue};
use comrak::options::{Extension, Parse, Render};
use comrak::{Arena, Options, create_formatter, parse_document};
use regex::Regex;
use std::fmt;
use std::fmt::Write;
use std::sync::LazyLock;

const DIMENSION_PATTERN: &str = r"^(?P<width>\d+(%|px)?)(x(?P<height>\d+(%|px)?))?$";
static DIMENSION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(DIMENSION_PATTERN).unwrap());

#[inline(always)]
fn options<'a>() -> Options<'a> {
    Options {
        extension: Extension {
            strikethrough: true,
            tagfilter: true,
            table: true,
            autolink: true,
            superscript: true,
            footnotes: true,
            description_lists: true,
            multiline_block_quotes: true,
            math_dollars: true,
            math_code: true,
            shortcodes: true,
            underline: true,
            subscript: true,
            spoiler: true,
            greentext: true,
            insert: true,
            ..Default::default()
        },
        parse: Parse {
            smart: true,
            ..Default::default()
        },
        render: Render {
            ..Default::default()
        },
    }
}

/// Render an image, with eventual size modifiers.
///
/// This is basically a copy-paste of the comrack `render_image` function,
/// with some code added
fn render_image<T>(
    context: &mut Context<T>,
    entering: bool,
    nl: &NodeLink,
) -> Result<ChildRendering, fmt::Error> {
    if entering {
        if context.options.render.figure_with_caption {
            context.write_str("<figure>")?;
        }
        context.write_str("<img src=\"")?;
        let url = nl.url.split('?').next().unwrap();
        if context.options.render.r#unsafe || !dangerous_url(url) {
            if let Some(rewriter) = &context.options.extension.image_url_rewriter {
                context.escape_href(&rewriter.to_html(&nl.url))?;
            } else {
                context.escape_href(url)?;
            }
        }
        context.write_str("\" alt=\"")?;
        return Ok(ChildRendering::Plain);
    } else {
        if !nl.title.is_empty() {
            context.write_str("\" title=\"")?;
            context.escape(&nl.title)?;
        }
        if let Some((_url, query)) = nl.url.rsplit_once('?')
            && let Some(caps) = DIMENSION_RE.captures(query)
        {
            context.write_str("\" style=\"")?;

            for dim in ["width", "height"] {
                if let Some(val) = caps.name(dim) {
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

create_formatter!(CustomFormatter, {
    NodeValue::Image(ref nl) => |context, entering| {
        return render_image(context, entering, nl);
    },
});

/// The aemark parser and html generator.
pub fn markdown(s: &str) -> String {
    let arena = Arena::new();
    let options = options();
    let root = parse_document(&arena, s, &options);

    for node in root.descendants() {
        if let NodeValue::Link(ref mut link) = node.data.borrow_mut().value
            && link.url.starts_with("page://")
        {
            link.url.replace_range(.."page://".len(), "/page/");
        }
    }

    let mut html = String::new();
    CustomFormatter::format_document(root, &options, &mut html).unwrap();
    html
}
