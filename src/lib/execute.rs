use crate::fetch_and_decode::OPCODE;
use crate::cpu::Chip8;


pub fn exe(op_code: OPCODE, code: &[u8; 2], cpu: &mut Chip8) {
    match op_code {
        OPCODE::OP00E0 => {
            println!("clean the screen not implmented");
            cpu.next_op();
        },
        OPCODE::OP00EE => {  //return from subroutine
            let address = cpu.pop_from_stack();
            cpu.set_address(address);
            cpu.next_op();
        },
        OPCODE::OP1NNN => {
            let adress: u16 = (code[0] as u16 & 0x0F) << 8 | code[1] as u16;
            cpu.set_address(adress);
        },
        OPCODE::OP2NNN => {  //call subroutine
            cpu.push_to_stack(cpu.pc);
            let adress: u16 = (code[0] as u16 & 0x0F) << 8 | code[1] as u16;
            cpu.set_address(adress);
        },
        OPCODE::OP3XNN => {
            if cpu.V[(code[0] & 0x0F) as usize] == code[1]{
                cpu.next_op();
            }
                cpu.next_op();
        },
        OPCODE::OP4XNN => {
            if cpu.V[(code[0] & 0x0F) as usize] != code[1]{
                cpu.next_op();
            }
            cpu.next_op();
        },
        OPCODE::OP5XY0 => {
            if cpu.V[(code[0] & 0x0F) as usize] == cpu.V[(code[1] & 0xF0) as usize]{
                cpu.next_op();
            }
            cpu.next_op();
        },
        OPCODE::OP6XNN => {
            cpu.set_register((code[0] & 0x0F) as usize, code[1]);
            cpu.next_op();
        },
        OPCODE::OP7XNN => {
            let register_num =  code[0] & 0x0F;
            cpu.set_register((register_num) as usize,
                             cpu.V[register_num as usize] + code[1]);
            cpu.next_op();
        },
        OPCODE::OP8XY0 => {
            let rx = code[0] & 0x0F;
            let ry = code[1] >> 4;
            cpu.set_register(rx as usize, cpu.V[ry as usize]);
            cpu.next_op();
        },
        OPCODE::OP8XY1 => {
            let rx = code[0] & 0x0F;
            let ry = code[1] >> 4;
            let new_val = cpu.V[rx as usize] |  cpu.V[ry as usize];
            cpu.set_register(rx as usize, new_val);
            cpu.next_op();
        },
        OPCODE::OP8XY2 => {},
        OPCODE::OP8XY3 => {},
        OPCODE::OP8XY4 => {},
        OPCODE::OP8XY5 => {},
        OPCODE::OP8XY6 => {},
        OPCODE::OP8XY7 => {},
        OPCODE::OP8XYE => {},
        OPCODE::OP9XY0 => {},
        OPCODE::OPANNN => {},
        OPCODE::OPBNNN => {},
        OPCODE::OPCXNN => {},
        OPCODE::OPDXYN => {},
        OPCODE::OPEX9E => {},
        OPCODE::OPEXA1 => {},
        OPCODE::OPFX07 => {},
        OPCODE::OPFX0A => {},
        OPCODE::OPFX15 => {},
        OPCODE::OPFX18 => {},
        OPCODE::OPFX1E => {},
        OPCODE::OPFX29 => {},
        OPCODE::OPFX33 => {},
        OPCODE::OPFX55 => {},
        OPCODE::OPFX65 => {},
        OPCODE::None => {},
    }
}

mod test{
    use super::*;

    fn get_code(cpu: &mut Chip8) -> [u8; 2]{
        [cpu.memory[cpu.pc as usize], cpu.memory[cpu.pc as usize + 1]]
    }

    #[test]
    fn test_execute_2NNN(){
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_1NNN build infstructures failed");
        cpu.pc = 0x0210;
        let code : [u8; 2] = get_code(&mut cpu  );
        exe(OPCODE::OP2NNN, &code, &mut cpu);
        assert_eq!(cpu.pc, 0x2BE);
        assert!(cpu.stack[0] == 0x0210);
        assert!(cpu.sp == 1);
        cpu.pc = 0x02D0;
        let code : [u8; 2] = get_code(&mut cpu  );
        exe(OPCODE::OP00EE, &code, &mut cpu);
    }
    #[test]
    fn test_execute_1NNN(){
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_1NNN build infstructures failed");
        cpu.pc = 0x022E;
        let code : [u8; 2] = get_code(&mut cpu  );
        exe(OPCODE::OP1NNN, &code, &mut cpu);
        assert_eq!(cpu.pc, 0x234);
    }
    #[test]
    fn test_execute_00EE(){
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_1NNN build infstructures failed");
        cpu.pc = 0x0210;
        let code : [u8; 2] = get_code(&mut cpu  );
        exe(OPCODE::OP2NNN, &code, &mut cpu);
        cpu.pc = 0x02D0;
        let code : [u8; 2] = get_code(&mut cpu  );
        exe(OPCODE::OP00EE, &code, &mut cpu);
        assert_eq!(cpu.pc, 0x210 + 2);
        assert!(cpu.stack[0] == 0);
        assert!(cpu.sp == 0);
    }
    // add test to op3, op4, op5 , op6
    #[test]
    fn test_execute_6XNN(){
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_1NNN build infstructures failed");
        cpu.pc = 0x021E;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP6XNN, &code, &mut cpu);
        assert!(cpu.V[2] == 0x17);
        for i in 0..15{
            if i != 2{
                assert!(cpu.V[i as usize] == 0);
            }
        }
        assert!(cpu.pc == 0x0220);
    }
    #[test]
    fn test_execute_7XNN() {
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_1NNN build infstructures failed");
        cpu.pc = 0x0242;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP7XNN, &code, &mut cpu);
        assert!(cpu.V[3] == 0x06);
        assert!(cpu.pc == 0x0244);
    }
    #[test]
    fn test_execute_8XY0(){
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_1NNN build infstructures failed");

        cpu.set_register(0x0E, 23);
        cpu.set_register(0x04, 45);
        cpu.pc = 0x02BA;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY0, &code, &mut cpu);
        assert!(cpu.pc == 0x02BC);
        assert!(cpu.V[4] == 0x2D);
        assert!(cpu.V[0x0E] == 0x2D);
    }
    #[test]
    fn test_execute_8XY1(){
        let mut cpu = Chip8::new("chip_games/fishi.ch8").
            expect("test  test_execute_1NNN build infstructures failed");

        cpu.set_register(0x00, 23);
        cpu.set_register(0x0C, 45);
        cpu.pc = 0x024A;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY1, &code, &mut cpu);
        assert!(cpu.pc == 0x024C);
        assert!(cpu.V[0x00] == 23 | 45);
        assert!(cpu.V[0x0C] == 0x2D);
    }
}