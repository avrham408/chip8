use crate::fetch_and_decode::OPCODE;
use crate::cpu::Chip8;
use rand::Rng;


type Code = [u8; 2];

pub fn exe(op_code: OPCODE, code: &Code, cpu: &mut Chip8) {
    match op_code {
        OPCODE::OP00E0 => {  // add support to scren
             println!("clean the screen not implmented");
            cpu.next_op();
        },
        OPCODE::OP00EE => {  //return from subroutine
            let address = cpu.pop_from_stack();
            cpu.set_address(address);
            cpu.next_op();
        },
        OPCODE::OP1NNN => {
            let address: u16 = (code[0] as u16 & 0x0F) << 8 | code[1] as u16;
            cpu.set_address(address);
        },
        OPCODE::OP2NNN => {  //call subroutine
            cpu.push_to_stack(cpu.pc);
            let address: u16 = (code[0] as u16 & 0x0F) << 8 | code[1] as u16;
            cpu.set_address(address);
        },
        OPCODE::OP3XNN => { // its's the opposite of programing if it true you skip not get inside
            if cpu.get_register(code[0] & 0x0F) == code[1]{
                cpu.next_op();
            }
                cpu.next_op();
        },
        OPCODE::OP4XNN => {
            if cpu.get_register(code[0] & 0x0F) != code[1]{
                cpu.next_op();
            }
            cpu.next_op();
        },
        OPCODE::OP5XY0 => {
            if cpu.get_register(code[0] & 0x0F) == cpu.get_register(code[1] >> 4){
                cpu.next_op();
            }
            cpu.next_op();
        },
        OPCODE::OP6XNN => {
            cpu.set_register(code[0] & 0x0F, code[1]);
            cpu.next_op();
        },
        OPCODE::OP7XNN => {
            let v_pointer =  code[0] & 0x0F;
            let rx_val = cpu.get_register(v_pointer);
            cpu.set_register(v_pointer, rx_val+ code[1]);
            cpu.next_op();
        },
        OPCODE::OP8XY0 => {
            let rx = code[0] & 0x0F;
            let ry = code[1] >> 4;
            let ry_val = cpu.get_register(ry);
            cpu.set_register(rx, ry_val);
            cpu.next_op();
        },
        OPCODE::OP8XY1 => {
            let rx = code[0] & 0x0F;
            let ry = code[1] >> 4;
            let new_val = cpu.get_register(rx) |  cpu.get_register(ry);
            cpu.set_register(rx, new_val);
            cpu.next_op();
        },
        OPCODE::OP8XY2 => {
            let rx = code[0] & 0x0F;
            let ry = code[1] >> 4;
            let new_val = cpu.get_register(rx) &  cpu.get_register(ry);
            cpu.set_register(rx, new_val);
            cpu.next_op();
        },
        OPCODE::OP8XY3 => {
            let rx = code[0] & 0x0F;
            let ry = code[1] >> 4;
            let new_val = cpu.get_register(rx) ^  cpu.get_register(ry);
            cpu.set_register(rx, new_val);
            cpu.next_op();
        },
        OPCODE::OP8XY4 => {
            let rx = code[0] & 0x0F;
            let ry = code[1] >> 4;
            let x_val = cpu.get_register(rx) as u16;
            let y_val = cpu.get_register(ry) as u16;

            if x_val > (0xFF - y_val){
                cpu.set_register(0x0F, 1);
                cpu.set_register(rx, (x_val + y_val - 256) as u8);
            }else {
                cpu.set_register(0x0F, 0);
                cpu.set_register(rx, (x_val + y_val) as u8);
            }
            cpu.next_op();
        },
        OPCODE::OP8XY5 => {
            let rx = code[0] & 0x0F;
            let ry = code[1] >> 4;
            let x_val = cpu.get_register(rx);
            let y_val = cpu.get_register(ry);
            if x_val < y_val{
                cpu.set_register(0x0F, 0);
            }else {
                cpu.set_register(0x0F, 1);
            }
            cpu.set_register(rx, x_val.wrapping_sub(y_val));
            cpu.next_op();
        },
        OPCODE::OP8XY6 => {
            let rx = code[0] & 0x0F;
            let x_val = cpu.get_register(rx);
            cpu.set_register(0x0F, x_val & 1);
            cpu.set_register(rx, x_val >>  1);
            cpu.next_op();
        },
        OPCODE::OP8XY7 => {
            let rx = code[0] & 0x0F;
            let ry = code[1] >> 4;
            let x_val = cpu.get_register(rx);
            let y_val = cpu.get_register(ry);
            if x_val > y_val{
                cpu.set_register(0x0F, 0);
            }else {
                cpu.set_register(0x0F, 1);
            }
            cpu.set_register(rx, y_val.wrapping_sub(x_val));
            cpu.next_op();
        },
        OPCODE::OP8XYE => {
            let rx = code[0] & 0x0F;
            let x_val = cpu.get_register(rx);
            if x_val & 2_u8.pow(7) != 0 {
                cpu.set_register(0x0F, 1);
            } else {
                cpu.set_register(0x0F, 0);
            }
            cpu.set_register(rx, x_val <<  1);
            cpu.next_op();
        },
        OPCODE::OP9XY0 => {
            let rx = code[0] & 0x0F;
            let ry = code[1] >> 4;
            if cpu.get_register(rx) != cpu.get_register(ry){
                cpu.next_op();
            }
            cpu.next_op();
        },
        OPCODE::OPANNN => {
            let address: u16 = (code[0] as u16 & 0x0F) << 8 | code[1] as u16;
            cpu.set_index(address);
            cpu.next_op();
        },
        OPCODE::OPBNNN => {
            let address: u16 = (code[0] as u16 & 0x0F) << 8 | code[1] as u16;
            let address: u16 = address + cpu.get_register(0) as u16;
            cpu.set_address(address);
        },
        OPCODE::OPCXNN => {
            let rx = code[0] & 0x0F;
            let rand: u8 = rand::thread_rng().gen();
            cpu.set_register(rx, rand & code[1]);
            cpu.next_op();
        },
        OPCODE::OPDXYN => {  // add support to scren
                println!("draw sprite not implmented");
                cpu.next_op();
        },
        OPCODE::OPEX9E => {
            let key_pos = cpu.get_register(code[0] & 0x0F);
            if cpu.get_key(key_pos){
                cpu.next_op();
            }
            cpu.next_op();
        },
        OPCODE::OPEXA1 => {
            let key_pos = cpu.get_register(code[0] & 0x0F);
            if !cpu.get_key(key_pos){
                cpu.next_op();
            }
            cpu.next_op();
        },
        OPCODE::OPFX07 => {
            let delay = cpu.get_delay();
            cpu.set_register(code[0] & 0x0F, delay);
            cpu.next_op();

        },
        OPCODE::OPFX0A => {
            println!("get key didn't implemented");
            cpu.next_op();
        },
        OPCODE::OPFX15 => {
            let new_delay = cpu.get_register(code[0] & 0x0F);
            cpu.set_delay(new_delay);
            cpu.next_op();

            ;        },
        OPCODE::OPFX18 => {
            let new_sound = cpu.get_register(code[0] & 0x0F);
            cpu.set_sound(new_sound);
            cpu.next_op();
        },
        OPCODE::OPFX1E => {
            let index = cpu.get_index();
            let register_val = cpu.get_register(code[0] & 0x0F) as u16;
            if index + register_val >  0xFFF {
                //all the implementaion in web don't support overflow
                // i add panic and if one of the games fall i will add
                // the support for this situation
                //cpu.set_register(0x0F, 1);
                //cpu.set_index(index + regiset_va; - 0xFFF);
                panic!("opFX1E set index overflow I = {} register value =  {} ",
                       index, register_val)
            }
            cpu.set_index(index + register_val);
            cpu.next_op();

        },
        OPCODE::OPFX29 => {
            println!("spirit not implemented");
            cpu.next_op();
        },
        OPCODE::OPFX33 => {
            println!("bcd not implemented");
            cpu.next_op();
        },
        OPCODE::OPFX55 => {
            for reg in 0_u16..(code[1] & 0x0F) as u16{
                let pos = cpu.get_index() + reg;
                let val: u8 = cpu.get_register(reg as u8);
                cpu.set_memory(pos, val);
            }
            cpu.next_op();
        },
        OPCODE::OPFX65 => {
            for reg in 0_u16..(code[1] & 0x0F) as u16{
                let pos = cpu.get_index() + reg;
                let val: u8 = cpu.get_register(reg as u8);
                cpu.set_memory(pos, val);
            }
            cpu.next_op();
        },
        OPCODE::None => {
            panic!("OP code UKNOWN {:x}{:x} {:x}{:x}", code[0] & 0xF0,
             code[0] & 0x0F, code[1] & 0xF0, code[1] & 0x0F);
        },
    }
}

mod test{
    use super::*;

    fn get_code(cpu: &mut Chip8) -> Code{
        [cpu.memory[cpu.pc as usize], cpu.memory[cpu.pc as usize + 1]]
    }

    #[test]
    fn test_execute_2NNN(){
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_1NNN build infstructures failed");
        cpu.pc = 0x0210;
        let code : [u8; 2] = get_code(&mut cpu);
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

    #[test]
    fn test_execute_3XNN_true(){
        let mut cpu = Chip8::new("chip_games/PONG").
            expect("test  test_execute_1NNN build infstructures failed");

        cpu.pc = 0x021C;
        cpu.set_register(0x00, 0);
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP3XNN, &code, &mut cpu);
        assert!(cpu.pc == 0x0220);
        assert!(cpu.V[0x00] == 0);
    }

    #[test]
    fn test_execute_3XNN_false(){
        let mut cpu = Chip8::new("chip_games/PONG").
            expect("test  test_execute_1NNN build infstructures failed");

        cpu.pc = 0x021C;
        cpu.set_register(0x00, 11);
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP3XNN, &code, &mut cpu);
        assert!(cpu.pc == 0x021e);
        assert!(cpu.V[0x00] == 11);
    }


    #[test]
    fn test_execute_4XNN_true(){
        let mut cpu = Chip8::new("chip_games/PONG").
            expect("test  test_execute_1NNN build infstructures failed");

        cpu.pc = 0x0270;
        cpu.set_register(0x07, 7);
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP4XNN, &code, &mut cpu);
        assert!(cpu.pc == 0x0274);
        assert!(cpu.V[0x07] == 7);
    }


    #[test]
    fn test_execute_4XNN_false(){
        let mut cpu = Chip8::new("chip_games/PONG").
            expect("test  test_execute_4XNN build infstructures failed");

        cpu.pc = 0x0270;
        cpu.set_register(0x07, 0);
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP4XNN, &code, &mut cpu);
        assert!(cpu.pc == 0x0272);
        assert!(cpu.V[0x07] == 0);
    }

    #[test]
    fn test_execute_5XY0_true(){
        let mut cpu = Chip8::new("chip_games/MISSILE").
            expect("test  test_execute_5XY0 build infstructures failed");

        cpu.pc = 0x0204;
        cpu.set_register(0x03, 92);
        cpu.set_register(0x05, 92);

        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP5XY0, &code, &mut cpu);
        assert!(cpu.pc == 0x0208);
        assert!(cpu.V[0x03] == 92);
        assert!(cpu.V[0x05] == 92);
    }

    #[test]
    fn test_execute_5XY0_false(){
        let mut cpu = Chip8::new("chip_games/MISSILE").
            expect("test  test_execute_1NNN build infstructures failed");

        cpu.pc = 0x0204;
        cpu.set_register(0x03, 1);
        cpu.set_register(0x05, 13);

        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP5XY0, &code, &mut cpu);
        assert!(cpu.pc == 0x0206);
        assert!(cpu.V[0x03] == 1);
        assert!(cpu.V[0x05] == 13);
    }


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

    #[test]
    fn test_execute_8XY2(){
        let mut cpu = Chip8::new("chip_games/KALEID").
            expect("test  test_execute_8XY2 build infstructures failed");
        cpu.set_register(0x02, 112);
        cpu.set_register(0x0B, 91);
        cpu.pc = 0x0256;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY2, &code, &mut cpu);
        assert!(cpu.pc == 0x0258);
        assert!(cpu.V[0x02] == 112 & 91);
        assert!(cpu.V[0x0B] == 91);
    }

    #[test]
    fn test_execute_8XY3(){
        let mut cpu = Chip8::new("chip_games/BLINKY").
            expect("test  test_execute_8XY3 build infstructures failed");
        cpu.set_register(0x0E, 12);
        cpu.pc = 0x08A8;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY3, &code, &mut cpu);
        assert!(cpu.pc == 0x08AA);
        assert!(cpu.V[0x0E] == 12 ^ 12);
    }

    #[test]
    fn test_execute_8XY4_carry(){
        let mut cpu = Chip8::new("chip_games/GUESS").
            expect("test  test_execute_8XY4 build infstructures failed");
        cpu.set_register(0x0E, 200);
        cpu.pc = 0x022C;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY4, &code, &mut cpu);
        assert!(cpu.pc == 0x022E);
        assert!(cpu.V[0x0F] == 1);
        assert_eq!(cpu.V[0x0E], (400 - 256) as u8);
    }

    #[test]
    fn test_execute_8XY4_without_carry(){
        let mut cpu = Chip8::new("chip_games/GUESS").
            expect("test  test_execute_8XY4 build infstructures failed");
        cpu.set_register(0x0E, 10);
        cpu.pc = 0x022C;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY4, &code, &mut cpu);
        assert!(cpu.pc == 0x022E);
        assert!(cpu.V[0x0F] == 0);
        assert!(cpu.V[0x0E] == 10 + 10);
    }

    #[test]
    fn test_execute_8XY5_without_carry(){
        let mut cpu = Chip8::new("chip_games/INVADERS").
            expect("test  test_execute_8XY5 build infstructures failed");
        cpu.set_register(0x0D, 10);
        cpu.set_register(0x0A, 8);
        cpu.pc = 0x03A4;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY5, &code, &mut cpu);
        assert!(cpu.pc == 0x03A6);
        assert!(cpu.V[0x0F] == 1);
        assert!(cpu.V[0x0D] == 2);
        assert!(cpu.V[0x0A] == 8);
    }

    #[test]
    fn test_execute_8XY5_carry() {
        let mut cpu = Chip8::new("chip_games/INVADERS").
            expect("test  test_execute_8XY5 build infstructures failed");
        cpu.set_register(0x0D, 10);
        cpu.set_register(0x0A, 12);
        cpu.pc = 0x03A4;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY5, &code, &mut cpu);
        assert!(cpu.pc == 0x03A6);
        assert!(cpu.V[0x0F] == 0);
        assert!(cpu.V[0x0D] == 254);
        assert!(cpu.V[0x0A] == 12);
    }

    #[test]
    fn test_execute_8XY6_significant_bit_1() {
        let mut cpu = Chip8::new("chip_games/BLINKY").
            expect("test  test_execute_8XY6 build infstructures failed");
        cpu.set_register(0x01, 121);
        cpu.pc = 0x02E6;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY6, &code, &mut cpu);
        assert!(cpu.pc == 0x02E8);
        assert!(cpu.V[0x0F] == 1);
        assert!(cpu.V[0x01] == 121 >> 1);
    }

    #[test]
    fn test_execute_8XY6_significant_bit_0() {
        let mut cpu = Chip8::new("chip_games/BLINKY").
            expect("test  test_execute_8XY6 build infstructures failed");
        cpu.set_register(0x01, 94);
        cpu.pc = 0x02E6;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY6, &code, &mut cpu);
        assert!(cpu.pc == 0x02E8);
        assert!(cpu.V[0x0F] == 0);
        assert!(cpu.V[0x01] == 94 >> 1);
    }

    #[test]
    fn test_execute_8XY7_without_carry(){
        let mut cpu = Chip8::new("chip_games/SYZYGY").
            expect("test  test_execute_8XY5 build infstructures failed");
        cpu.set_register(0x00, 5);
        cpu.set_register(0x01, 10);
        cpu.pc = 0x02DA;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY7, &code, &mut cpu);
        assert!(cpu.pc == 0x02DC);
        assert!(cpu.V[0x0F] == 1);
        assert!(cpu.V[0x00] == 10 - 5);
        assert!(cpu.V[0x01] == 10);
    }

    #[test]
    fn test_execute_8XY7_carry() {
        let mut cpu = Chip8::new("chip_games/SYZYGY").
            expect("test  test_execute_8XY5 build infstructures failed");
        cpu.set_register(0x00, 10);
        cpu.set_register(0x02, 4);
        cpu.pc = 0x0474;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XY7, &code, &mut cpu);
        assert!(cpu.pc == 0x0476);
        assert!(cpu.V[0x0F] == 0);
        assert!(cpu.V[0x00] == 250);
        assert!(cpu.V[0x02] == 4);
    }

    #[test]
    fn test_execute_8XYE_significant_bit_1() {
        let mut cpu = Chip8::new("chip_games/TICTAC").
            expect("test  test_execute_8XY6 build infstructures failed");
        cpu.set_register(0x09, 132);
        cpu.pc = 0x0344;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XYE, &code, &mut cpu);
        assert!(cpu.pc == 0x0346);
        assert!(cpu.V[0x0F] == 1);
        assert!(cpu.V[0x09] == 132 << 1);
    }

    #[test]
    fn test_execute_8XYE_significant_bit_0() {
        let mut cpu = Chip8::new("chip_games/TICTAC").
            expect("test  test_execute_8XY6 build infstructures failed");
        cpu.set_register(0x09, 74);
        cpu.pc = 0x0344;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP8XYE, &code, &mut cpu);
        assert!(cpu.pc == 0x0346);
        assert!(cpu.V[0x0F] == 0);
        assert!(cpu.V[0x09] == 74 << 1);
    }

    #[test]
    fn test_execute_9XY0_false() {
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_9XY0 build infstructures failed");
        cpu.set_register(0x04, 74);
        cpu.set_register(0x05, 74);

        cpu.pc = 0x024E;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP9XY0, &code, &mut cpu);
        assert!(cpu.pc == 0x0250);
        assert!(cpu.V[0x04] == 74);
        assert!(cpu.V[0x05] == 74);
    }

    #[test]
    fn test_execute_9XY0_true() {
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_9XY0 build infstructures failed");
        cpu.set_register(0x04, 11);
        cpu.set_register(0x05, 74);

        cpu.pc = 0x024E;
        let code : [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OP9XY0, &code, &mut cpu);
        assert!(cpu.pc == 0x0252);
        assert!(cpu.V[0x04] == 11);
        assert!(cpu.V[0x05] == 74);
    }

    #[test]
    fn test_execute_ANNN() {
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_ANNN build infstructures failed");
        cpu.pc = 0x0208;
        let code: [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OPANNN, &code, &mut cpu);
        assert!(cpu.pc == 0x020A);
        assert!(cpu.I == 0x0203);
    }

    #[test]
    fn test_execute_CXNN() {
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_CNNN build infstructures failed");
        cpu.pc = 0x2D4;
        cpu.set_register(0x0D, 9);
        let code: [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OPCXNN, &code, &mut cpu);
        assert_eq!(cpu.pc, 0x2D6);
        assert_ne!(cpu.get_register(0x0D), 9); // the function generate random number that fail once in 256 time
    }

    #[test]
    fn test_execute_EX9E_true() {
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_CNNN build infstructures failed");
        cpu.pc = 0x2DE;
        cpu.set_register(0x0D, 4);
        cpu.key[0x04] = true;
        let code: [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OPEX9E, &code, &mut cpu);
        assert_eq!(cpu.pc, 0x2E2);
    }

    #[test]
    fn test_execute_EX9E_false() {
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_CNNN build infstructures failed");
        cpu.pc = 0x2DE;
        cpu.set_register(0x0D, 4);
        let code: [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OPEX9E, &code, &mut cpu);
        assert_eq!(cpu.pc, 0x2E0);
    }

    #[test]
    fn test_execute_EXA1_true() {
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_EXA1 build infstructures failed");
        cpu.pc = 0x2E2;
        cpu.set_register(0x0D, 4);
        cpu.key[0x04] = true;
        let code: [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OPEXA1, &code, &mut cpu);
        assert_eq!(cpu.pc, 0x2E4);
    }

    #[test]
    fn test_execute_EXA1_false() {
        let mut cpu = Chip8::new("chip_games/15PUZZLE").
            expect("test  test_execute_CNNN build infstructures failed");
        cpu.pc = 0x2E2;
        cpu.set_register(0x0D, 4);
        let code: [u8; 2] = get_code(&mut cpu);
        exe(OPCODE::OPEXA1, &code, &mut cpu);
        assert_eq!(cpu.pc, 0x2E6);
    }
    // add test for all F...
}