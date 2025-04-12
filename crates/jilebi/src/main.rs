use dosa::run_code;

fn main() {
    let code = r#"
	export function main(name) {
		console.log("main says = ", name);
		return "Hello dosa!";
	}
	
	"#;
    match run_code(String::from(code)) {
        Ok(result) => println!("Function output = {}", result),
        Err(err) => println!("Function failed with error {}", err),
    }
}
