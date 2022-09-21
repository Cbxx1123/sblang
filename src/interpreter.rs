use std::collections::HashMap;
use logos::{*};
#[derive(Debug,Clone)]
pub enum Command{
    OpPush(i32),
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpDump,
    OpIdent(String),
    OpEnd,
    OpDel(String),
    OpEq,
    OpNeq,
    OpGt,
    OpLt,
    OpGte,
    OpLte,
    CfIf(Vec<Command>),
    CfElse(Vec<Command>),
    CfDFn(String,Vec<String>,Vec<Command>),
    CfCall(String),
    CfFor(String,Option<i32>,Vec<Command>),
}

pub struct Interpreter{
    stack: Vec<i32>,
    heap: HashMap<String, i32>,
    fn_stack: HashMap<String,(HashMap<String,i32>,Vec<Command>)>,
}
impl Interpreter{
    pub fn new() -> Interpreter{
        Interpreter{
            stack: Vec::new(),
            heap: HashMap::new(),
            fn_stack: HashMap::new(),
        }
    }
    pub fn run(&mut self,program: Vec<Command>){
        for command in program{
            match command{
                Command::OpPush(val) => self.stack.push(val),
                Command::OpAdd => {
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("Stack underflow");
                    self.stack.push(a + b);
                },
                Command::OpSub => {
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("Stack underflow");
                    self.stack.push(b-a);
                },
                Command::OpMul => {
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("Stack underflow");
                    self.stack.push(a * b);
                },
                Command::OpDiv => {
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("stack underflow");
                    self.stack.push(b / a);
                },
                Command::OpMod => {
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("Stack underflow");
                    self.stack.push(b%a);
                },
                Command::OpDump => {
                    println!("{}", self.stack.pop().expect("No value on stack to print"));
                },
                Command::OpIdent(ident) => {
                    if self.heap.contains_key(&ident){
                        self.stack.push(self.heap[&ident]);
                    }else{
                        let value = self.stack.pop().expect(&("No value to assign to variable: ".to_owned()+ &ident));
                        self.heap.insert(ident.clone(), value);
                    }
                },
                Command::OpEnd => {
                    self.stack.clear();
                },
                Command::OpDel(ident) => {
                    self.heap.remove(&ident);
                },
                Command::OpEq => {
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("Stack underflow");
                    self.stack.push((a == b) as i32);
                },
                Command::OpNeq =>{
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("Stack underflow");
                    self.stack.push((a != b) as i32);
                },
                Command::OpGt => {
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("Stack underflow");
                    self.stack.push((a < b) as i32);
                },
                Command::OpLt =>{
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("Stack underflow");
                    self.stack.push((a >b) as i32);
                },
                Command::CfIf(p) => {
                    let condition = self.stack.pop().expect("Stack underflow");
                    if condition == 1{
                        self.run(p);
                        self.stack.clear();
                        self.stack.push(1);
                    }
                    else{
                        self.stack.push(0);
                    }

                },
                Command::CfElse(p) => {
                    let condition = self.stack.pop().expect("Stack underflow");
                    if condition == 0{
                        self.run(p);
                        self.stack.clear();
                    }
                },
                Command::CfDFn(ident,args,p ) => {
                    let mut new_heap = HashMap::new();
                    for arg in args{
                        new_heap.insert(arg, 0);
                    }
                    self.fn_stack.insert(ident, (new_heap,p));
                },
                Command::CfCall(ident) => {
                    let ident = ident[0..ident.len()-2].to_string();
                    if self.fn_stack.contains_key(&ident){
                        for (k,_v) in &self.fn_stack[&ident].0{
                            self.heap.insert(k.to_string(),self.stack.pop().expect("Stack underflow"));
                        }
                        self.run(self.fn_stack[&ident].1.clone());
                    }
                    else {
                        println!("Function {} not found", ident);
                    }
                },
                Command::OpGte => {
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("Stack underflow");
                    self.stack.push((b <= a) as i32);
                },
                Command::OpLte => {
                    let a = self.stack.pop().expect("Stack underflow");
                    let b = self.stack.pop().expect("Stack underflow");
                    self.stack.push((b >= a) as i32);
                },
                Command::CfFor(var,x, p) => {
                    if x.is_some(){
                        for i in 0..x.unwrap(){
                            self.heap.insert(var.clone(), i);
                            self.run(p.clone());
                        }
                    }
                    else{
                        let x = self.stack.pop().expect("Stack underflow");
                        for i in 0..x{
                            self.heap.insert(var.clone(), i);
                            self.run(p.clone());
                        }
                    }
                },
            }
        }
    }
}








#[derive(Logos, Debug, PartialEq,Clone)]
enum Token{
    #[regex(r"[0-9]+")]
    Number,
    #[regex(r"[a-zA-Z]+")]
    Ident,
    #[regex(r"\+")]
    Add,
    #[regex(r"-")]
    Sub,
    #[regex(r"\*")]
    Mul,
    #[regex(r"/")]
    Div,
    #[regex(r"%")]
    Mod,
    #[regex(r";")]
    Semicolon,
    #[regex(r"\$")]
    Dump,
    #[token(r"del")]
    Del,
    #[token(r"==")]
    Eq,
    #[token(r"!=")]
    Neq,
    #[token(r">")]
    Gt,
    #[token(r"<")]
    Lt,
    #[token(r">=")]
    Gte,
    #[token(r"<=")]
    Lte,
    #[token(r"if")]
    If,
    #[token(r"else")]
    Else,
    #[token(r"end")]
    End,
    #[token(r"fn")]
    Fn,
    #[regex(r"[a-zA-Z]+\(\)")]
    Call,
    #[token(r"do")]
    Do,
    #[token(r"for")]
    For,
    #[token(r"in")]
    In,
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}


fn match_tokens(tokens: &mut logos::Lexer<Token>) -> Vec<Command>{
    let mut commands = Vec::new();
    //For each token match it to a command and push it to the vector
    while let Some(token) = tokens.next(){
        match token{
            Token::Number => {
                let value = tokens.slice().parse::<i32>().unwrap();
                commands.push(Command::OpPush(value));
            },
            Token::Ident => {
                let ident = String::from(tokens.slice());
                commands.push(Command::OpIdent(ident));
            },
            Token::Add => commands.push(Command::OpAdd),
            Token::Sub => commands.push(Command::OpSub),
            Token::Mul => commands.push(Command::OpMul),
            Token::Div => commands.push(Command::OpDiv),
            Token::Mod => commands.push(Command::OpMod),
            Token::Semicolon => commands.push(Command::OpEnd),
            Token::Dump => commands.push(Command::OpDump),
            Token::Error => panic!("Error: {}", tokens.slice()),
            Token::Del => {
                //Get the next token and if it is an ident delete it from the heap
                let token = tokens.next().expect("Expected identifier after del");
                if token == Token::Ident{
                    let ident = String::from(tokens.slice());
                    commands.push(Command::OpDel(ident));
                }
            },
            Token::Eq => commands.push(Command::OpEq),
            Token::Neq => commands.push(Command::OpNeq),
            Token::Gt => commands.push(Command::OpGt),
            Token::Lt => commands.push(Command::OpLt),
            Token::If => {
                let mut str = String::new(); 
                while let Some(token) = tokens.next(){
                    if token == Token::End{
                        break;
                    }
                    str.push_str(&(tokens.slice().to_owned()+ " "));
                }
                let mut lexer = Token::lexer(str.as_str());
                let p = match_tokens(&mut lexer);
                commands.push(Command::CfIf(p));
            },
            Token::End =>{},
            Token::Else => {
                let mut str = String::new(); 
                while let Some(token) = tokens.next(){
                    if token == Token::End{
                        break;
                    }
                    str.push_str(&(tokens.slice().to_owned()+ " "));
                }
                let mut lexer = Token::lexer(str.as_str());
                let p = match_tokens(&mut lexer);
                commands.push(Command::CfElse(p));
            },
            Token::Fn => {
                let token = tokens.next().expect("Expected identifier after fn");
                if token == Token::Ident{
                    let ident = String::from(tokens.slice());
                    let mut a = vec![];
                    while let Some(token) = tokens.next(){
                        if token == Token::Do{
                            break;
                        }
                        if token == Token::Ident{
                            a.push(String::from(tokens.slice()));
                        }
                    }
                    let mut str = String::new(); 
                    while let Some(token) = tokens.next(){
                        if token == Token::End{
                            break;
                        }
                        str.push_str(&(tokens.slice().to_owned()+ " "));
                    }
                    let mut lexer = Token::lexer(str.as_str());
                    let p = match_tokens(&mut lexer);
                    commands.push(Command::CfDFn(ident,a, p));
                }
            },
            Token::Call => {
                let ident = String::from(tokens.slice());
                commands.push(Command::CfCall(ident));
            },
            Token::Gte => {
                commands.push(Command::OpGte);
            },
            Token::Lte => {
                commands.push(Command::OpLte);
            },
            Token::Do => {},
            Token::For => {
                let token = tokens.next().expect("Expected identifier after for");
                if token == Token::Ident{
                    let ident = String::from(tokens.slice());
                    let token = tokens.next().expect("Expected in after identifier");
                    if token == Token::In{
                        let token = tokens.next().expect("Expected number after in");
                        if token == Token::Number{
                            let x = tokens.slice().parse::<i32>().unwrap();
                            let mut str = String::new(); 
                            while let Some(token) = tokens.next(){
                                if token == Token::End{
                                    break;
                                }
                                str.push_str(&(tokens.slice().to_owned()+ " "));
                            }
                            let mut lexer = Token::lexer(str.as_str());
                            let p = match_tokens(&mut lexer);
                            commands.push(Command::CfFor(ident.clone(),Some(x), p));
                        }
                        if token == Token::Ident{
                            commands.push(Command::OpIdent(String::from(tokens.slice())));
                            
                            let mut str = String::new(); 
                            while let Some(token) = tokens.next(){
                                if token == Token::End{
                                    break;
                                }
                                str.push_str(&(tokens.slice().to_owned()+ " "));
                            }
                            let mut lexer = Token::lexer(str.as_str());
                            let p = match_tokens(&mut lexer);
                            commands.push(Command::CfFor(ident, None, p));
                        }
                    }
                }
            },
            Token::In => {},
        }
    }
    return commands;

    }

pub struct Lexer();
impl Lexer{
    pub fn lex_file(file_path: String) -> Vec<Command>{
        let file = std::fs::read_to_string(file_path).unwrap();
        let mut lexer = Token::lexer(&file);
        
        let mut commands = match_tokens(&mut lexer);
        commands.push(Command::CfCall(String::from("main()")));

        return commands;
    }
}