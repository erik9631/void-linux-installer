# Minimal Void Linux Setup

A minimalist Void Linux configuration focused on efficiency and functionality.

## Core System Components

### Display Server & Desktop Environment
- **Wayland** - Modern display server protocol
- **Wayfire** - Lightweight, 3D Wayland compositor
- **Waybar** - Highly customizable status bar
- **Mako** - Lightweight Wayland notification daemon
- **Fuzzel** - Minimal application launcher
- **wl-clipboard** - Command-line clipboard utilities for Wayland


### System Essentials

#### Time Synchronization
- **chrony** - Modern NTP daemon
  - Keeps system time accurate
  - More efficient than ntpd
  - Required for proper system operation and security

#### Polkit Authentication
- **polkit** - Authorization manager
  - Handles privileged operations
  - Required for mounting drives, power management
  - Needed by many desktop applications

#### XDG Portals (Wayland Integration)
- **xdg-desktop-portal-wlr** - Wayland portal backend
  - Required for screen sharing
  - Handles file picking dialogs
  - Enables desktop integration features

### Audio Stack
Modern audio system built on PipeWire with comprehensive compatibility layers:

- **pipewire** - Modern audio/video server
  - Handles audio/video routing with low latency
  - Provides better performance than traditional solutions
  - Core component required for all audio functionality

Compatibility Layers:
- **pipewire-pulse** - PulseAudio compatibility
  - Required for modern desktop applications
  - Enables per-application volume control
  - Needed for browsers and most modern Linux software

- **pipewire-alsa** - ALSA compatibility
  - Supports legacy applications
  - Handles basic system sounds
  - Provides compatibility with the kernel's base sound system

Optional Components:
- **pipewire-jack** - JACK audio compatibility
  - Only needed for professional audio work
  - Required for Digital Audio Workstations (DAWs)
  - Can be omitted in standard desktop setups

Note: For a basic desktop setup, only `pipewire`, `pipewire-pulse`, and `pipewire-alsa` are necessary.

### Power Management
TLP-based power management solution:

- **tlp** - Advanced power management
  - Optimizes battery life automatically
  - Manages CPU frequency scaling
  - Controls USB autosuspend
  - Handles disk power management
  - Manages Wi-Fi and Bluetooth power states
  - Configurable via `/etc/tlp.conf`

- **powertop** - Power consumption analyzer
  - Diagnostic tool for power usage
  - Install temporarily when needed for troubleshooting
  - Not required for regular operation

### Network Management
- **NetworkManager** - Complete network connection manager (Wi-Fi, Ethernet, Mobile Broadband)

## Applications

### System Utilities
- **Alacritty** - GPU-accelerated terminal emulator
- **nnn** - Fast and lightweight terminal file manager
- **btop** - Resource monitor and process viewer
- **grim** - Screenshot utility for Wayland
- **p7zip** - File archiver with high compression ratio

### Daily Use Applications
- **Firefox** - Web browser
- **VLC** - Media player supporting wide range of formats
- **vim** - Terminal-based text editor
- **git** - Version control system

## Printing System
Complete printing solution with network discovery:
- **CUPS** - Print server and manager
- **cups-pdf** - PDF virtual printer driver
- **ghostscript** - PostScript interpreter (required for CUPS)
- **avahi** - Network service discovery
- **nss-mdns** - Multicast DNS resolution
- **sane** - Scanner Access Now Easy (scanning utility)

## Services to Enable
```bash
# Core
ln -s /etc/sv/dbus /var/service/
ln -s /etc/sv/NetworkManager /var/service/

# Printing
ln -s /etc/sv/cupsd /var/service/
ln -s /etc/sv/avahi-daemon /var/service/

# Audio
ln -s /etc/sv/pipewire /var/service/
# Power Management
ln -s /etc/sv/tlp /var/service/
```

## Post-Installation Notes
- Configure NetworkManager with `nmtui` or `nmcli`
- Set up printing via CUPS web interface at `localhost:631`
- Configure Wayfire using `~/.config/wayfire.ini`
- Configure Waybar using `~/.config/waybar/config`

---
*This setup aims for minimalism while maintaining full functionality for daily use.*