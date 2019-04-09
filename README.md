<div align="center">

<a href="./assets/logo">
    <img src="./assets/logo/logo.png" width="20%" />
</a>
<br><br>

Another terminal based graphical activity monitor, inspired by [gtop](https://github.com/aksakalli/gtop) and [vtop](https://github.com/MrRio/vtop), originally written in Go, now in Rust!

<img src="./assets/demos/demo.gif" />
<img src="./assets/screenshots/minimal.png" width="96%" />

</div>

## Installation

Working and tested on Linux, FreeBSD and macOS. Windows support is planned. OpenBSD works with some caveats.

### Source

Latest release:

```bash
cargo install gotop
```

Latest commit:

```bash
cargo install --git https://github.com/cjbassi/gotop
```

### Prebuilt binaries

**Note**: Doesn't require Cargo/Rust.

Run the following to download the correct binary for your system from the releases tab into `$CARGO_HOME/bin`, courtesy of [japaric/trust](https://github.com/japaric/trust):

```bash
bash <(curl -LSfs https://japaric.github.io/trust/install.sh) \
  -f --git cjbassi/trash-man
```

You can change the download destination by passing the `--to <location>` flag (useful if Cargo/Rust isn't installed).

### Arch Linux

Install `gotop`, `gotop-bin`, or `gotop-git` from the AUR.

### FreeBSD

```bash
pkg install gotop
```

### Homebrew

```bash
brew tap cjbassi/gotop
brew install gotop
```

### Snap

```bash
snap install gotop-cjbassi
```

**Note**: You may need to enable certain permissions for all of the widgets to work:

```bash
snap connect gotop-cjbassi:hardware-observe
snap connect gotop-cjbassi:mount-observe
snap connect gotop-cjbassi:system-observe
```

## Usage

### Keybinds

- Quit: `q` or `<C-c>`
- Process navigation
  - `k` and `<Up>`: up
  - `j` and `<Down`: down
  - `<C-u>`: half page up
  - `<C-d>`: half page down
  - `<C-b>`: full page up
  - `<C-f>`: full page down
  - `gg` and `<Home>`: jump to top
  - `G` and `<End>`: jump to bottom
- Process actions:
  - `<Tab>`: toggle process grouping
  - `dd`: kill selected process or group of processes
- Process sorting
  - `c`: CPU
  - `m`: Mem
  - `p`: PID
- CPU and Mem graph scaling:
  - `h`: scale in
  - `l`: scale out
- `?`: toggles keybind help menu

### Mouse

- click to select process
- mouse wheel to scroll through processes

### Colorschemes

gotop ships with a few colorschemes which can be set with the `-c` flag followed by the name of one. You can find all the colorschemes in the [colorschemes folder](./colorschemes).

To make a custom colorscheme, copy one of the default ones to `~/.config/gotop/<name>.json` and load it with `gotop -c <name>`. Colorschemes PR's are welcome!

### CLI Options

`-c`, `--color=NAME` Set a colorscheme.  
`-m`, `--minimal` Only show CPU, Mem and Process widgets.  
`-r`, `--rate=RATE` Number of times per second to update CPU and Mem widgets [default: 1].  
`-V`, `--version` Print version and exit.  
`-p`, `--percpu` Show each CPU in the CPU widget.  
`-a`, `--averagecpu` Show average CPU in the CPU widget.  
`-s`, `--statusbar` Show a statusbar with the time.  
`-b`, `--battery` Show battery level widget (`minimal` turns off). [preview](./assets/screenshots/battery.png)
`-i`, `--interface=NAME` Select network interface [default: all].
