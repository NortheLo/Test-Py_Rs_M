import bin_fil
from pathlib import Path



class FileReader:
    i = 100

    def get_header(counter: int, data: list):
        return bin_fil.Header(counter = counter, data=data)


    def write_file(path: Path, hdr: bin_fil.Header):
        return bin_fil.write_bin(path, hdr)
