globalThis.html2markdown = (html) => {
  return Deno.core.ops.html2markdown(html);
}
