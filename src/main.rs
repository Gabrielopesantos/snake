mod lexer;

use lexer::lex;

fn main() {
    let input = "
print(\"Hello world\");
print(\"\");
let x = 5* 3;
fn add_two(number: int) int {
    return number + 2;
}
print(add_two(x));
";
    match lex(input) {
        Ok(tokens) => {
            for token in &tokens {
                println!(
                    "Token: {:?}, Start: {}, End: {}",
                    token.token, token.start, token.end
                );
            }
        }
        Err(err) => {
            println!("Lexer error: {}", err);
        }
    }
}
