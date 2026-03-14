[Skip to content](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#start-of-content)

Open menu

-   [Home](https://github.com/dashboard)
-   [Issues](https://github.com/issues)
-   [Pull requests](https://github.com/pulls)

Homepage (g then d) gGthen dD

1.  [linux-surface](https://github.com/linux-surface)
2.  [linux-surface](https://github.com/linux-surface/linux-surface)

Type / to search

Search or jump to…(s,/) s,/S,/

# Search code, repositories, users, issues, pull requests...

Search

Clear

0 suggestions.

[Search syntax tips](https://docs.github.com/search-github/github-code-search/understanding-github-code-search-syntax)

Give feedback

# Provide feedback

We read every piece of feedback, and take your input very seriously.

 Include my email address so I can be contacted

Cancel Submit feedback

# Saved searches
## Use saved searches to filter your results more quickly

Name  

Query 

To see all available qualifiers, see our [documentation](https://docs.github.com/search-github/github-code-search/understanding-github-code-search-syntax).

Cancel Create saved search

Chat with Copilot

Create new...IssuesPull requestsRepositories

You have no unread notifications(g then n) gGthen nN

![河原田陸渡](https://avatars.githubusercontent.com/u/46557056?v=4&size=64)Open user navigation menu

## Repository navigation

-   [Code](https://github.com/linux-surface/linux-surface)
-   [Issues764 (764)](https://github.com/linux-surface/linux-surface/issues)
-   [Pull requests4 (4)](https://github.com/linux-surface/linux-surface/pulls)
-   [Agents](https://github.com/linux-surface/linux-surface/agents?author=Yourumark)
-   [Discussions](https://github.com/linux-surface/linux-surface/discussions)
-   [Actions](https://github.com/linux-surface/linux-surface/actions)
-   [Projects](https://github.com/linux-surface/linux-surface/projects)
-   [Wiki](https://github.com/linux-surface/linux-surface/wiki)
-   [Security](https://github.com/linux-surface/linux-surface/security)
-   [Insights](https://github.com/linux-surface/linux-surface/pulse)

# Search code, repositories, users, issues, pull requests...

Search

Clear

0 suggestions.

[Search syntax tips](https://docs.github.com/search-github/github-code-search/understanding-github-code-search-syntax)

Give feedback

# Provide feedback

We read every piece of feedback, and take your input very seriously.

 Include my email address so I can be contacted

Cancel Submit feedback

# Saved searches
## Use saved searches to filter your results more quickly

Name  

Query 

To see all available qualifiers, see our [documentation](https://docs.github.com/search-github/github-code-search/understanding-github-code-search-syntax).

Cancel Create saved search

You signed in with another tab or window. [Reload]() to refresh your session. You signed out in another tab or window. [Reload]() to refresh your session. You switched accounts on another tab or window. [Reload]() to refresh your session. Dismiss alert

# Camera Support

[Jump to bottom](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#wiki-pages-box) [Edit](https://github.com/linux-surface/linux-surface/wiki/Camera-Support/_edit) [New page](https://github.com/linux-surface/linux-surface/wiki/_new)

nekokiwa edited this page Mar 9, 2026 · [91 revisions](https://github.com/linux-surface/linux-surface/wiki/Camera-Support/_history)

Running the cameras on Surface is a work in progress.  
Check out [https://github.com/linux-surface/linux-surface/issues/91#issuecomment-1917874317](https://github.com/linux-surface/linux-surface/issues/91#issuecomment-1917874317) for links to ongoing discussions.

Please note that where your camera is marked as working (✅), the image quality is not expected to be perfect yet. The algorithms are still undergoing development, and need some camera specific tuning.

This page will detail the steps required to be able to get the camera working on your device, with the current latest support.

If you have successfully enabled your camera on a device not yet in this table, please update it:

| Device | ISP | Front | Back | IR |
| --- | --- | --- | --- | --- |
| Surface Go | IPU3 | OV5693 ✅ | OV8865 ✅ | OV7251 🚫 |
| Surface Go 2 | IPU3 | OV5693 ✅ | OV8865 ✅ | OV7251 🚫 |
| Surface Go 3 | IPU3 | OV5693 ✅ | OV8865 ✅ | OV7251 🚫 |
| Surface Go 4 | ? | ? | ? | ? |
| Surface Pro 4 | IPU3 | OV5693 ✅ | OV8865 ✅ | OV7251 🚫 |
| Surface Pro 5 | IPU3 | OV5693 ✅ | OV8865 ✅ | OV7251 🚫 |
| Surface Pro 6 | IPU3 | OV5693 ✅ | OV8865 ✅ | OV7251 🚫 |
| Surface Pro 7 | IPU4 | OV5693 🚫 | OV8865 🚫 | OV7251 🚫 |
| Surface Pro 7+ | IPU6 | OV5693 🚫 | OV8865 🚫 | OV7251 🚫 |
| Surface Pro 8 | IPU6 | OV5693 🚫 | OV13858 ✅ | VD55G0 🚫 |
| Surface Pro 9 (Intel) | IPU6 | OV5693 🚫 | OV13858 ✅ | VD55G0 (?) 🚫 |
| Surface Pro 10 | IPU6 | IMX681 🚫 | OV13858 🚫 | VD55G0 🚫 |
| Surface Pro 11 (Intel | IPU6 | IMX681 🚫 | OV13858 🚫 | VD55G0 🚫 |
| Surface Book 1 | IPU3 | OV5693 ✅ | OV8865 ✅ | OV7251 🚫 |
| Surface Book 2 13" | IPU3 | OV5693 ✅ | OV8865 ✅ | OV7251 🚫 |
| Surface Book 3 | IPU4 | OV5693 🚫 | OV8865 🚫 | OV7251 🚫 |
| Surface Laptop 1 | IPU3 | OV9734 🚫 | N/A | OV7251 🚫 |
| Surface Laptop 2 | IPU3 | OV9734 🚫 | N/A | OV7251 🚫 |
| Surface Laptop 3 (Intel) | IPU4 | OV9734 🚫 | N/A | OV7251 🚫 |
| Surface Laptop 3 (AMD) | (USB/UVC) | OV5693 ✅ | N/A | OV7251 ✅ |
| Surface Laptop 4 | (USB/UVC) | ✅ | N/A | ? |
| Surface Laptop 5 | (USB/UVC) | ? | N/A | ? |
| Surface Laptop Studio | (USB/UVC) | ✅ | N/A | ? |
| Surface Laptop Studio 2 | (USB/UVC) | ? | N/A | ? |

**The Surface Pro 7, Surface Laptop 3 (Intel) and the Surface Book 3 use an IPU4 which can not be supported at this time**

# Kernel Support

Kernel patches and drivers are required to be built to support your device. The best way to obtain these for a Surface device is to use one of the binary releases provided by this repository. Alternatively, you can also build a kernel yourself and apply the `cameras.patch` found in this repository under `patches/<version>/cameras.patch` (e.g. `patches/v5.12/cameras.patch`).

Supporting the Camera on the Surface range requires extending the Intel IPU3 driver to support the Windows based ACPI definitions. This is being handled by an extension to the IPU3-CIO2 driver called the CIO2-Bridge.

### Device-Specific Fixes

#### Surface Pro 5 - Rear Camera (OV8865)

Some Surface Pro 5 units have a Microsoft BIOS bug where GPIO 151 is misconfigured in the DSDT as "Privacy LED" (0x0d) instead of "Power Enable" (0x0b). This causes the rear camera (OV8865) to not receive power and fail to initialize.

**Symptoms:**

-   Front camera works, rear camera doesn't
-   `dmesg` shows `ov8865` probe failures
-   No `/dev/video*` device for rear camera

**Fix:** [https://github.com/Eberhar3/surface-pro5-kamera-fix](https://github.com/Eberhar3/surface-pro5-kamera-fix)

The fix combines the `intel_skl_int3472` kernel modules with a quirk that corrects the GPIO function mapping.

> ⚠️ Documentation is in German 🇩🇪 — use Google Translate or learn German. Your choice.

### IPU3 Firmware

The IPU3 used by the Surface requires some firmware to support the devices which is loaded at runtime.

On Ubuntu-based systems, this firmware is provided by the 'linux-firmware' package.

For Fedora the firmware is provided by the `intel-vsc-firmware` package.

According to [this response](https://github.com/linux-surface/linux-surface/issues/91#issuecomment-987431880), for Debian, the ipu3 firmware is provided by `firmware-misc-nonfree` from the nonfree repo. The .bin file is named `irci_irci_ecr-master_20161208_0213_20170112_1500.bin` and must be renamed and moved to `/lib/firmware/intel/ipu3-fw.bin`.

The file:

**/lib/firmware/intel/ipu3-fw.bin**

is required to be present at kernel boot time to support loading of the IPU3 IMGU device.

# libcamera Support

To test the cameras on the Surface devices we need [libcamera](https://libcamera.org) to provide the userspace support required to operate the cameras.

### Install libcamera from repositories

Some flavours of Linux have repositories containing versions of libcamera, which you can install without building libcamera from source.

#### Ubuntu

Use apt to install libcamera packages:

```
sudo apt install libcamera0.2 gstreamer1.0-libcamera libcamera-ipa pipewire-libcamera libcamera-tools
```

#### Fedora

It has been reported that, on Fedora 36, as long as you have linux-surface kernel 5.18 or newer installed, you should be able to install libcamera components via `dnf`:

```
sudo dnf install libcamera libcamera-tools libcamera-qcam libcamera-gstreamer libcamera-ipa pipewire-plugin-libcamera
```

If you're using an immutable variant of Fedora (Silverblue, Kinoite,Sericea, etc), you should use `rpm-ostree`:

```
rpm-ostree install libcamera libcamera-tools libcamera-qcam libcamera-gstreamer libcamera-ipa pipewire-plugin-libcamera
```

#### Arch Linux

Arch now ships a stable `libcamera` package (`pacman -S libcamera`). You may also want to install the following packages:

-   `libcamera-tools` for `cam` and `qcam`
-   `gst-plugin-libcamera` for the gstreamer libcamera plugin (required to use applications that don't directly support libcamera)

### Build libcamera from the latest git source

Both clang and gcc are supported compilers.

First install the required dependencies (e.g. on Debian/Ubuntu, **a '\\' denotes the line is continued**)

```
$ sudo apt install \
    build-essential meson ninja-build pkg-config libgnutls28-dev openssl \
    python3-pip python3-yaml python3-ply python3-jinja2 \
    qtbase5-dev libqt5core5a libqt5gui5 libqt5widgets5 qttools5-dev-tools \
    libtiff-dev libevent-dev libyaml-dev \
    gstreamer1.0-tools libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev
```

And now obtain and build libcamera:

```
$ sudo apt install git
$ git clone https://git.libcamera.org/libcamera/libcamera.git
$ cd libcamera
```

Configure libcamera minimally for the Surface

```
$ meson build -Dpipelines=uvcvideo,vimc,ipu3 -Dipas=vimc,ipu3 -Dprefix=/usr -Dgstreamer=enabled -Dv4l2=true -Dbuildtype=release
$ ninja -C build
$ sudo ninja -C build install
```

#### Troubleshooting

Running on a fresh system, I needed to run this, and it doesn't hurt to be sure (_but shouldn't be needed_):

`$ ldconfig`

If your distro doesn't provide meson>=0.55, you can use the following methods to install the latest meson for building:

##### From PyPI

```
$ sudo pip3 install -U meson
```

Usually you shouldn't use `sudo pip` because it might conflict with your distro's package manager. However, it is a [documented method](https://mesonbuild.com/Getting-meson.html#installing-meson-with-pip) to install meson. And if you have different versions installed for user and system-wide, or only install meson for user, `sudo ninja -C build install` might not work.

##### From MPR

[MPR](https://mpr.hunterwittenborn.com/) is like AUR for Debian/Ubuntu. To use it, you need to install [makedeb](https://docs.hunterwittenborn.com/makedeb/).

After installing makedeb, pull meson's PKGBUILD from MBR:

```
$ git clone https://mpr.hunterwittenborn.com/meson.git
$ cd meson
$ cat PKGBUILD  # Have a look at the PKGBUILD
```

If your apt version is lower than 1.9, makedeb cannot install the dependencies for you, and you'll need to install the packages in `makedepends` array manually. Then build and install the package:

```
$ makedeb -si
```

Alternatively, use the method above to install MPR helper [tap](https://mpr.hunterwittenborn.com/packages/tap/), and install meson through tap:

```
$ tap install meson
```

### Ensure your user account has permissions

Make sure your user is part of the 'video' group. (you only need to do this once on the machine)

```
$ sudo usermod -aG video $USER
$ newgrp video
```

### Test with 'cam'

Now you can run cam --list and you should see the sensor listed

```
$ cam --list
Available cameras:
1: Internal front camera (\*SB*.PCI0.LNK1)
```

and running qcam should (all being well) get you a view of the video capture.

`$ qcam`

# Applications

## Cheese

Cheese uses Gstreamer to read from cameras, and should work as long as the gstreamer libcamerasrc plugin is correctly built and installed.

## Firefox

Since version 116, Firefox added experimental support to pipewire camera. Navigate to `about:config`, enable `media.webrtc.camera.allow-pipewire` and restart Firefox. Make sure the plugin `pipewire-plugin-libcamera` is installed in your distribution.

## GStreamer Loopback Device for other Applications

Right now only applications using libcamera (directly or indirectly, e.g. via the gstreamer plugin) are supported. It is, however, possible to use gstreamer to create a loopback device. To do this, you have to install `v4l2loopback-dkms` (if you're using Secure Boot you have to also follow this guide: [https://github.com/umlaeute/v4l2loopback/wiki/Secure-Boot](https://github.com/umlaeute/v4l2loopback/wiki/Secure-Boot)) and run

```
sudo modprobe v4l2loopback video_nr=42 card_label="virtualcam" exclusive_caps=1 max_buffers=4
```

to set up the kernel module and

```
gst-launch-1.0 libcamerasrc camera-name='\\\_SB_.PCI0.I2C2.CAMF' ! \
    video/x-raw,width=1280,height=720,framerate=30/1,format=NV12 \
    ! videoconvert ! video/x-raw,format=YUY2 ! queue ! \
    v4l2sink device=/dev/video42
```

to create the loopback device. Note that you may need to change `camera-name`, `v4l2sink device`, and/or other parameters for your setup.

If issues occur, you maybe need to remove `queue !` from the command:

```
gst-launch-1.0 libcamerasrc camera-name='\\\_SB_.PCI0.I2C2.CAMF' ! \
    video/x-raw,width=1280,height=720,framerate=30/1,format=NV12 \
    ! videoconvert ! video/x-raw,format=YUY2 ! \
    v4l2sink device=/dev/video42
```

This guide also explains how to automatically load v4l2loopback on startup and how to create a terminal command to start the cameras quicker: [https://neilzone.co.uk/2021/08/working-front-and-rear-cameras-on-debian-11-on-a-surface-pro-6-surfacebook-2-and-surface-go/](https://neilzone.co.uk/2021/08/working-front-and-rear-cameras-on-debian-11-on-a-surface-pro-6-surfacebook-2-and-surface-go/)

# Development Information

## Module IDs and Sensor Mappings

| Device | ISP | Front Sensor | Front Module | Rear Sensor | Rear Module | IR Sensor | IR Module |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SGo1 | IPU3 | OV5693 | YHCU | OV8865 | YHCT | OV7251 | YHCV |
| SGo2/3 | IPU3 | OV5693 | YHCU | OV8865 | YHCT | OV7251 | YHCV |
| SB1 | IPU3 | OV5693 | MSHW0070 | OV8865 | MSHW0071 | OV7251 | MSHW0072 |
| SB2 13" | IPU3 | OV5693 | MSHW0140 | OV8865 | MSHW0141 | OV7251 | MSHW0142 |
| SB2 15" | IPU3 | OV5693 | MSHW0150 | OV8865 | MSHW0151 | OV7251 | MSHW0152 |
| SB3 13" | IPU4 | OV5693 | MSHW0210 | OV8865 | MSHW0211 | OV7251 | MSHW0212 |
| SB3 15" | IPU4 | OV5693 | MSHW0200 | OV8865 | MSHW0201 | OV7251 | MSHW0202 |
| SL1 | IPU3 | OV9734 | MSHW0073 | - | - | OV7251 | MSHW0074 |
| SL2 | IPU3 | OV9734 | MSHW0073 | - | - | OV7251 | MSHW0074 |
| SP4 | IPU3 | OV5693 | MSHW0070 | OV8865 | MSHW0071 | OV7251 | MSHW0072 |
| SP5 | IPU3 | OV5693 | MSHW0120 | OV8865 | MSHW0121 | OV7251 | MSHW0122 |
| SP6 | IPU3 | OV5693 | MSHW0120 | OV8865 | MSHW0121 | OV7251 | MSHW0122 |
| SP7 | IPU4 | OV5693 | MSHW0190 | OV8865 | MSHW0191 | OV7251 | MSHW0192 |

Note: SL3 and SLGo1 use generic USB cameras and work out of the box.

## ACPI Sensor IDs

| Sensor ID | ACPI ID |
| --- | --- |
| AR0330 | APTA0330 |
| IMX135 | INT3471 |
| OV2680 | OVTI2680 |
| OV2740 | INT3474 |
| OV5648 | OVTI5648 |
| OV5693 | INT33BE |
| OV7251 | INT347E |
| OV8835 | OV8835, OVTI8835 |
| OV8865 | INT347A |
| OV9734 | OVTI9734 |

## Reverse engineering tips

For some of the work to build camera support, reverse engineering the settings made by the Windows drivers for either the camera devices or their supporting ICs is necessary. One way to do this is to talk to the i2c devices directly and read the registers to discover the settings. A mandatory first step is informing Windows that direct user access to those devices is allowed.

To perform that operation you will first need the [microsoft asl compiler tool](https://docs.microsoft.com/en-gb/windows-hardware/drivers/bringup/microsoft-asl-compiler). This lets you extract and insert modified DSDT tables in Windows.

Once you've got that tool built, you need to edit the DSDT table to tell Windows to expose the I2C device for the PMIC. Run `asl.exe /tab=DSDT` to get a file named DSDT.asl in the current directory. You then need to make edits [following this guide](https://docs.microsoft.com/en-us/windows/uwp/devices-sensors/enable-usermode-access) to add a section to the DSDT that creates an MSFT8000 dummy device and adds an [I2cSerialBus](https://docs.microsoft.com/en-us/windows/uwp/devices-sensors/enable-usermode-access#i2c) entry describing the I2C Bus that the target device sits on, for example:

```
Device(RHPX)
{
    Name(\_HID, "MSFT8000")
    Name(\_CID, "MSFT8000")
    Name(\_UID, 1)

    Name(\_CRS, ResourceTemplate()
    {
        I2CSerialBus(0xFFFF, , 0, , "\\\\\_SB.PCI0.I2C2", , ,) // in this case, we want to grant access to the I2C2 bus
    })

    Name(\_DSD, Package()
    {
        ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
        Package()
        {
            Package(2) { "bus-I2C-I2C2", Package() { 0 }}, // The 0 here is the index of the related resource in \_CRS
        }
    })
}
```

**You also need to increase the version number of the table in the \`DefinitionBlock() or the edits will be ignored**

Once that's done, compile the edited DSDT.asl using `asl.exe DSDT.ASL`. If it throws compiler errors you'll have to fix them. If you can't get it working using this tool you can actually boot Linux and try using iasl, which is Intel's equivalent and seems to work a lot better. You'd have to transfer the compiled file back to Windows somehow.

Once the file's compiled, you insert it using `asl.exe /loadtable -v DSDT.AML`. Pay attention to the output of the command as you may need to enable test signing mode (`bcdedit /set testsigning on`) and disable secure boot. Finally, you need to reboot.

To talk to the I2C devices themselves, use the [I2cTestTool](https://github.com/microsoft/Windows-iotcore-samples/tree/master/BusTools/I2cTestTool). Build that and run `I2cTestTool <<addr>> <<busname>>` to connect to the device. For example to connect to the device at 0x4d on I2C bus 2 the command would be `I2cTestTool 0x4d I2C2`. You can then perform read and write operations over I2C. To read a register value you use the `writeread` command, sending the address of the register followed by the number of registers you want to read, for example `writeread {30 0a} 2` to read 0x300a and 0x300b, which on Omnivision sensors is usually the chip's ID.

### I2C Workaround

_**The I2C workaround should not be required any more. This is fixed in the linux-surface kernels, and the fix should be heading upstream to other distribution kernels. This section is kept for reference only.**_

On a Surface Go 2 a workaround is required to enable the front camera support: The kernel needs to have the following command line parameter added to the boot arguments.

**acpi\_enforce\_resources=lax**

Where to put this is quite platform specific, however continuing the assumption that an Ubuntu or Debian based system is in use, we can infer that the system uses grub.

**On the Surface Go device itself,** edit the file "**/etc/default/grub"** and update the **GRUB\_CMDLINE\_LINUX\_DEFAULT** entry to add the workaround:

```
 -GRUB_CMDLINE_LINUX_DEFAULT="quiet splash"
 +GRUB_CMDLINE_LINUX_DEFAULT="quiet splash acpi_enforce_resources=lax"
```

[Install Instructions](https://github.com/linux-surface/linux-surface/wiki/Installation-and-Setup) | [Feature Matrix](https://github.com/linux-surface/linux-surface/wiki/Supported-Devices-and-Features)

## Toggle table of contents Pages 53

-   Loading[Home](https://github.com/linux-surface/linux-surface/wiki)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Camera Support](https://github.com/linux-surface/linux-surface/wiki/Camera-Support)
    
    -   [Kernel Support](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#kernel-support)
    -   [Device-Specific Fixes](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#device-specific-fixes)
    -   [Surface Pro 5 - Rear Camera (OV8865)](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#surface-pro-5---rear-camera-ov8865)
    -   [IPU3 Firmware](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#ipu3-firmware)
    -   [libcamera Support](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#libcamera-support)
    -   [Install libcamera from repositories](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#install-libcamera-from-repositories)
    -   [Ubuntu](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#ubuntu)
    -   [Fedora](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#fedora)
    -   [Arch Linux](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#arch-linux)
    -   [Build libcamera from the latest git source](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#build-libcamera-from-the-latest-git-source)
    -   [Troubleshooting](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#troubleshooting)
    -   [From PyPI](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#from-pypi)
    -   [From MPR](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#from-mpr)
    -   [Ensure your user account has permissions](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#ensure-your-user-account-has-permissions)
    -   [Test with 'cam'](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#test-with-cam)
    -   [Applications](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#applications)
    -   [Cheese](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#cheese)
    -   [Firefox](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#firefox)
    -   [GStreamer Loopback Device for other Applications](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#gstreamer-loopback-device-for-other-applications)
    -   [Development Information](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#development-information)
    -   [Module IDs and Sensor Mappings](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#module-ids-and-sensor-mappings)
    -   [ACPI Sensor IDs](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#acpi-sensor-ids)
    -   [Reverse engineering tips](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#reverse-engineering-tips)
    -   [I2C Workaround](https://github.com/linux-surface/linux-surface/wiki/Camera-Support#i2c-workaround)
    
-   Loading[Compiling the Kernel from Source](https://github.com/linux-surface/linux-surface/wiki/Compiling-the-Kernel-from-Source)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Disk Encryption](https://github.com/linux-surface/linux-surface/wiki/Disk-Encryption)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Gaming configuration for Surface Pro and Go devices](https://github.com/linux-surface/linux-surface/wiki/Gaming-configuration-for-Surface-Pro-and-Go-devices)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Installation and Setup](https://github.com/linux-surface/linux-surface/wiki/Installation-and-Setup)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Intel 12th Generation Devices Issues](https://github.com/linux-surface/linux-surface/wiki/Intel-12th-Generation-Devices-Issues)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Known Issues](https://github.com/linux-surface/linux-surface/wiki/Known-Issues)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Known Issues and FAQ](https://github.com/linux-surface/linux-surface/wiki/Known-Issues-and-FAQ)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[libcamera Chromium support (deprecated)](https://github.com/linux-surface/linux-surface/wiki/libcamera-Chromium-support-(deprecated))
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Links and Associated Projects](https://github.com/linux-surface/linux-surface/wiki/Links-and-Associated-Projects)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[LTE modem](https://github.com/linux-surface/linux-surface/wiki/LTE-modem)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Marvell 88W8897 quirks](https://github.com/linux-surface/linux-surface/wiki/Marvell-88W8897-quirks)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Nvidia GPU](https://github.com/linux-surface/linux-surface/wiki/Nvidia-GPU)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Package Repositories](https://github.com/linux-surface/linux-surface/wiki/Package-Repositories)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Pen DFT data format](https://github.com/linux-surface/linux-surface/wiki/Pen-DFT-data-format)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Secure Boot](https://github.com/linux-surface/linux-surface/wiki/Secure-Boot)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Supported Devices and Features](https://github.com/linux-surface/linux-surface/wiki/Supported-Devices-and-Features)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface 3](https://github.com/linux-surface/linux-surface/wiki/Surface-3)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Book](https://github.com/linux-surface/linux-surface/wiki/Surface-Book)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Book 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Book-2)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Book 3](https://github.com/linux-surface/linux-surface/wiki/Surface-Book-3)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Dock](https://github.com/linux-surface/linux-surface/wiki/Surface-Dock)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Go](https://github.com/linux-surface/linux-surface/wiki/Surface-Go)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Go 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Go-2)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Go 3](https://github.com/linux-surface/linux-surface/wiki/Surface-Go-3)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Go 4](https://github.com/linux-surface/linux-surface/wiki/Surface-Go-4)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-2)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop 3](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-3)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop 4](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-4)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop 5](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-5)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop 6](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-6)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop Go](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-Go)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop Go 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-Go-2)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop Go 3](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-Go-3)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop Studio](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-Studio)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Laptop Studio 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-Studio-2)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Pro 11](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-11)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Pro 3](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-3)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Pro 4](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-4)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Pro 5 taming the LTE beast](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-5-taming-the-LTE-beast)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Pro 6](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-6)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Pro 7](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-7)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Pro 7 plus](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-7-plus)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Pro 8](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-8)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Pro 9](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-9)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Surface Studio 2 plus](https://github.com/linux-surface/linux-surface/wiki/Surface-Studio-2-plus)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Systemd](https://github.com/linux-surface/linux-surface/wiki/Systemd)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Testers and Contributing](https://github.com/linux-surface/linux-surface/wiki/Testers-and-Contributing)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Thermald setup and configuration](https://github.com/linux-surface/linux-surface/wiki/Thermald-setup-and-configuration)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[UEFI](https://github.com/linux-surface/linux-surface/wiki/UEFI)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Loading[Utilities and Packages](https://github.com/linux-surface/linux-surface/wiki/Utilities-and-Packages)
    
    ### Uh oh!
    
    There was an error while loading. [Please reload this page]().
    
-   Show 38 more pages…

### Table of Contents

-   [Home](https://github.com/linux-surface/linux-surface/wiki/Home)
-   [Installation and Setup](https://github.com/linux-surface/linux-surface/wiki/Installation-and-Setup)
    -   [Package Repositories](https://github.com/linux-surface/linux-surface/wiki/Package-Repositories)
    -   [Compiling the Kernel from Source](https://github.com/linux-surface/linux-surface/wiki/Compiling-the-Kernel-from-Source)
    -   [Secure Boot](https://github.com/linux-surface/linux-surface/wiki/Secure-Boot)
    -   [Utilities and Packages](https://github.com/linux-surface/linux-surface/wiki/Utilities-and-Packages)
-   [Supported Devices and Features](https://github.com/linux-surface/linux-surface/wiki/Supported-Devices-and-Features)
    -   [Surface Book 1](https://github.com/linux-surface/linux-surface/wiki/Surface-Book)
    -   [Surface Book 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Book-2)
    -   [Surface Book 3](https://github.com/linux-surface/linux-surface/wiki/Surface-Book-3)
    -   [Surface Laptop Studio](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-Studio)
    -   [Surface Laptop Studio 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-Studio-2)
    -   [Surface Laptop 1](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop)
    -   [Surface Laptop 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-2)
    -   [Surface Laptop 3](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-3)
    -   [Surface Laptop 4](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-4)
    -   [Surface Laptop 5](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-5)
    -   [Surface Laptop 6](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-6)
    -   [Surface Laptop Go 1](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-Go)
    -   [Surface Laptop Go 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-Go-2)
    -   [Surface Laptop Go 3](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-Go-3)
    -   [Surface Laptop SE](https://github.com/linux-surface/linux-surface/wiki/Surface-Laptop-SE)
    -   [Surface Go 1](https://github.com/linux-surface/linux-surface/wiki/Surface-Go)
    -   [Surface Go 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Go-2)
    -   [Surface Go 3](https://github.com/linux-surface/linux-surface/wiki/Surface-Go-3)
    -   [Surface Go 4](https://github.com/linux-surface/linux-surface/wiki/Surface-Go-4)
    -   [Surface 3](https://github.com/linux-surface/linux-surface/wiki/Surface-3)
    -   [Surface Pro 1](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-1)
    -   [Surface Pro 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-2)
    -   [Surface Pro 3](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-3)
    -   [Surface Pro 4](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-4)
    -   [Surface Pro 5](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-5)
    -   [Surface Pro 6](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-6)
    -   [Surface Pro 7](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-7)
    -   [Surface Pro 7+](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-7-plus)
    -   [Surface Pro 8](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-8)
    -   [Surface Pro 9 (Intel)](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-9)
    -   [Surface Pro 10](https://github.com/linux-surface/linux-surface/wiki/Surface-Pro-10)
    -   [Surface Studio 1](https://github.com/linux-surface/linux-surface/wiki/Surface-Studio)
    -   [Surface Studio 2](https://github.com/linux-surface/linux-surface/wiki/Surface-Studio-2)
    -   [Surface Studio 2+](https://github.com/linux-surface/linux-surface/wiki/Surface-Studio-2-plus)
    -   [Surface Dock](https://github.com/linux-surface/linux-surface/wiki/Surface-Dock)
-   Device Independent Information
    -   [Intel 12th Generation Devices Issues](https://github.com/linux-surface/linux-surface/wiki/Intel-12th-Generation-Devices-Issues)
    -   [Camera Support](https://github.com/linux-surface/linux-surface/wiki/Camera-Support)
        -   [Compiling Chromium with camera support (deprecated)](https://github.com/linux-surface/linux-surface/wiki/libcamera-Chromium-support-%28deprecated%29)
    -   [Disk Encryption](https://github.com/linux-surface/linux-surface/wiki/Disk-Encryption)
    -   [Marvell 88W8897 quirks](https://github.com/linux-surface/linux-surface/wiki/Marvell-88W8897-quirks)
    -   [UEFI](https://github.com/linux-surface/linux-surface/wiki/UEFI)
    -   [Systemd](https://github.com/linux-surface/linux-surface/wiki/Systemd)
    -   [Thermald setup and configuration](https://github.com/linux-surface/linux-surface/wiki/Thermald-setup-and-configuration)
    -   [Gaming configuration for Surface Pro and Go devices](https://github.com/linux-surface/linux-surface/wiki/Gaming-configuration-for-Surface-Pro-and-Go-devices)
-   [Known Issues / FAQ](https://github.com/linux-surface/linux-surface/wiki/Known-Issues-and-FAQ)
-   [Links and Associated Projects](https://github.com/linux-surface/linux-surface/wiki/Links-and-Associated-Projects)
-   [Testers and Contributing](https://github.com/linux-surface/linux-surface/wiki/Testers-and-Contributing)

### Clone this wiki locally

## Footer

© 2026 GitHub, Inc.

### Footer navigation

-   [Terms](https://docs.github.com/site-policy/github-terms/github-terms-of-service)
-   [Privacy](https://docs.github.com/site-policy/privacy-policies/github-privacy-statement)
-   [Security](https://github.com/security)
-   [Status](https://www.githubstatus.com/)
-   [Community](https://github.community/)
-   [Docs](https://docs.github.com/)
-   [Contact](https://support.github.com?tags=dotcom-footer)
-   Manage cookies
-   Do not share my personal information

You can’t perform that action at this time.

.user-mention\[href$="/Yourumark"\] { color: var(--color-user-mention-fg); background-color: var(--bgColor-attention-muted, var(--color-attention-subtle)); border-radius: 2px; margin-left: -2px; margin-right: -2px; } .user-mention\[href$="/Yourumark"\]:before, .user-mention\[href$="/Yourumark"\]:after { content: ''; display: inline-block; width: 2px; }