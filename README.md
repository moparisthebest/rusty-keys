rusty-keys
==========

[![Build Status](https://ci.moparisthe.best/job/moparisthebest/job/rusty-keys/job/master/badge/icon%3Fstyle=plastic)](https://ci.moparisthe.best/job/moparisthebest/job/rusty-keys/job/master/)

low level keyboard mapper for linux and windows, with advanced caps lock and shift swapping behavior

This is the only keymapper I am aware of capable of implementing this layout, which I call Unix Programmer's Dvorak, which has been my daily driver since 2014:  
![Unix Programmer's Dvorak](https://www.moparisthebest.com/kbs/programmer-dvorak-NoSecondary-NumpadStandard-NoSwap-StandardNums-SwapAt-SwapPipe.svg)

The Problem
-----------
If you ever have mapped keys on linux, you know that there is the console keymap (loadkeys) and the X keymap (setxkbmap),
also things like SDL and Virtualbox grab the input directly and respect no maps.  Lastly I want to revert to QWERTY when
holding ctrl so ctrl+c works just like normal, without remapping all programs to ctrl+j.  Linux keymaps cannot do this either.

The Solution
------------
1. Grab a keyboard device directly so only we can read events from it.
2. Create a new keyboard input device with uinput, this looks identical to any other keyboard device to anything running on the box.
3. Read input_events from the real device, map them, send them to our created device.

This solution is what rusty-keys implements, it works in ttys, in X, in Wayland, in virtualbox even running windows or whatever,
on SDL games, it will work literally everywhere, because rusty-keys just creates a regular keyboard.

How to run
----------

When ran, it will read a keymap.toml configuration file, refer to example and tweak to suit.

```
Usage: rusty-keys [options] [device_files...]

Options:
    -h, --help          prints this help message
    -v, --version       prints the version
    -c, --config FILE   specify the keymap config file to use (default:
                        /etc/rusty-keys/keymap.toml)

```

when ran without specifying input devices, it maps all currently connected keyboards, and watches /dev/input/ with
inotify and starts mapping any new keyboards that are plugged in forever, until you kill it:
`rusty-keys`

or you can specify one or multiple input devices, and it will run until all are disconnected, then stop:  
`rusty-keys /dev/input/event0` or `rusty-keys /dev/input/event0 /dev/input/event2`

An example systemd service is in systemd/rusty-keys.service, enable it to have mapped keyboards all the time.

How to install
--------------
 * `cargo install rusty-keys`  
 * Arch Linux [rusty-keys](https://aur.archlinux.org/packages/rusty-keys/) [rusty-keys-git](https://aur.archlinux.org/packages/rusty-keys-git/)
 * Download a static binary for your system from the [releases](https://code.moparisthebest.com/moparisthebest/rusty-keys/releases) section. [github mirror](https://github.com/moparisthebest/rusty-keys/releases)

License
-------
AGPLv3 for now, message me if you have a problem with this

Notes
-----
Technically this is a re-implementation of a [previous](https://code.moparisthebest.com/moparisthebest/uinput-mapper/src/master/uinputmapper/keymapper.py) [python](https://code.moparisthebest.com/moparisthebest/uinput-mapper/src/master/keymaps/dvorak.py) [program](https://code.moparisthebest.com/moparisthebest/uinput-mapper/src/master/input-read#L151)
I had been using for 3 years previously.
