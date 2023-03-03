# ipcalc6
an ipv6 calculator built in rust that "looks and feel" like the ipcalc command on linux

```
$ ipcalc6
error : ipcalc6 requires 1 argument and you provided 0 arguments

        Usage :
                ipcalc6 [ipv6_address]
                ipcalc6 [ipv6_address]/[prefix]

        Example :
                ipcalc6 fe80::fcba:82ff:fe06:c2f1
                ipcalc6 fe80::fcba:82ff:fe06:c2f1/64
```

```
$ ipcalc6 2a00:1450:4007:807::2003/56

Type:           Unicast Global
Address:        2a00:1450:4007:0807:0000:0000:0000:2003         Prefix: 56
Hosts/Net:      75557863725914323419136

Address:        2a00:1450:4007:0807:0000:0000:0000:2003
Prefix:         ffff:ffff:ffff:ff00:0000:0000:0000:0000
HostMin:        2a00:1450:4007:0800:0000:0000:0000:0000
HostMax:        2a00:1450:4007:08ff:ffff:ffff:ffff:ffff

Address:        0010101000000000.0001010001010000.0100000000000111.0000100000000111.0000000000000000.0000000000000000.0000000000000000.0010000000000011
Prefix:         1111111111111111.1111111111111111.1111111111111111.1111111100000000.0000000000000000.0000000000000000.0000000000000000.0000000000000000
HostMin:        0010101000000000.0001010001010000.0100000000000111.0000100000000000.0000000000000000.0000000000000000.0000000000000000.0000000000000000
HostMax:        0010101000000000.0001010001010000.0100000000000111.0000100011111111.1111111111111111.1111111111111111.1111111111111111.1111111111111111

```

Limitation : although prefixes smaller than /48 are used for CIDR routing, they are not used on end device configuration and not supported by this tool.
