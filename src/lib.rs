// toy-lang/src/lib.rs

mod lexer;
mod token;

use lexer::Scanner;
use token::Token;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // JavaScript console.log for debugging
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Helper macro for logging to JavaScript console
macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)));
}

#[wasm_bindgen]
pub struct Tokenizer {
    scanner: Scanner,
    tokens: Vec<JsValue>,
}

#[wasm_bindgen]
impl Tokenizer {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Self {
        console_log!("Creating new tokenizer with input: {}", input);
        Self {
            scanner: Scanner::new(input),
            tokens: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn tokenize(&mut self) -> Vec<JsValue> {
        // Convert tokens to JsValue for JavaScript
        let mut tokens = Vec::new();
        for token in &mut self.scanner {
            let token_js = token_to_js_value(&token);
            tokens.push(token_js);
        }
        self.tokens = tokens.clone();
        tokens
    }

    #[wasm_bindgen]
    pub fn get_token_count(&self) -> usize {
        self.tokens.len()
    }
}

// Convert Token to a JavaScript-friendly format
fn token_to_js_value(token: &Token) -> JsValue {
    use token::TokenType;

    // Create a JS object for the token
    let obj = js_sys::Object::new();

    // Set token span properties
    let span = js_sys::Object::new();
    js_sys::Reflect::set(
        &span,
        &"start".into(),
        &JsValue::from(token.token_span.start as u32),
    )
    .unwrap();
    js_sys::Reflect::set(
        &span,
        &"end".into(),
        &JsValue::from(token.token_span.end as u32),
    )
    .unwrap();
    js_sys::Reflect::set(
        &span,
        &"line".into(),
        &JsValue::from(token.token_span.line as u32),
    )
    .unwrap();
    js_sys::Reflect::set(
        &span,
        &"column".into(),
        &JsValue::from(token.token_span.column as u32),
    )
    .unwrap();

    js_sys::Reflect::set(&obj, &"span".into(), &span).unwrap();

    // Set the token type and value
    match &token.token_type {
        TokenType::Identifier(id) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"identifier".into()).unwrap();
            js_sys::Reflect::set(&obj, &"value".into(), &JsValue::from(&id.value)).unwrap();
        }
        TokenType::Literal(lit) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"literal".into()).unwrap();

            match lit {
                token::LiteralToken::Number(num) => {
                    js_sys::Reflect::set(&obj, &"literalType".into(), &"number".into()).unwrap();

                    match num {
                        token::NumberToken::SignedInteger(n) => {
                            js_sys::Reflect::set(&obj, &"value".into(), &JsValue::from(*n))
                                .unwrap();
                            js_sys::Reflect::set(&obj, &"numberType".into(), &"signed".into())
                                .unwrap();
                        }
                        token::NumberToken::UnsignedInteger(n) => {
                            js_sys::Reflect::set(&obj, &"value".into(), &JsValue::from(*n as f64))
                                .unwrap();
                            js_sys::Reflect::set(&obj, &"numberType".into(), &"unsigned".into())
                                .unwrap();
                        }
                        token::NumberToken::Float(n) => {
                            js_sys::Reflect::set(&obj, &"value".into(), &JsValue::from(*n))
                                .unwrap();
                            js_sys::Reflect::set(&obj, &"numberType".into(), &"float".into())
                                .unwrap();
                        }
                    }
                }
                token::LiteralToken::String(s) => {
                    js_sys::Reflect::set(&obj, &"literalType".into(), &"string".into()).unwrap();
                    js_sys::Reflect::set(&obj, &"value".into(), &JsValue::from(s)).unwrap();
                }
                token::LiteralToken::Boolean(b) => {
                    js_sys::Reflect::set(&obj, &"literalType".into(), &"boolean".into()).unwrap();
                    js_sys::Reflect::set(&obj, &"value".into(), &JsValue::from(*b)).unwrap();
                }
                token::LiteralToken::Null => {
                    js_sys::Reflect::set(&obj, &"literalType".into(), &"null".into()).unwrap();
                    js_sys::Reflect::set(&obj, &"value".into(), &JsValue::null()).unwrap();
                }
                token::LiteralToken::Undefined => {
                    js_sys::Reflect::set(&obj, &"literalType".into(), &"undefined".into()).unwrap();
                    js_sys::Reflect::set(&obj, &"value".into(), &JsValue::undefined()).unwrap();
                }
            }
        }
        TokenType::WhiteSpace(_) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"whitespace".into()).unwrap();
        }
        TokenType::Delimiter(delim) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"delimiter".into()).unwrap();

            let delim_str = match delim {
                token::DelimiterToken::SingleQuote => "singleQuote",
                token::DelimiterToken::DoubleQuote => "doubleQuote",
                token::DelimiterToken::BackTick => "backTick",
                token::DelimiterToken::OpenParenthesis => "openParenthesis",
                token::DelimiterToken::CloseParenthesis => "closeParenthesis",
                token::DelimiterToken::OpenBracket => "openBracket",
                token::DelimiterToken::CloseBracket => "closeBracket",
                token::DelimiterToken::OpenBrace => "openBrace",
                token::DelimiterToken::CloseBrace => "closeBrace",
                token::DelimiterToken::EOF => "eof",
            };

            js_sys::Reflect::set(&obj, &"delimiterType".into(), &JsValue::from(delim_str)).unwrap();
        }
        TokenType::Punctuation(punct) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"punctuation".into()).unwrap();

            let punct_str = match punct {
                token::PunctuatorToken::Semicolon => "semicolon",
                token::PunctuatorToken::Comma => "comma",
                token::PunctuatorToken::Dot => "dot",
                token::PunctuatorToken::Colon => "colon",
            };

            js_sys::Reflect::set(&obj, &"punctuationType".into(), &JsValue::from(punct_str))
                .unwrap();
        }
        TokenType::Arithmetic(arith) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"arithmetic".into()).unwrap();

            let arith_str = match arith {
                token::ArithmeticToken::Add => "add",
                token::ArithmeticToken::Subtract => "subtract",
                token::ArithmeticToken::Multiply => "multiply",
                token::ArithmeticToken::Divide => "divide",
                token::ArithmeticToken::BitwiseAnd => "bitwiseAnd",
                token::ArithmeticToken::BitwiseOr => "bitwiseOr",
                token::ArithmeticToken::Or => "or",
                token::ArithmeticToken::And => "and",
                token::ArithmeticToken::Modulo => "modulo",
            };

            js_sys::Reflect::set(&obj, &"arithmeticType".into(), &JsValue::from(arith_str))
                .unwrap();
        }
        TokenType::Comparison(comp) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"comparison".into()).unwrap();

            let comp_str = match comp {
                token::ComparisonToken::Equal => "equal",
                token::ComparisonToken::NotEqual => "notEqual",
                token::ComparisonToken::GreaterThan => "greaterThan",
                token::ComparisonToken::GreaterThanOrEqual => "greaterThanOrEqual",
                token::ComparisonToken::LessThan => "lessThan",
                token::ComparisonToken::LessThanOrEqual => "lessThanOrEqual",
                token::ComparisonToken::Not => "not",
            };

            js_sys::Reflect::set(&obj, &"comparisonType".into(), &JsValue::from(comp_str)).unwrap();
        }
        TokenType::Assignment(assign) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"assignment".into()).unwrap();

            let assign_str = match assign {
                token::AssignmentToken::Assign => "assign",
                token::AssignmentToken::PlusAssign => "plusAssign",
                token::AssignmentToken::MinusAssign => "minusAssign",
                token::AssignmentToken::MultiplyAssign => "multiplyAssign",
                token::AssignmentToken::DivideAssign => "divideAssign",
                token::AssignmentToken::BitwiseAndAssign => "bitwiseAndAssign",
                token::AssignmentToken::BitwiseOrAssign => "bitwiseOrAssign",
                token::AssignmentToken::OrAssign => "orAssign",
                token::AssignmentToken::AndAssign => "andAssign",
            };

            js_sys::Reflect::set(&obj, &"assignmentType".into(), &JsValue::from(assign_str))
                .unwrap();
        }
        TokenType::ControlFlow(cf) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"controlFlow".into()).unwrap();

            let cf_str = match cf {
                token::ControlFlowToken::If => "if",
                token::ControlFlowToken::Else => "else",
                token::ControlFlowToken::For => "for",
                token::ControlFlowToken::In => "in",
                token::ControlFlowToken::Has => "has",
                token::ControlFlowToken::Return => "return",
            };

            js_sys::Reflect::set(&obj, &"controlFlowType".into(), &JsValue::from(cf_str)).unwrap();
        }
        TokenType::Declaration(decl) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"declaration".into()).unwrap();

            let decl_str = match decl {
                token::DeclarationToken::Let => "let",
                token::DeclarationToken::Function => "function",
                token::DeclarationToken::Object => "object",
            };

            js_sys::Reflect::set(&obj, &"declarationType".into(), &JsValue::from(decl_str))
                .unwrap();
        }
        TokenType::ObjectReference(obj_ref) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"objectReference".into()).unwrap();

            let obj_ref_str = match obj_ref {
                token::ObjectReferenceToken::This => "this",
                token::ObjectReferenceToken::Super => "super",
                token::ObjectReferenceToken::New => "new",
            };

            js_sys::Reflect::set(
                &obj,
                &"objectReferenceType".into(),
                &JsValue::from(obj_ref_str),
            )
            .unwrap();
        }
        TokenType::Logical(logic) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"logical".into()).unwrap();

            let logic_str = match logic {
                token::LogicalToken::And => "and",
                token::LogicalToken::Or => "or",
                token::LogicalToken::Not => "not",
                token::LogicalToken::XOr => "xor",
                token::LogicalToken::XAnd => "xand",
            };

            js_sys::Reflect::set(&obj, &"logicalType".into(), &JsValue::from(logic_str)).unwrap();
        }
        TokenType::Unknown(c) => {
            js_sys::Reflect::set(&obj, &"type".into(), &"unknown".into()).unwrap();
            js_sys::Reflect::set(&obj, &"value".into(), &JsValue::from(c.to_string())).unwrap();
        }
    }

    obj.into()
}
