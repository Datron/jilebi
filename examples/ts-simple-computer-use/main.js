export function get_files(request, env) {
	console.log("get_files called with args ", request, env);
	return ["main.js", "index.js"];
}

export function get_processes(request, env) {
	console.log("get_processes called with args ", request, env);
	return ["jilebi", "bash"];
}

export function create_new_directory(request, env) {
	console.log("create_new_directory called with args ", request, env);
	return true;
}