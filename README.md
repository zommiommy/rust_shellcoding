# rust_shellcoding
A minimal example to shellcode in rust, this is an example of my post <zom.wtf/posts/shellcoding-in-rust>.

```rust
const N: u64 = 100;

// our shellcode
#[no_mangle]
pub fn _start() {
    let mut buffer = [0; 1024];
    let mut sum = 0;
    for i in 0..N {
        let start = rdtsc();
        buffer[i as usize] = 0;
        let end = rdtsc();
        sum += end - start;
    }
    print!(sum / N);
}
```

Compile the shellcode with:
```bash
RUSTFLAGS="-C relocation-model=pie" cargo build --release --target="x86_64-unknown-none"
```

View the shellcode with:
```bash
objdump -D ./target/x86_64-unknown-none/release/shellcode -M intel -j .text

./target/x86_64-unknown-none/release/shellcode:     file format elf64-x86-64

Disassembly of section .text:

000000000000120d <.text>:
    120d:       6a 64                   push   0x64
    120f:       5f                      pop    rdi
    1210:       31 c9                   xor    ecx,ecx
    1212:       48 83 ef 01             sub    rdi,0x1
    1216:       72 1d                   jb     0x1235
    1218:       0f 31                   rdtsc  
    121a:       48 89 d6                mov    rsi,rdx
    121d:       48 c1 e6 20             shl    rsi,0x20
    1221:       48 09 c6                or     rsi,rax
    1224:       0f 31                   rdtsc  
    1226:       48 c1 e2 20             shl    rdx,0x20
    122a:       48 09 c2                or     rdx,rax
    122d:       48 29 f1                sub    rcx,rsi
    1230:       48 01 d1                add    rcx,rdx
    1233:       eb dd                   jmp    0x1212
    1235:       50                      push   rax
    1236:       6a 64                   push   0x64
    1238:       5e                      pop    rsi
    1239:       48 89 c8                mov    rax,rcx
    123c:       31 d2                   xor    edx,edx
    123e:       48 f7 f6                div    rsi
    1241:       48 89 c1                mov    rcx,rax
    1244:       51                      push   rcx
    1245:       48 c7 c2 08 00 00 00    mov    rdx,0x8
    124c:       48 89 e6                mov    rsi,rsp
    124f:       48 c7 c7 01 00 00 00    mov    rdi,0x1
    1256:       48 c7 c0 01 00 00 00    mov    rax,0x1
    125d:       0f 05                   syscall 
    125f:       58                      pop    rax
    1260:       c3                      ret    
```