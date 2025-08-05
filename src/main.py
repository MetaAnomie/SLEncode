def rle_encode_file(ifile, ofile):    
    with open(ifile, 'rb') as file:
        ibytes = file.read()
        prev = None
        buffer = bytearray(b'')
        cnt = 0
        for b in ibytes:
            if cnt == 0:
                cnt += 1
                prev = b
            elif b == prev and cnt < 259 and not (cnt == 258 and prev == 255):
                cnt += 1
            else:
                if cnt >= 4:
                    buffer.append(255)
                    buffer.append(prev)
                    if prev == 255: buffer.append(255)
                    buffer.append(cnt-4)                    
                else:
                    if prev == 255: cnt *= 2
                    for i in range(cnt): buffer.append(prev)
                prev = b
                cnt = 1
        
        if cnt >= 4:
            buffer.append(255)
            buffer.append(prev)
            if prev == 255: buffer.append(255)
            buffer.append(cnt-4)
        else:
            if prev == 255: cnt *= 2
            for i in range(cnt): buffer.append(prev)        
        
        with open(ofile, 'wb') as out_file:
            ibytes = out_file.write(buffer)