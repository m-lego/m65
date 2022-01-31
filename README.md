the lego keyboard
=================

  current iteration...

  ![rev4 aka version 5](pics/m65-rev4.jpg)

  the 3d render looks like (version 5)

  ![3d render](pics/m65-rev4-render.png)

  the pcb for version 5

  ![pcb](pics/m65-rev4-pcb.png)

  the real pcb

  ![the current](pics/m65-rev4-pcb.jpg)

  the case from lego...

  ![the case render](pics/m65-case.png)

status: **version 5** **rev 4** tested ok, current
----------------------------------

* [x] gerbers designed
* [x] firmware
* [x] breadboard tested
* [x] gerbers printed
* [x] board tested

v1 (rev1) and v2 (rev2) are fully tested
v3 I never printed and do not intend to do so.
v4 is tested and working (rev3) is the firmware working with it..
v5 is current and working (rev4)

warning
-------

**this project is**

* fun
* educational
* functional
* cheap(ish), till switches and keycaps.

**this project is not**

* end game


motivation
----------

this is just a fun project started some time ago.
The idea was to have an iso friendly ortholinear keyboard.
Initially I started with a 6x13 but then settled to a 5x13 as best option.
more info in [here](https://alin.elena.space/blog/keeblego/)


There is a [6x13](https://github.com/Kyrremann/index-tab) project if you want
and a [4x13](https://github.com/farfalleflickan/nack)

I am not related with any of them and discovered some of them too late.

I have also made a split, 5x12 and 4x12, see below.


what is special about this one?

  * the case is set in lego (almost lego)
  * microcontroller is just a standard apm/stm32f103 blackpill from robotdyn  or gd32f303 bluepill plus from we act studio([gerbers](gerbers-stm32f103.zip)), version
    with stm32f401 available now too see the [gerbers](gerbers-stm32f401.zip)
  * rotary encoder (no switch)
  * oled 128x32 i2c
  * optional led strip (you will need a full brick lego or one more layer..)

**note** the holes in current version are cut at the official lego dimension. Unfortunately the drilling machines have tolerance and
the same is true for the lego making tools... the tolerance in lego varies with colour of the plastic... you will find some colours may not fit
as well as others. I suggest you to use 1x2 and 1x4 plates if you want the finest keyboard, since they tend to bend the pcb... you will be
amased but the force.

if you do not want an angle for typing use a single sided plate as base.

foaming the bottom makes a difference i use usually diy/hobbycraft foam, that you can buy for close to nothing in a4 sheets.

see also
--------

 * [5x12 split in lego](https://gitlab.com/m-lego/m60_split)
 * [5x12 in lego](https://gitlab.com/m-lego/m60)
 * [4x12 in lego](https://gitlab.com/m-lego/m48)

bom
---

 * lego: suggested 1x2 plates(46) or 1x4 plates some colours fit better than others.
 * double sided plate 16x32 studs.... you will need to live with a compatible since lego does not make them in this size, one thing
   i noticed is some plates may make the pcb to slightly bend... seems gray coloured ones are ok... no idea why.
 * 4 1x1 lego tiles or eyes.(optionally)
 * optional 4 2x2 corner plates(optionally)
 * 65 signal diodes 1N4148 , do 35
 * 2 resistors (510Ω (R1,R2) works fine for green and yellow leds you need to check the resistance for the right intensity you want)  and 2 leds, resistors need to be computed to match the colour of the led
1kΩ for whites will be bright
 * 1 apm/stm32f103 blackpill from robotdyn, F303 will work also since they are pin identical.
 * or gd32f303 from we act studio...
 * or stm32f401 from we act
 * or stm32f411 from we act, ask for the firmware
 * 100kΩ resistor for (R3)
 * switches (5 pin) and keycaps... for pcb mount
 * rotary encoder (I got this Bourns 24 Pulse Incremental Mechanical Rotary Encoder with a 6 mm Flat Shaft but any similar shall do)
 * jst horizontal header 3 pin, if you add leds strip
 * oled 128x32 i2c (4 pins)
 * dil socket 1x4 row for oled.
 * round pin headers 2x20 for MCU
 * dil socket for MCU 40 way 15.24mm
 * usb cable, c or mini depending on the mcu.


assembly
--------

this is a very rush [assembly guide](assembly.md)
thickness
----------

depending on what you want the box can come out very thin.. 1 base plate(single or double, flat or angled usage) plus pcb(1.6mm) plus one plate
intermediate(3.2mm).
that is thin. This shall allow you to put in a sheet of foam for sound dampening.

  ![1 double base plate, 1 intermediate plate and pcb. 8 mm ](pics/thick-1.jpg)

2 layers of intermediate plates, for thicker foam sheet or for led strip which is connected by hand to the pins or holes... no jst
header.

   ![1 double base plate, 2 intermediate plate and pcb. 11.2 mm ](pics/thick-2.jpg)

1 full brick if you want a jst header.

  ![1 double base plate, 1 intermediate brick and pcb. 14.4  mm ](pics/thick-3.jpg)

pictures
--------

  the 3d render looks like (version 1)

  ![3d render](pics/m65.png)

  version 2

  ![3d render](pics/m65-v2.png)

  the pcb (version 1)

  ![the pcb](pics/m65-pcb.jpg)

   version (2)

  ![the pcb](pics/m65-pcb-2.jpg)

   version (3) stm32f103 compatible pinout superseeded by version 4 and 401 go for that

  ![the pcb](pics/m65-pcb3.png)

   version (4) stm32f401 from we act

  ![the pcb](pics/m65-pcb4.png)

  ![the pcb version 4 rev3 ](pics/m65-pcb-v4.jpg)

  ![the pcb rev4 ](pics/m65-rev4-pcb.jpg)

  just switches mounted

  ![just there](pics/m65-nokey.jpg)

  keyboard no 1 (mt3 tty bleached, kailh crystal royal)

  ![with dev tty](pics/m65-tty.png)

  ![with dev tty](pics/m65-tty-f.jpg)

  keyboard no 2 (mt3 susuwatari, kailh crystal jade)

  ![with susuwatari](pics/m65-susu.jpg)

  ![with susuwatari](pics/m65-susu-2.jpg)

  keyboard no 3 (mt3 tty, kailh crystal royal)

  ![one more /dev/tty](pics/m65-tty-2.jpg)

  ![one more /dev/tty](pics/m65-home.jpg)

  ![one more /dev/tty](pics/m65-home-2.jpg)

  keyboard no 4 (tbd, novelkeys blueberry in process of re-springing)

  ![one more /dev/tty](pics/m65-tty-nkblueberry.jpg)

  keyboard no 5 (tbd, gateron ink v2 yellow and red)

  ![keycaps to come... dk layout envisaged](pics/m65-gats.jpg)

  keyboard no 6 (kailh crystal jades with click bar removed... )

  ![keycaps to come... prob mt3 godspeed mixed with tty](pics/m65-jade-lin.jpg)

  ![keycaps to come... prob mt3 godspeed mixed with tty](pics/m65-jade-lin-2.jpg)

  ![in /dev/tty with a touch of godspeed and "linear" jades -- clickbars removed](pics/m65-japan.jpg)

  ![in /dev/tty with a bigger touch of godspeed and "linear" jades -- clickbars removed](pics/m65-japan-2.jpg)

  keyboard no 7 (gateron whites and /dev/tty)

  ![/dev/tty mt3 ](pics/m65-white-naked.jpg)

  ![/dev/tty mt3 ](pics/m65-gat-white.jpg)

  ![/dev/tty and godspeed mods ](pics/m65-g-white.jpg)

  keyboard no 8 (frankenstein switches linears with mt3 camillo)

  ![mt3 camillo with a touch of godspeed and susuwatari ](pics/m65-olivetti.jpg)

  keyboard no 9 (mt3 2048)

  ![mt3 2048 with a touch of colour ](pics/m65-2048.jpg)

  keyboard no 10 (mt3 dasher)

  ![mt3 dasher ](pics/m65-dasher.jpg)

  keyboard no 11 (mt3 cyber)

  ![mt3 mito cyber ](pics/m65-cyber.jpg)

  keyboard no 12 (mt3 3277)

  ![mt3 matt3o 3277 ](pics/m65-3277.jpg)

  keyboard no 13 (mt3 fairlane)

  ![mt3 fairlane ](pics/m65-fair.jpg)

  keyboard no 14 (commodore c64 harvest keycaps with mx adapters)

  ![mt3 c64 ](pics/m65_c64.jpg)

  keyboard no 15 (mt3 camillo, eva 02)

  ![mt3 camillo ](pics/m65-camillo.jpg)

firmware
--------

   layout is bellow, but since is qmk can be whatever one likes, no 5 will be Danish for example.

   ![layout simplified summary](pics/m65-l1.png)

   ![layout lwr](pics/m65-lwr.png)

   ![layout rse](pics/m65-rse.png)

   is qmk and is in a branch for the moment, I assume you already have qmk environment configured.

   microcontroller stm32f103/apm32 from robotdyn... aka black pill, https://robotdyn.com/black-pill-apm32f103cb-128kb-flash-20kb-sram-stm32-compatible-arm-cortexr-m3-mcu-mini-board.html

```bash
   git clone --recurse-submodules https://github.com/qmk/qmk_firmware.git
   make mlego/m65/rev1:uk
   make mlego/m65/rev1:uk:flash
```

you can use also gdf303 from we act aka bluepill plus  https://github.com/WeActTC/BluePill-Plus

```bash
   git clone --recurse-submodules https://github.com/qmk/qmk_firmware.git
   make mlego/m65/rev2:uk
   make mlego/m65/rev2:uk:flash
```

you can use also stm32f401 from we act

```bash
   git clone --recurse-submodules https://github.com/qmk/qmk_firmware.git
   make mlego/m65/rev3:uk
   make mlego/m65/rev3:uk:flash
```

for rev4

```bash
   git clone --recurse-submodules https://github.com/qmk/qmk_firmware.git
   make mlego/m65/rev4:uk
   make mlego/m65/rev4:uk:flash
```


pins and more
=============

## Pins and leds rev1 STM/APM32F103C8T6

similar pinout for STM32F303

| Rows  | C0    | C1    | C2    | C3    | C4    | C5    | C6    | C7    | C8    | C9    | C10   | C11   | C12   | Pins  |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| R0    | Esc   | 1     | 2     | 3     | 4     | 5     | 6     | 7     | 8     | 9     | 0     | -     | Bksp  | B11   |
| R1    | Tab   | q     | w     | e     | r     | t     | y     | u     | i     | o     | p     | [     | ]     | B0    |
| R2    | #     | a     | s     | d     | f     | g     | h     | j     | k     | l     | ;     | '     | Enter | B1    |
| R3    | Shift | \     | z     | x     | c     | v     | b     | n     | m     | ,     | .     | Up    | /     | A2    |
| R4    | Ctrl  | Menu  | Lower | Alt   | Raise | Space | Space | Space | AltGr | Shift | Left  | Down  | Right | A3    |
|       | A10   | A15   | B3    | B4    | B5    | B9    | B8    | B7    | B6    | C15   | A0    | A7    | B10   |       |

### Encoders

  - Pad_A: A8
  - Pad_B: A9

### Leds

| Leds        | Pin |
| ----------- | --- |
| NUM_LOCK    | B12 |
| CAPS_LOCK   | C13 |
| SCROLL_LOCK | B13 |
| RBG_DI      | B15 |

## Pins and leds rev2 GD32F303CCT6


| Rows  | C0    | C1    | C2    | C3    | C4    | C5    | C6    | C7    | C8    | C9    | C10   | C11   | C12   | Pins  |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| R0    | Esc   | 1     | 2     | 3     | 4     | 5     | 6     | 7     | 8     | 9     | 0     | -     | Bksp  | B11   |
| R1    | Tab   | q     | w     | e     | r     | t     | y     | u     | i     | o     | p     | [     | ]     | B0    |
| R2    | #     | a     | s     | d     | f     | g     | h     | j     | k     | l     | ;     | '     | Enter | B1    |
| R3    | Shift | \     | z     | x     | c     | v     | b     | n     | m     | ,     | .     | Up    | /     | A2    |
| R4    | Ctrl  | Menu  | Lower | Alt   | Raise | Space | Space | Space | AltGr | Shift | Left  | Down  | Right | A3    |
|       | A10   | A15   | B3    | B4    | B5    | B9    | B8    | B7    | B6    | C15   | A0    | A7    | B10   |       |


### Encoders

  - Pad_A: A8
  - Pad_B: A9

### Leds

| Leds        | Pin |
| ----------- | --- |
| NUM_LOCK    | B12 |
| CAPS_LOCK   | B2  |
| SCROLL_LOCK | B13 |
| RBG_DI      | B15 |


## Pins and leds rev3 stm32f401

the pinout is the same for stm32f411

| Rows  | C0    | C1    | C2    | C3    | C4    | C5    | C6    | C7    | C8    | C9    | C10   | C11   | C12   | Pins  |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| R0    | Esc   | 1     | 2     | 3     | 4     | 5     | 6     | 7     | 8     | 9     | 0     | -     | Bksp  | B10   |
| R1    | Tab   | q     | w     | e     | r     | t     | y     | u     | i     | o     | p     | [     | ]     | A5    |
| R2    | #     | a     | s     | d     | f     | g     | h     | j     | k     | l     | ;     | '     | Enter | A6    |
| R3    | Shift | \     | z     | x     | c     | v     | b     | n     | m     | ,     | .     | Up    | /     | A7    |
| R4    | Ctrl  | Menu  | Lower | Alt   | Raise | Space | Space | Space | AltGr | Shift | Left  | Down  | Right | B0    |
|       | A10   | A15   | B3    | B4    | B5    | B9    | B8    | B7    | A1    | A2    | A3    | A4    | B1    |       |

### Encoders

  - Pad_A: A0
  - Pad_B: B6

### LEDS

| Leds        | Pin |
| ----------- | --- |
| NUM_LOCK    | B12 |
| CAPS_LOCK   | C13 |
| SCROLL_LOCK | B13 |
| RBG_DI      | B15 |

## Pins and leds rev4 stm32f401

the pinout is the same for stm32f411

| Rows  | C0    | C1    | C2    | C3    | C4    | C5    | C6    | C7    | C8    | C9    | C10   | C11   | C12   | Pins  |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| R0    | Esc   | 1     | 2     | 3     | 4     | 5     | 6     | 7     | 8     | 9     | 0     | -     | Bksp  | B10   |
| R1    | Tab   | q     | w     | e     | r     | t     | y     | u     | i     | o     | p     | [     | ]     | A5    |
| R2    | #     | a     | s     | d     | f     | g     | h     | j     | k     | l     | ;     | '     | Enter | A6    |
| R3    | Shift | \     | z     | x     | c     | v     | b     | n     | m     | ,     | .     | Up    | /     | A7    |
| R4    | Ctrl  | Menu  | Lower | Alt   | Raise | Space | Space | Space | AltGr | Shift | Left  | Down  | Right | B0    |
|       | B14   | A8    | A10   | A15   | B3    | B4    | B5    | B7    | A1    | A2    | A3    | A4    | B1    |       |

### Encoders

  - Pad_A: A0
  - Pad_B: B6

### Oled

  - SDA: B9
  - SCL/SCK: B8

### LEDS

| Leds        | Pin |
| ----------- | --- |
| NUM_LOCK    | B12 |
| CAPS_LOCK   | C13 |
| SCROLL_LOCK | B13 |
| RBG_DI      | B15 |


