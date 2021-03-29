the lego keyboard
=================

motivation
----------

this is just a fun project started some time ago.
The idea was to have an iso friendly ortholinear keyboard.
Initially I started with a 6x13 but then settled to a 5x13 as best option.
more info in [here](https://alin.elena.space/blog/keeblego/)


There is a [6x13](https://github.com/Kyrremann/index-tab) project if you want
and a [4x13](https://github.com/farfalleflickan/nack)

I am not related with any of them and dicovered them too late.

what is special about this one?

  * the case is set in lego (almost lego)
  * microcontroller is just a standard apm/stm32f103 blackpill from robotdyn


bom
---

 * lego: suggested 1x2 plates, they need to be "rotated" in holes initially since are tight fit
 * double sided plate 16x32 studs.... you will need to live with a compatible since lego does not make them in this size
 * 4 1x1 lego tiles or eyes.
 * optional 4 2x2 corner plates
 * 65 signal diodes 1N4148 , do 35
 * 2 resistors  and 2 leds, resistors need to be computed to match the colour of the led
 * 1 apm/stm32f103 blackpill from robotdyn F303 will work also since they are pin identical.
 * switches (5 pin) and keycaps... for pcb mount

pictures
--------

  the 3d render looks like

  ![3d render](pics/m65.png)


firmware
--------


