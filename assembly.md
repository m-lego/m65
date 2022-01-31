assembly guide
==============


before you start
----------------

mcu
---

whatever mcu you are using I suggest strongly to socket it.

first step is to add headers on it.

i use round pins and dil sockets, 40 way. picture below


![header mcu](pics/header_1.jpg)

put the headers into the socket no need to fully insert them just be sure the conical bit points towars the socket and the mcu on
top. see picture below.

![header mcu](pics/header_2.jpg)

be sure headers are vertical and stable. now you can start soldering.

I solder usually the extremes for each row and then the rest.


oled
----

if you are using an oled i suggest to socket it too. Again I use round pins 4 in a row are enough and use a low profile socket, see
below.


![header oled](pics/header_3.jpg)

for soldering this you can use the pcb as support. add the headers into the pcb(conical side towards pcb) put the oled on top and be
sure is vertical. you can use some blue tack or whatever. now you can solder the oled(be sure you do not solder the keyboard pcb
side). Once done you can remove it from pcb shall look like this.

![header oled](pics/header_4.jpg)


diodes
------

get 65 of them 1n4148 do-35, they look like this.

![diodes](pics/diodes_1.jpg)
notice the black colar at one end.

bend the legs at 90˚ like in the picture.

![diodes](pics/diodes_2.jpg)

insert them in the holes, with the black colar oriented to the lower part of the board.
see picture.

![diodes](pics/diodes_2.jpg)

bend the legs on the reverse of the pcb to assure the diode is fixed in the position.
see picture.

![diodes](pics/diodes_3.jpg)

now you can solder, once done, wriggle the legs of the diode until they detach, leaving a clean cut at solder level.
see picture

![diodes](pics/diodes_4.jpg)

repeat for all the diodes next to the switches.you may find it easier if you work your way for each column.

for the diodes at the top, bending shall be done differently, see picture. this allows to add them all in one go.

![diodes](pics/diodes_5.jpg)

resistors
---------

there are max three resistors you need to put procedure is the same as for the diodes, bend, insert, bend.
it does not matter how you insert them, just be sure you have the right values in the right places.

R3  is 100kΩ R1 and R2 need to be calculated based on the colour of the led used and desired intensity. for yellow and green leds I
use 510Ω(not very bright) and for white leds 1kΩ (quite bright)

![resistors](pics/res_1.jpg)

**note**: you may want to use a breadboard to test before to see if you are happy with the intensities, use a 3.3V DC source. alternatively
you can use the mcu itself. if you do not have a breadboard dupont wires may be enough.
also in this step you will determine the led orientation... note which leg goes to 3.3V and which one goes to ground.
this step is important since I noticed some of the leds do not follow the convention... long leg positive, short leg negative.


![resistors](pics/res_2.jpg)

sockets
-------


![socket mcu ](pics/socket_1.jpg)

put the 40 way dil socket into the pcb (front side). secure it with tape. turn the pcb.
you can solder the  extreme pins on rows. be sure the socket is properly aligned with the pcb


![socket oled ](pics/socket_2.jpg)

leds
----

![led oled](pics/led_1.jpg)

put the leds in the right position, matching the resistor value determined before. also you need to be sure the leg that pointed to
ground goes in the hole with the square plating. once inserted you can bend the legs on the back of the board and solder them.
wriggle to remove the legs.

![led oled](pics/led_2.jpg)

switches
--------

you need 65 switches 5 pins. be sure the legs are not bent. in general switches shall fit quite tight so you may have to push.
once they are in place inspect all of them are aligned. just look along columns and rows and see if any is out of place... if yes
just push them.
some switches do not fit tight (kailh for example) you will need to be creative and use a hardcover book or something to turn then
on the over side to solder.

![switches front](pics/switches_1.jpg)


![switches front](pics/switches_2.jpg)

now you can solder...

encoder
-------

![encoder front](pics/encoder_1.jpg)

![encoder soldered](pics/encoder_2.jpg)

