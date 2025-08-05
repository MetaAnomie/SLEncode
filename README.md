# SLEncode
Customer run-length encoding (RLE) algorithm for python implemented in Rust.

## Native Rust Binary
https://github.com/MetaAnomie/SLEncode/blob/main/releases/v0.1.0/SLEncode.exe<br>
Usage:<br>
SLEncode.exe encode infile.txt outfile.enc<Br>
SLEncode.exe decode outfile.enc orig.txt<br>

## Python Usage

## Performance


```python
import slencode

slencode.rle_encodee_file("in.txt", "test.enc")
slencode.rle_decode_file("in.enc", "orig2.mp4")

```
