clc; clear;

% set to interpreter from venv
pyenv(Version=".venv/bin/python");

bin_fil = py.importlib.import_module('wrap');

path = "m_file2.bin";

fil_obj = bin_fil.FileReader

file = fil_obj.read_file("m_file.bin")

i = fil_obj.i



hdr = fil_obj.get_header(cast(1, "uint32"), [9.0, 9.0, 78.9])

fil_obj.write_file(path, hdr)

