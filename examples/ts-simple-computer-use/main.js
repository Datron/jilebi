export function get_files(request, env) {
	console.log("get_files called with args ", request, env);
	return ["main.js", "index.js"];
}

export function get_processes(request, env) {
	console.log("get_processes called with args ", request, env);
	return ["jilebi", "bash"];
}

export function create_new_directory({ name }, env) {
	let response = `create_new_directory called with args ${name}, env: ${env}`;
	return {
		content: [
			{ type: "text", text: response }
		]
	};
}