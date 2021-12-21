# m65

more info at https://gitlab.com/m-lego/m65


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

+ Qwerty

|------+------+------+------+------+------+------+------+------+------+------+------+------+------|
| Esc  |   1  |   2  |   3  |   4  |   5  |   6  |   7  |   8  |   9  |   0  |  -   | Bksp | B10  |
|------+------+------+------+------+------+------+------+------+------+------+------+------+------|
| Tab  |   q  |   W  |   E  |   R  |   T  |   Y  |   U  |   I  |   O  |   P  |  [   |   ]  |  A5  |
|------+------+------+------+------+-------------+------+------+------+------+------+------+------|
|   #  |   a  |   S  |   D  |   F  |   G  |   H  |   J  |   K  |   L  |   ;  |  '   | Enter|  A6  |
|------+------+------+------+------+------|------+------+------+------+------+------+------+------|
| Shift|  \   |   z  |   x  |   c  |   v  |   b  |   n  |   m  |   ,  |   .  |  Up  |  /   |  A7  |
|------+------+------+------+------+------+------+------+------+------+------+------+------+------|
| Ctrl | Menu | Lower| Alt  |Raise | Space| Space| Space|AltGr | Shift| Left | Down |Right |  B0  |
|------+------+------+------+------+------+------+------+------+------+------+------+------+------|
| A10  | A15  |  B3  |  B4  |  B5  |  B9  |  B8  |  B7  |  A1  |  A2  |  A3  |  A4  | B1   | Pins |
|------+------+------+------+------+------+------+------+------+------+------+------+------+------|


+ Encoders:

  - Pad_A: A0
  - Pad_B: B6

+ Leds

| Leds        | Pin |
| ----------- | --- |
| NUM_LOCK    | B12 |
| CAPS_LOCK   | C13 |
| SCROLL_LOCK | B13 |
| RBG_DI      | B15 |
