#[derive(Debug)]
pub struct Instruction {
  pub opcode: u8,
  pub operand1: u8,
  pub operand2: u8,
}

pub fn disassemble(index: usize, rom: [u8; 65535]) -> (Instruction, u8) {

    match rom[index] {
      0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x36 | 0x3e | 0xc6 | 0xce | 0xd6 | 0xdb | 0xde | 0xe6 | 0xee | 0xf6 | 0xfe => {
        return (Instruction {
          opcode: rom[index],
          operand1: rom[index + 1],
          operand2: 0x00,
        }, 2);
      }
      0x01 | 0x11 | 0x21 | 0x22 | 0x2a | 0x31 | 0x32 | 0x3a | 0xc2 | 0xc3 | 0xc4 | 0xca | 0xcc | 0xcd | 0xd2 | 0xd4 | 0xda | 0xdc | 0xe2 | 0xe4 | 0xea | 0xec | 0xf2 | 0xf4 | 0xfa | 0xfc => {
        return (Instruction {
          opcode: rom[index],
          operand1: rom[index + 1],
          operand2: rom[index + 2],
        }, 3);
      }
      _ => {
        return (Instruction{
          opcode: rom[index],
          operand1: 0x00,
          operand2: 0x00,
        }, 1);
      }
    }
}

pub fn disassembler(rom: &Vec<u8>) -> Vec<Instruction> {
  let mut index = 0;
  let mut result: Vec<Instruction> = vec![];
  while index < rom.len() {
    match rom[index] {
      0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x36 | 0x3e | 0xc6 | 0xce | 0xd6 | 0xdb | 0xde | 0xe6 | 0xee | 0xf6 | 0xfe => {
        result.push(Instruction {
          opcode: rom[index],
          operand1: rom[index + 1],
          operand2: 0x00,
        });
        index += 2;
      }
      0x01 | 0x11 | 0x21 | 0x22 | 0x2a | 0x31 | 0x32 | 0x3a | 0xc2 | 0xc3 | 0xc4 | 0xca | 0xcc | 0xcd | 0xd2 | 0xd4 | 0xda | 0xdc | 0xe2 | 0xe4 | 0xea | 0xec | 0xf2 | 0xf4 | 0xfa | 0xfc => {
        result.push(Instruction {
          opcode: rom[index],
          operand1: rom[index + 1],
          operand2: rom[index + 2],
        });
        index += 3;
      }
      0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 | 0xcb | 0xd9 | 0xdd | 0xed | 0xfd => {
        index += 1;
      }
      _ => {
        let ins = Instruction{
          opcode: rom[index],
          operand1: 0x00,
          operand2: 0x00,
        };
        result.push(ins);
        index += 1;
      }
    }
  }
  return result;
}