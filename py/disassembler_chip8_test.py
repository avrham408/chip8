from disassembler_chip8 import load_rom, disassembler, parse_code, parse_file
import os 


FILES_DIR = '../chip_games'


#help
def get_files_data():
    chip_games = []
    for f in get_files_name():
        with open(os.path.join(FILES_DIR, f), 'rb') as fb:
            chip_games.append(fb.read())
    return chip_games


def get_files_name():
    return os.listdir(FILES_DIR) 


def test_load_rom_0x200_empty():
    data = get_files_name()
    for f in data:
        mem = [0] * 4096
        try:
            load_rom(os.path.join(FILES_DIR, f), mem)
        except IndexError:
            print(f)
        for bite in mem[0: 0x200]:
            assert bite == 0


def test():
    pass
    

if __name__ == "__main__":
    test_load_rom_0x200_empty()
