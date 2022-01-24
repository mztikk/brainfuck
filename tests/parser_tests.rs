use brainfuck::{self, parser::Parser};
use poetic::instruction::Instruction;

#[test]
fn test_parse_fwd() {
    let source = ">";
    let instructions = Parser::parse_instructions(&Parser::parse_string(source));
    assert_eq!(instructions, vec![Instruction::FWD(1)]);
}

#[test]
fn test_parse_bak() {
    let source = "<";
    let instructions = Parser::parse_instructions(&Parser::parse_string(source));
    assert_eq!(instructions, vec![Instruction::BAK(1)]);
}

#[test]
fn test_parse_inc() {
    let source = "+";
    let instructions = Parser::parse_instructions(&Parser::parse_string(source));
    assert_eq!(instructions, vec![Instruction::INC(1)]);
}

#[test]
fn test_parse_dec() {
    let source = "-";
    let instructions = Parser::parse_instructions(&Parser::parse_string(source));
    assert_eq!(instructions, vec![Instruction::DEC(1)]);
}

#[test]
fn test_parse_out() {
    let source = ".";
    let instructions = Parser::parse_instructions(&Parser::parse_string(source));
    assert_eq!(instructions, vec![Instruction::OUT]);
}

#[test]
fn test_parse_in() {
    let source = ",";
    let instructions = Parser::parse_instructions(&Parser::parse_string(source));
    assert_eq!(instructions, vec![Instruction::IN]);
}

#[test]
fn test_parse_if_open() {
    let source = "[";
    let instructions = Parser::parse_instructions(&Parser::parse_string(source));
    assert_eq!(instructions, vec![Instruction::IF]);
}

#[test]
fn test_parse_if_close() {
    let source = "]";
    let instructions = Parser::parse_instructions(&Parser::parse_string(source));
    assert_eq!(instructions, vec![Instruction::EIF]);
}

#[test]
fn test_parse_all() {
    let source = ">+<-.,[]";
    let instructions = Parser::parse_instructions(&Parser::parse_string(source));
    assert_eq!(
        instructions,
        vec![
            Instruction::FWD(1),
            Instruction::INC(1),
            Instruction::BAK(1),
            Instruction::DEC(1),
            Instruction::OUT,
            Instruction::IN,
            Instruction::IF,
            Instruction::EIF,
        ]
    );
}
