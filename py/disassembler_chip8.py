import logging
import sys

logger = logging.getLogger(__name__)


def load_rom(path: str, memory: list) -> int:
    with open(path, 'rb') as f:
        chip8_file = f.read()
    for byte_pos in range(len(chip8_file)):
        memory[0x200 + byte_pos] = chip8_file[byte_pos]
    return len(chip8_file)


def disassembler(code, pc):
    firstnib = code[0] >> 4
    print("{:04X} {:02X} {:02X}".format(pc, code[0], code[1]), end=" ")
    if firstnib == 0x00:
        if code[1] == 0xe0:
            print("{} \t ".format("CLS"))
        elif code[1] == 0xee:
            print("{} \t ".format("RTS"))
        else:
            print("UKNOWN 0")
    elif firstnib == 0x01:
        print("{} \t #${:01X}{:02X}".format("JUMP", code[0] & 0XF, code[1]))
    elif firstnib == 0x02:
        print("{} \t #${:01X}{:02X}".format("CALL", code[0] & 0XF, code[1]))
    elif firstnib == 0x03:
        print("{} \t V{:01X},#${:02X}".format("SKIP.EQ", code[0] & 0XF, code[1]))
    elif firstnib == 0x04:
        print("{} \t V{:01X},#${:02X}".format("SKIP.NE", code[0] & 0XF, code[1]))
    elif firstnib == 0x05:
        print("{} \t V{:01X},V{:01X}".format("SKIP.EQV", code[0] & 0XF, code[1] >> 4))
    elif firstnib == 0x06:
        reg = code[0] & 0x0f
        print("{} \t V{:01X},#${:02X}".format("MVI", reg, code[1]))
    elif firstnib == 0x07:
        print("{} \t V{:01X},#${:02X}".format("ADI", code[0] & 0XF, code[1]))
    elif firstnib == 0x08:
        lastnib = code[1] & 0x0F
        if lastnib == 0x00:
            print("{} \t V{:01X},V{:01X}".format("MOV", code[0] & 0XF, code[1] >> 4))
        elif lastnib == 0x01:
            print("{} \t V{:01X},V{:01X}".format("OR", code[0] & 0XF, code[1] >> 4))
        elif lastnib == 0x02:
            print("{} \t V{:01X},V{:01X}".format("AND", code[0] & 0XF, code[1] >> 4))
        elif lastnib == 0x03:
            print("{} \t V{:01X},V{:01X}".format("XOR", code[0] & 0XF, code[1] >> 4))
        elif lastnib == 0x04:
            print("{} \t V{:01X},V{:01X}".format("ADD", code[0] & 0XF, code[1] >> 4))
        elif lastnib == 0x05:
            print("{} \t V{:01X},V{:01X},V{:01X}".format("SUB", code[0] & 0XF, code[0] & 0XF, code[1] >> 4))
        elif lastnib == 0x06:
            print("{} \t V{:01X},V{:01X}".format("SHR", code[0] & 0XF, code[1] >> 4))
        elif lastnib == 0x07:
            print("{} \t V{:01X},V{:01},V{:01X}".format("SUB", code[0] & 0XF, code[1] >> 4, code[1] >> 4))
        elif lastnib == 0x0E:
            print("{} \t V{:01X},V{:01X}".format("SHL", code[0] & 0XF, code[1] >> 4))
        else:
            print("UKNOWN 8 -{}".format(lastnib))
    elif firstnib == 0x09:
        print("{} \t V{:01X},V{:01X}".format("SKIP.NE", code[0] & 0XF, code[1] >> 4))
    elif firstnib == 0x0a:
        address = code[0] & 0x0f
        print("{}\t I,#${:01X}{:02X}".format("MVI", address, code[1]))
    elif firstnib == 0x0b:
        print("{} \t #${:01X}{:02X}(V0)".format("JUMP", code[0] & 0XF, code[1]))
    elif firstnib == 0x0c:
        print("{} \t V{:01X},#${:02X}".format("RNDMSK", code[0] & 0XF, code[1]))
    elif firstnib == 0x0d:
        print("{} \t V{:01X},V{:01X},#?{:01X}".format("SPRITE", code[0] & 0XF, code[1] >> 4, code[1] & 0x0F))
    elif firstnib == 0x0e:
        if code[1] == 0x9E:
            print("{} \t V{:01X}".format("SKIPKEY.Y", code[0] & 0XF))
        elif code[1] == 0xA1:
            print("{} \t V{:01X}".format("SKIPKEY.N", code[0] & 0XF))
        else:
            # raise Exception("UKNOWN E")
            pass
    elif firstnib == 0x0f:
        if code[1] == 0X07:
            print("{} \t V{:01X}, DELAY".format("MOV", code[0] & 0XF))
        elif code[1] == 0X0a:
            print("{} \t V{:01X}".format("KEY", code[0] & 0XF))
        elif code[1] == 0X15:
            print("{} \t DELAY,V{:01X}".format("MOV", code[0] & 0XF))
        elif code[1] == 0X18:
            print("{} \t SOUND,V{:01X}".format("MOV", code[0] & 0XF))
        elif code[1] == 0X1e:
            print("{} \t I,V{:01X}".format("ADI", code[0] & 0XF))
        elif code[1] == 0X29:
            print("{} \t I,V{:01X}".format("SPRITECHAR", code[0] & 0XF))
        elif code[1] == 0X33:
            print("{} \t (I),V{:01X}".format("MOVBCD", code[0] & 0XF))
        elif code[1] == 0X55:
            print("{} \t (I),V0-V{:01X}".format("MOVM", code[0] & 0XF))
        elif code[1] == 0X65:
            print("{} \t V0-V{:01X},(I)".format("MOVM", code[0] & 0XF))
        else:
            # raise Exception("UKNOWN F")
            pass


def parse_code(memory: list, size: int, PC):
    for _ in range(int(size / 2)):
        code = memory[PC: PC + 2]
        disassembler(code, PC)
        PC += 2


def parse_file(file_name):
    memory = [0] * 4096
    PC = 0X200  # program counter
    size = load_rom(file_name, memory)
    parse_code(memory, size, PC)


if __name__ == "__main__":
    file_name = sys.argv[1]
    parse_file(file_name)

