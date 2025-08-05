# SLEncode
Customer run-length encoding (RLE) algorithm for python implemented in Rust.

## Native Rust Binary
https://github.com/MetaAnomie/SLEncode/blob/main/releases/v0.1.0/SLEncode.exe<br><br>
<b>Usage:</b><br><br>
```
SLEncode.exe encode in.txt out.enc<br>
SLEncode.exe decode out.enc orig.txt<br>
```
## Python Usage

```python
import slencode

slencode.rle_encodee_file("in.txt", "out.enc")
slencode.rle_decode_file("out.enc", "orig.txt")
```

## Performance

Benchmarked on Dell XPS 8930, Core i7-9700 CPU @ 3.00GHz (8 core), 16 GB Ram<br>
Test File Size: 492 MB<br><br>

Rust Implementation (Encoding): 23.01 Seconds<br>
Pure Python Implementation (Encoding): 65.97 Seconds
