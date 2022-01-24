use poetic::instruction::Instruction;

pub struct Parser {}

impl Parser {
    pub fn parse_string(source: &str) -> Vec<char> {
        source
            .chars()
            .filter(|c| {
                c == &'>'
                    || c == &'<'
                    || c == &'+'
                    || c == &'-'
                    || c == &'.'
                    || c == &','
                    || c == &'['
                    || c == &']'
            })
            .collect()
    }

    fn opcode_to_instruction(opcode: &char) -> Instruction {
        match opcode {
            '>' => Instruction::FWD(1),
            '<' => Instruction::BAK(1),
            '+' => Instruction::INC(1),
            '-' => Instruction::DEC(1),
            '.' => Instruction::OUT,
            ',' => Instruction::IN,
            '[' => Instruction::IF,
            ']' => Instruction::EIF,
            _ => panic!("Unknown instruction {}", opcode),
        }
    }

    pub fn parse_instructions(source: &[char]) -> Vec<Instruction> {
        source.iter().map(Parser::opcode_to_instruction).collect()
    }
}
