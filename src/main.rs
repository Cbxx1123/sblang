use interpreter::Lexer;

mod interpreter;
fn main(){
    let program = Lexer::lex_file("tests/program.txt".to_string());
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.run(program);
}