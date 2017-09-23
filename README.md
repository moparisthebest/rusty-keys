rusty-keys
======
uinput level keyboard mapper for linux, with advanced caps lock and shift swapping behavior

This is the only keymapper I am aware of capable of implementing this layout:  
![Unix Programmer's Dvorak](https://www.moparisthebest.com/kbs/programmer-dvorak-NoSecondary-NumpadStandard-NoSwap-StandardNums-SwapAt-SwapPipe.svg)

The Problem
-----------
If you ever have mapped keys on linux, you know that there is the console keymap (loadkeys) and the X keymap (setxkbmap)
also things like SDL and Virtualbox grab the input directly and respect no maps.  Lastly I want to revert to QWERTY when
holding ctrl so ctrl+c works just like normal, without remapping all programs to ctrl+j.  Linux keymaps cannot do this either.

The Solution
------------
1. Grab a keyboard device directly so only we can read events from it.
2. Create a new keyboard input device with uinput, this is identical to any other keyboard device to anything running on the box.
3. Read input_events from real device, map them, send them to our created device.

This solution is what rusty-keys implements, it works in ttys, in X, in virtualbox even running windows or whatever,
on SDL games, it will work literally everywhere, because rusty-keys just creates a regular keyboard.

How to run
----------

When ran, it will read a keymap.toml file from your current working directory, refer to example and tweak to suit.

```
Usage: rusty-keys [options]

Options:
    -h, --help          prints this help message
    -v, --version       prints the version
    -d, --device DEVICE specify the keyboard input device file
    -c, --config FILE   specify the keymap config file to use
```

with only one keyboard attached:  
`rusty-keys`

with multiple keyboards, currently you must specify one:  
`rusty-keys -d /dev/input/event0`

find all eligible keyboard devices like:  
`grep -E 'Handlers|EV' /proc/bus/input/devices | grep -B1 120013 | grep -Eo event[0-9]+`

For using the systemd unit with by-id or by-path:
```
$ systemd-escape --template=rusty-keys@.service by-id/usb-04c8_USB_Keyboard-event-kbd
rusty-keys@by\x2did-usb\x2d04c8_USB_Keyboard\x2devent\x2dkbd.service
```

How to install
--------------
 * `cargo install rusty-keys`  
 * Arch Linux [AUR PKGBUILD](https://aur.archlinux.org/packages/rusty-keys/)

License
-------
AGPLv3 for now, message me if you have a problem with this

Notes
-----
Technically this is a re-implementation of a [previous](https://code.moparisthebest.com/moparisthebest/uinput-mapper/src/master/uinputmapper/keymapper.py) [python](https://code.moparisthebest.com/moparisthebest/uinput-mapper/src/master/keymaps/dvorak.py) [program](https://code.moparisthebest.com/moparisthebest/uinput-mapper/src/master/input-read#L151)
I had been using for 3 years previously.
