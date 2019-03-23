# dasblinkenping

ACHTUNG!  ALLES LOOKENSPEEPERS!

Dasblinkenping pings a block of IPs and displays a grid of round-trip times in
your terminal.

## What it looks like


```
⋅○⋅⋅⋅⋅⋅⋅⋅⋅⋅⊚⊚⊚⋅⊚
⋅●●⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⊚⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⊚⊚●⊚⋅⋅⋅⋅⋅⋅⊚⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅●⊚⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅
⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⋅⊚⋅⋅⋅
```

## Usage

    sudo dasblinkenping [cidr]... [ip]...

For example:

    sudo dasblinkenping 192.168.1.0/24

Press 'q' to quit.

## Wanze (bugs) und Einschränkungen (limitations)

* CTRL-C doesn't work.
* Not ipv6 compliant.
* Chews up a lot of CPU.

