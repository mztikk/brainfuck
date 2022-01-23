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

    pub fn parse_instructions(source: &[char]) -> Vec<Instruction> {
        let mut result = Vec::new();

        for opcode in source.iter() {
            match opcode {
                '>' => result.push(Instruction::FWD(1)),
                '<' => result.push(Instruction::BAK(1)),
                '+' => result.push(Instruction::INC(1)),
                '-' => result.push(Instruction::DEC(1)),
                '.' => result.push(Instruction::OUT),
                ',' => result.push(Instruction::IN),
                '[' => result.push(Instruction::IF),
                ']' => result.push(Instruction::EIF),
                _ => panic!("Unknown instruction {}", opcode),
            }
        }

        result
    }
}
