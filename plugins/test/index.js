export async function get_files(request, env) {
	console.info("get_files called with args ", request, env);
	let response = await fetch('https://jsonplaceholder.typicode.com/todos/1');
	let data = await response.json();
	console.log(data);
	console.info(env);
	// let file = Deno.readTextFileSync("/home/kartik/jilebi/plugins/ts-simple-computer-use/manifest.toml");
	// console.log("allowed file read", file);
	return {
		contents: [
			{
				uri: "file://man.js",
				text: "main.js"
			}
		]
	}
}

export async function get_processes(request, env) {
	console.log("get_processes called with args ", request, env);
	let file = Deno.readTextFileSync("/home/kartik/jilebi/plugins/ts-simple-computer-use/test.txt");
	console.log("allowed file read", file);
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