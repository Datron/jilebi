globalThis.console.log = (msg) => {
	Deno.core.ops.op_trace(msg);
};
globalThis.console.warn = (msg) => {
	Deno.core.ops.op_warn(msg);
};
globalThis.console.debug = (msg) => {
	Deno.core.ops.op_debug(msg);
};
globalThis.console.error = (msg) => {
	Deno.core.ops.op_error(msg);
};
globalThis.console.info = (msg) => {
    Deno.core.ops.op_info(msg);
};