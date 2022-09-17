## tipro 8x16

  ![tipro 128 8x16](pics/tipro/m8x16.jpg)

### introduction

this is a pos tipro 8x16 which I converted to use qmk.
It is a handwired like project with a daugher board.

it uses stm32f401 on a pcb but since there are not enough pins for the 7 leds I used a shift register to control the leds.

### pcb

 3d render

 ![tipro conversion pcb](handwire8x16-bottom.png)

 printed pcbs

 ![tipro pcb](pics/tipro/mpcb-1.jpg)
 ![tipro pcb](pics/tipro/mpcb-2.jpg)

 wiring

 ![tipro 128 8x16](pics/tipro/m8x16-back.jpg)

  full kicad project for the pcb in [here](https://gitlab.com/m-lego/hand8x16/)

### firmware

  ready made firmware can be downloaded

  + [tipro_rev1_default.uf2](https://gitlab.com/m-lego/hand8x16/-/blob/develop/firmware/tipro_rev1_default.uf2)

  build your own

   ```bash
      git clone --recurse-submodules -b mlego https://github.com/alinelena/qmk_firmware.git qmk-alin
      cd qmk-alin
      qmk compile -kb tipro/rev1 -km default

   ```
   copy the resulting uf2 on the mcu.

### pins

   - cols: B5,B4,C14,C15, A0, A1, A2, A3, A7, B0, B10, B1
   - rows: B12, B13, B14, B15, A8, A10, A15, B3

  - leds: shift register - QA-QG

** 74HC595N **

  - data: A4
  - latch: A6
  - clock: A5


### pictures

original

![tipro 128 8x16](pics/tipro/m8x16-top.jpg)


