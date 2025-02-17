import bin_fil

print(bin_fil.sum_as_string(5, 20))

file = bin_fil.read_bin("m_file.bin")

hdr = bin_fil.Header(counter = 1, data=[2.0, 9.0, 9.9])
result = bin_fil.write_bin("test.bin", hdr)
print(result) 

