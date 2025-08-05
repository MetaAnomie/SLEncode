# SLEncode
Customer run-length encoding (RLE) algorithm for python implemented in Rust.

## Native Rust Binary
https://github.com/MetaAnomie/SLEncode/blob/main/releases/v0.1.0/SLEncode.exe<br><br>
### Usage:<br><br>
SLEncode.exe encode in.txt out.enc<br>
SLEncode.exe decode out.enc out.dec<br>

## Python Usage

```python
import slencode

slencode.rle_encodee_file("in.txt", "test.enc")
slencode.rle_decode_file("in.enc", "orig2.mp4")

```

## Performance

Benchmarked on Dell XPS 8930, Core i7-9700 CPU @ 3.00GHz (8 core), 16 GB Ram<b>
Test File Size: 492 MB<br><br>

Rust Encoding: 23.01 Seconds<br>
Python Encoding: 65.97 Seconds
