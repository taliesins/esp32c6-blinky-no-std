
# How to connect COM ports to WSL
## Use WSL kernel that support usb ip
To download custom kernel https://github.com/taliesins/WSL2-Linux-Kernel-Rolling/releases

Create or edit the file `%USERPROFILE%\.wslconfig` with the following content:
 

```
[wsl2]
kernel=C:\\bzImage
```

Open a PowerShell terminal window as Administrator

Stop the WSL instance:
PowerShell
```
wsl --shutdown
```

## Install usbip-win onto Windows host
To download windows cli: https://github.com/dorssel/usbipd-win/releases

## Attach serial and jtag ports of microcontroller to WSL

Instruction to get COM port forwarded to WSL https://github.com/dorssel/usbipd-win/wiki/WSL-support

See the devices that are available:
```
usbipd list
```

```
C:\WINDOWS\system32>usbipd list
Connected:
BUSID  VID:PID    DEVICE                                                        STATE
2-5    8087:0032  Intel(R) Wireless Bluetooth(R)                                Not shared
2-7    13d3:56d5  Integrated Camera, Integrated IR Camera                       Not shared
2-8    1532:0245  USB Input Device, Razer Blade                                 Not shared
4-2    1a86:55d3  USB-Enhanced-SERIAL CH343 (COM3)                              Not shared
4-4    303a:1001  USB Serial Device (COM4), USB JTAG/serial debug unit          Not shared

Persisted:
GUID                                  DEVICE
```

To attach to serial port of microcontroller:
```
usbipd bind --busid 4-2 
usbipd attach --wsl --busid=4-2 
```

```
C:\WINDOWS\system32>usbipd bind --busid 4-2

C:\WINDOWS\system32>usbipd attach --wsl --busid=4-2
usbipd: info: Using WSL distribution 'Ubuntu-22.04' to attach; the device will be available in all WSL 2 distributions.
usbipd: info: Using IP address 10.152.0.1 to reach the host.
```

To attach to jtag port of microcontroller:
```
usbipd bind --busid 4-4
usbipd attach --wsl --busid=4-4
```

```
C:\WINDOWS\system32>usbipd bind --busid 4-4

C:\WINDOWS\system32>usbipd attach --wsl --busid=4-4
usbipd: info: Using WSL distribution 'Ubuntu-22.04' to attach; the device will be available in all WSL 2 distributions.
usbipd: info: Using IP address 10.152.0.1 to reach the host.
```

See if the ports are attached to wsl:
```
usbipd list
```

```
C:\WINDOWS\system32>usbipd list
Connected:
BUSID  VID:PID    DEVICE                                                        STATE
2-5    8087:0032  Intel(R) Wireless Bluetooth(R)                                Not shared
2-7    13d3:56d5  Integrated Camera, Integrated IR Camera                       Not shared
2-8    1532:0245  USB Input Device, Razer Blade                                 Not shared
4-2    1a86:55d3  USB-Enhanced-SERIAL CH343 (COM3)                              Attached
4-4    303a:1001  USB Serial Device (COM4), USB JTAG/serial debug unit          Attached

Persisted:
GUID                                  DEVICE

```

See ports inside WSL:
```
lsusb
```

```
taliesins@tali-laptop:~$ lsusb
Bus 002 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
Bus 001 Device 003: ID 303a:1001 Espressif USB JTAG/serial debug unit
Bus 001 Device 002: ID 1a86:55d3 QinHeng Electronics USB Single Serial
Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
```