# Minimal Void Linux Setup

A minimalist Void Linux configuration focused on efficiency and functionality.

## Core System Components

### Display Server & Desktop Environment
- **Wayland** - Modern display server protocol
- **Wayfire** - Lightweight, 3D Wayland compositor and window manager
- **Waybar** - Highly customizable status bar
- **wl-clipboard** - Command-line clipboard utilities for Wayland
- Also allows copy pasting between various windows
- **mesa-dri (Direct Rendering Infrastructure)**:
  - This package provides the OpenGL drivers for 3D acceleration
  - It's essential for hardware-accelerated graphics on Linux systems
  - Enables direct rendering between applications and your graphics hardware
  - Important for gaming, 3D applications, and smooth desktop effects
  - Part of the Mesa 3D graphics library system
- **vulkan-loader**:
  - This is the runtime loader for the Vulkan graphics API
  - It's required to run applications that use Vulkan
  - Handles the loading of Vulkan drivers and layers
  - Essential for modern gaming on Linux
  - Provides a lower-level, higher-performance alternative to OpenGL

### System Essentials

#### IPC
- **dbus** - System message bus
- **rtkit** - Realtime process scheduler (For realtime IPC communication like audio)

#### Displa


#### Time Synchronization
- **chrony** - Modern NTP daemon
  - Keeps system time accurate
  - More efficient than ntpd
  - Required for proper system operation and security

#### Polkit Authentication
- **seatd** - Authorization manager
  - Handles privileged operations
  - Needed by window managers to manage privileged operations.
  - Acts like a centralized session system for managing privileged operations.

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
  - 
### Network Management
- **wpa_supplicant** - Complete network connection manager (Wi-Fi, Ethernet, Mobile Broadband)

## Applications


### System Utilities
- **Fuzzel** - Minimal application launcher
- **Alacritty** - GPU-accelerated terminal emulator
- **nnn** - Fast and lightweight terminal file manager
- **btop** - Resource monitor and process viewer
- **grim** - Screenshot utility for Wayland
- **p7zip** - File archiver with high compression ratio

### Daily Use Applications
- **Firefox** - Web browser
- **VLC** - Media player supporting wide range of formats
- **nvim** - Terminal-based text editor
- **git** - Version control system

## Printing System
Complete printing solution with network discovery:
- **CUPS** - Print server and manager
- **cups-pdf** - PDF virtual printer driver
- **ghostscript** - PostScript interpreter (required for CUPS)
- **avahi** - Network service discovery
- **nss-mdns** - Multicast DNS resolution
- **sane** - Scanner Access Now Easy (scanning utility)

---
*This setup aims for minimalism while maintaining full functionality for daily use.*