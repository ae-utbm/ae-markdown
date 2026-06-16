mod image;

use crate::image::render_image;
use comrak::nodes::NodeValue;
use comrak::options::{Extension, Parse, Render};
use comrak::{Arena, Options, create_formatter, parse_document};

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
            escape: true,
            ..Default::default()
        },
    }
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
