globalThis.setState = (env, key, value) => Deno.core.ops.set_state(env.id, key, JSON.stringify(value));
globalThis.getState = (env, key) => {
	let value = Deno.core.ops.get_state(env.id, key);
	return JSON.parse(value);
}
globalThis.deleteState = (env, key) => Deno.core.ops.delete_state(env.id, key);
