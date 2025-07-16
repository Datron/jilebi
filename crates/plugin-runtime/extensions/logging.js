function argsToMessage(args) {
	return args.map((arg) => JSON.stringify(arg)).join(" ");
}


globalThis.console = {
	log: (...args) => {
		Deno.core.ops.op_trace(`[out]: ${argsToMessage(args)}\n`);
	},
	warn: (...args) => {
		Deno.core.ops.op_warn(`[warn]: ${argsToMessage(args)}\n`);
	},
	debug: (...args) => {
		Deno.core.ops.op_debug(`[debug]: ${argsToMessage(args)}\n`);
	},
	error: (...args) => {
		Deno.core.ops.op_error(`[error]: ${argsToMessage(args)}\n`);
	},
	info: (...args) => {
		Deno.core.ops.op_info(`[info]: ${argsToMessage(args)}\n`);
	},
}