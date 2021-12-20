install
-------


```
curl https://sh.rustup.rs -sSf | sh
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

build
-----

```
cargo objcopy --bin m65 --release -- -O binary m65.bin
```

flash
-----

```
dfu-util -d 0483:df11 -a 0 --dfuse-address 0x08000000 -D m65.bin
```


Pins
----


| Row | Pin |
| --- | --- |
| 0   | B10 |
| 1   | A5  |
| 2   | A6  |
| 3   | A7  |
| 4   | B0  |

| Col | Pin |
| --- | --- |
| 0   | A10 |
| 1   | A15 |
| 2   |  B3 |
| 3   |  B4 |
| 4   |  B5 |
| 5   |  B9 |
| 6   |  B8 |
| 7   |  B7 |
| 8   |  A1 |
| 9   |  A2 |
|10   |  A3 |
|11   |  A4 |
|12   |  B1 |

Encoders:

Pad_A: A0
Pad_B: B6

| Leds       | Pin |
| ---------- | --- |
| NUMLOCK    | B12 |
| CAPSLOCK   | C13 |
| SCROLLLOCK | B13 |
| RBG_DI     | B15 |
