use deno_core::{extension, op2};

#[op2]
#[string]
fn html2markdown(#[string] html: &str) -> String {
    html2md::parse_html(html)
}

extension!(
    html2markdown,
    ops = [html2markdown],
    esm_entry_point = "ext:html2markdown/html2markdown.js",
    esm = [ dir "extensions", "html2markdown.js" ],
);