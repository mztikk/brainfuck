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
        let mut iter = source.iter().peekable();

        while let Some(opcode) = iter.next() {
            match opcode {
                '>' => {
                    let mut count = 1;
                    while iter.peek() == Some(&&'>') {
                        iter.next();
                        count += 1;
                    }

                    result.push(Instruction::FWD(count));
                }
                '<' => {
                    let mut count = 1;
                    while iter.peek() == Some(&&'<') {
                        iter.next();
                        count += 1;
                    }

                    result.push(Instruction::BAK(count));
                }
                '+' => {
                    let mut count = 1;
                    while iter.peek() == Some(&&'+') {
                        iter.next();
                        count += 1;
                    }

                    result.push(Instruction::INC(count))
                }
                '-' => {
                    let mut count = 1;
                    while iter.peek() == Some(&&'-') {
                        iter.next();
                        count += 1;
                    }

                    result.push(Instruction::DEC(count))
                }
                '.' => result.push(Instruction::OUT),
                ',' => result.push(Instruction::IN),
                '[' => result.push(Instruction::IF),
                ']' => result.push(Instruction::EIF),
                _ => panic!("Unknown instruction {}", opcode),
            }
        }

        // for opcode in iter {
        //     match opcode {
        //         '>' => result.push(Instruction::FWD(1)),
        //         '<' => result.push(Instruction::BAK(1)),
        //         '+' => {
        //             let mut count = 1;
        //             while iter.peek() == Some(&'+') {
        //                 iter.next();
        //                 count += 1;
        //             }
        //             result.push(Instruction::INC(count))
        //         },
        //         '-' => result.push(Instruction::DEC(1)),
        //         '.' => result.push(Instruction::OUT),
        //         ',' => result.push(Instruction::IN),
        //         '[' => result.push(Instruction::IF),
        //         ']' => result.push(Instruction::EIF),
        //         _ => panic!("Unknown instruction {}", opcode),
        //     }
        // }

        result
    }
}
