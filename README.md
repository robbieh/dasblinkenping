# dasblinkenping

ACHTUNG!  ALLES LOOKENSPEEPERS!

Dasblinkenping pings a block of IPs and displays a grid of round-trip times in
your terminal.

## What it looks like

The IPs are laid out on a Hilbert curve.

![dasblinkenping](../assets/dasblinkenping.gif?raw=true)


## Usage

    sudo dasblinkenping [cidr]... [ip]...

For example:

    sudo dasblinkenping 192.168.1.0/24

Press 'q' to quit.

Press 'h', 'j', 'k', or 'l' to move cursor. 'Esc' to clear the cursor.

Press 'n'ext or 'p'revious to move cursor to next/previous IP. This makes it easier to trace the Hilbert curve.

## Wanze (bugs) und Einschränkungen (limitations)

* CTRL-C doesn't work.
* Probably has other weird behaviors.

