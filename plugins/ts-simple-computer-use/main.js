export function get_files(request, env) {
	console.info("get_files called with args ", request, env);
	return {
		contents: [
			{
				uri: "file://man.js",
				text: "main.js"
			}
		]
	}
}

export function get_processes(request, env) {
	// console.log("get_processes called with args ", request, env);
	return {
		contents: [
			{
				uri: "ps://jilebi",
				text: "jilebi"
			},
			{
				uri: "ps://bash",
				text: "bash"
			}
		]
	}
}

export function create_new_directory({ name }, env) {
	let response = `create_new_directory called with args ${name}, env: ${env}`;
	return {
		content: [
			{ type: "text", text: response }
		]
	};
}