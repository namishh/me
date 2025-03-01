---
title: The Nixos Guide
description: The nixos guide for the noobs made by a noob. This guide will aim to get you to have a basic flake powered nix setup with hyprland and wezterm. Rofi, mpd, ncmpcpp, and handling colors also included
date: 09 Oct 2023
author: Namish Pande
draft: false
category: guide
---

<img src="/nixwall.png" />

<a href="https://github.com/NixOS/nixos-artwork">wall from official nixos repo</a>

<br/>

<br/>

## Table Of Contents

1. [Intro](#intro)
    + [Basic Nix Commands](#basic-nix-commands)
2. [Flakes](#flakes)
    + [Why flakes?](#why-flakes)
    + [Enabling flakes](#enabling-flakes)
    + [Basic Flake](#basic-flake)
3. [Deriviations and overlays](#deriviations-and-overlays)
    + [Deriviations](#deriviations)
    + [Example Deriviations](#example-deriviations)
    + [Overlays](#overlays)
4. [Configuring the system](#configuring-the-system)
    + [Shared Settings](#shared-settings)
    + [System Specific configurations](#system-specific-configurations)
5. [Home Manager](#home-manager)
    + [Handling Colors](#handling-colors)
    + [Xresources](#xresources)
    + [Hyprland](#hyprland)
    + [Waybar](#waybar)
    + [Wezterm](#wezterm)
    + [Dunst](#dunst)
    + [ZSH](#zsh)
    + [Music](#music)
    + [Bonus - Creating Files](#bonus-creating-files)
6. [Dynamic GTK Theming](#dynamic-gtk-theming)
7. [Nix Shell](#nix-shell)
8. [Conclusion](#conclusion)
    + [Some sites and cool dotfiles](#some-sites-and-cool-dotfiles)


## Intro

Nixos is not your regular distro. Its "immutable" and "reproducible" and all that, things that most the of arch linux elites ignore before jumping into the nixos hype train. Instead of configuring your programs through `~/.config/appname` files, we do it through `.nix` files.  Instead of starting and stopping services with `systemctl`, we do it through `.nix` files. Instead of installing programs via a package manager, we do it through, you guessed it, `.nix` files. Most of the linux youtubers and average users do not fully explore nixos before going back to their old distro. They just configure their stuff in a big and ugly `configuration.nix` file. This blog will try to get you working `hyprland` on a `flake` powered nixos setup.

### Basic Nix Commands

+ Rebuilding Nix System 

```bash
$ sudo nixos-rebuild switch
# with flakes 
$ sudo nixos-rebuild switch --flake '/etc/nixos#frostbyte'
```

+ Rebuilding Home Directory 

```bash
$ home-manager switch

# with flakes 
$ home-manager switch --flake '/etc/nixos#namish' # replace namish with your user
```

+ Updating nix flake inputs 

```bash
$ nix flake update
```

## Flakes

A nix flake is a directory with a `flake.nix` and `flake.lock` that  returns Nix expressions that can be used to install packages, run programs or in out case, create a whole ass NixOs configuration. Each flake consists of two parts defined in the `flake.nix`, `inputs` and `outputs`

+ Inputs: `inputs` are the equivalent of dependencies. They are the external resources or Flakes that a particular Flake needs to accomplish its tasks

+ Outpus: `outputs` refer to the different results or artifacts that a Flake can produce when it's built or evaluated. Think of them as the things that a Flake can create or provide. In this case it provides us with a nixos configuration

### Why flakes?

NixOS Flakes is a feature that enhances the Nix package manager's capabilities by providing a structured and reproducible way to define and manage software packages and system configurations. It promotes consistency, immutability, and composability, making it particularly valuable for system administrators and developers who need to manage complex and reliable computing environments

### Enabling Flakes

Currently flakes is a beta experimental feature that we have to manually enable
```nix
{ pkgs, ... }: {
  nix.settings.experimental-features = [ "nix-command" "flakes" ];
}
```
Then rebuild system with `sudo nixos-rebuild switch`


### Basic Flake

```nix
{
  description = "i have no idea how this works";

  inputs = {
    # Package sources.
    master.url = "github:nixos/nixpkgs/master";
    stable.url = "github:nixos/nixpkgs/nixos-22.11";
    unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    hyprland.url = "github:hyprwm/Hyprland";
    spicetify-nix.url = "github:the-argus/spicetify-nix";
    nixpkgs-f2k.url = "github:fortuneteller2k/nixpkgs-f2k";
    hyprland-plugins.url = "github:hyprwm/hyprland-plugins";

    # Channel to follow.
    home-manager.inputs.nixpkgs.follows = "unstable";
    nixpkgs.follows = "unstable";
  };
  outputs = { self, nixpkgs, home-manager, hyprland, hyprland-plugins, ... } @inputs:
    let
      inherit (self) outputs;
      forSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      # host configurations
      nixosConfigurations = {
        frostbyte = nixpkgs.lib.nixosSystem
          {
            specialArgs = {
              inherit inputs outputs home-manager hyprland hyprland-plugins;
            };
            modules = [
              # > Our main nixos configuration file <
              home-manager.nixosModule
              ./hosts/frostbyte/configuration.nix
            ];
          };
      };
      home-manager = home-manager.packages.${nixpkgs.system}."home-manager";
      # user configurations
      homeConfigurations = {
        namish = home-manager.lib.homeManagerConfiguration {
          pkgs = nixpkgs.legacyPackages.x86_64-linux; # Home-manager requires 'pkgs' instance
          extraSpecialArgs = { inherit inputs outputs home-manager self; };
          modules = [
            ./home/namish/home.nix
          ];
        };
      };
      frostbyte = self.nixosConfigurations.frostbyte.config.system.build.toplevel;
    };
}

```

Now of course this will not work, we still have to configure home manager and our system. Be sure to replace `namish` with your username.

## Deriviations and overlays

This section is very important to understand. These two topics are really important to install programs that are not in the nix repos. In our config derivs will be placed at `./derivs` and overlays are at `./overlays/default.nix`

### Deriviations 

At a basic level, deriviations are just a way to build packages on your system. The structure of a Nix derivation is a set of specifications and instructions that define how to build and package a software component within the Nix package management system

Common Pieces of a nix deriviation:

+ Name and Version: A derivation typically starts with specifying the name and version of the software component you want to package. 

```
name = "example";
version = "1.0";
```

+ Source: You specify the source code or binary of the software you're packaging. This could be from a `tar`, `github repo`, a `url` or even local files

```
  src = fetchFromGitHub {
    owner = "ozwaldorf";
    repo = "lutgen-rs";
    rev = "621db41b10e5a1a923ef67094ce1fc05c618d6ae";
    sha256 = "0dwj3cksf62z89ihqnhhxj1wgzjqqwlc40hwdfw18yqwr3byzfxf";
  };
```

+ Build Process: This section outlines the steps to compile and build the software. You specify how to configure, compile, and install the software

```
buildPhase = ''
  make
'';

installPhase = ''
  make install
'';
```

+ Dependencies: You declare the dependencies required to build and run the software. Nix will ensure that these dependencies are available during the build process
```
buildInputs = with pkgs;[ gcc autoconf ];
```

+ Meta Info: You can include metadata about the package, such as its description and license information.

```
meta = with lib; {
  description = "An example software package";
  license = licenses.mit;
};
```
### Example Deriviations

```nix
{ lib, buildPythonPackage, fetchFromGitHub, pkgs, ... }\:

buildPythonPackage rec {
  pname = "imagecolorizer";
  version = "git";
  preBuild = ''
    cat > setup.py << EOF
    from setuptools import setup
    setup(
        name='ImageColorizer',
        version='1.2',
        packages=['ImageColorizer'],
        entry_points = {
            'console_scripts': ['ImageColorizer = ImageColorizer.__main__:main']
        }
    )
    EOF
  '';
  propagatedBuildInputs = with pkgs;[
    python310Packages.pillow
  ];
  src = fetchFromGitHub {
    repo = "ImageColorizer";
    owner = "kiddae";
    rev = "48623031e3106261093723cd536a4dae74309c5d";
    sha256 = "0ai4i3qmk55z3zc2gd8nicgx04pmfxl5wcq43ryy6l4c6gj2ik5r";
  };
  meta = {
    description = "ImageColorizer is a Python module and a CLI tool that you can easily use to colorize wallpapers for them to fit a terminal colorscheme.";
    homepage = "https://github.com/kiddae/ImageColorizer";
    license = lib.licenses.mit;
    platforms = lib.platforms.unix;
  };
}

```
<br/>
<br/>
```nix
{ lib, fetchFromGitHub, rustPlatform, pkgs }:
rustPlatform.buildRustPackage rec {
  pname = "lutgen";
  name = "lutgen";

  src = fetchFromGitHub {
    owner = "ozwaldorf";
    repo = "lutgen-rs";
    rev = "621db41b10e5a1a923ef67094ce1fc05c618d6ae";
    sha256 = "0dwj3cksf62z89ihqnhhxj1wgzjqqwlc40hwdfw18yqwr3byzfxf";
  };
  nativeBuildInputs = with pkgs;[
    cargo
    rustc
  ];
  cargoSha256 = "sha256-s5ejGEFMxDg+ENLg0Y1ZXgk2bDyy4H5C7tNMjVEp8kY=";
}
```

### Overlays

Nix overlays are a mechanism in the Nix package manager that allows you to extend or modify the package set provided by the Nixpkgs repository. They are a way to add, replace, or customize packages and configurations without altering the global Nixpkgs repository. For example to use a custom fork of st we can make an overlay like this

```nix
{ inputs }:
{
  additions = final: _prev: import ../pkgs { pkgs = final; inherit inputs; };
  modifications = final: prev: {
    # WE WILL ADD OUR OVERLAYS HERE
    st = prev.st.overrideAttrs (oldAttrs: {
      buildInputs = oldAttrs.buildInputs ++ [ prev.harfbuzz ];
      src = prev.fetchFromGitHub {
        owner = "chadcat7";
        repo = "st";
        rev = "3d9eb51d43981963638a1b5a8a6aa1ace4b90fbb";
        sha256 = "007pvimfpnmjz72is4y4g9a0vpq4sl1w6n9sdjq2xb2igys2jsyg";
      };
    });
  };
}
```

For this to work, you need `./nixpkgs.nix` and `./pkgs/default.nix`

```nix
# A nixpkgs instance that is grabbed from the pinned nixpkgs commit in the lock file
# This is useful to avoid using channels when using legacy nix commands
let lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
in
import (fetchTarball {
  url = "https://github.com/nixos-unstable/nixpkgs/archive/${lock.rev}.tar.gz";
  sha256 = lock.narHash;
})

```


```nix
# Custom packages, that can be defined similarly to ones from nixpkgs
# You can build them using 'nix build .#example' or (legacy) 'nix-build -A example'
{ pkgs ? (import ../nixpkgs.nix) { }, inputs }: {
  # example = pkgs.callPackage ./example { };
}
```

We can even use our deriviations to create an overlay

```nix
  modifications = final: prev: {
    imgclr = prev.callPackage ../derivs/imagecolorizer.nix {
      buildPythonPackage = prev.python310Packages.buildPythonPackage;
    };
    lutgen = prev.callPackage ../derivs/lutgen.nix { };
  };
```
### Example Deriviations

```nix
{ lib, buildPythonPackage, fetchFromGitHub, pkgs, ... }:

buildPythonPackage rec {
  pname = "imagecolorizer";
  version = "git";
  preBuild = ''
    cat > setup.py << EOF
    from setuptools import setup
    setup(
        name='ImageColorizer',
        version='1.2',
        packages=['ImageColorizer'],
        entry_points = {
            'console_scripts': ['ImageColorizer = ImageColorizer.__main__:main']
        }
    )
    EOF
  '';
  propagatedBuildInputs = with pkgs;[
    python310Packages.pillow
  ];
  src = fetchFromGitHub {
    repo = "ImageColorizer";
    owner = "kiddae";
    rev = "48623031e3106261093723cd536a4dae74309c5d";
    sha256 = "0ai4i3qmk55z3zc2gd8nicgx04pmfxl5wcq43ryy6l4c6gj2ik5r";
  };
  meta = {
    description = "ImageColorizer is a Python module and a CLI tool that you can easily use to colorize wallpapers for them to fit a terminal colorscheme.";
    homepage = "https://github.com/kiddae/ImageColorizer";
    license = lib.licenses.mit;
    platforms = lib.platforms.unix;
  };
}
```

```nix
{ lib, fetchFromGitHub, rustPlatform, pkgs }:
rustPlatform.buildRustPackage rec {
  pname = "lutgen";
  name = "lutgen";

  src = fetchFromGitHub {
    owner = "ozwaldorf";
    repo = "lutgen-rs";
    rev = "621db41b10e5a1a923ef67094ce1fc05c618d6ae";
    sha256 = "0dwj3cksf62z89ihqnhhxj1wgzjqqwlc40hwdfw18yqwr3byzfxf";
  };
  nativeBuildInputs = with pkgs;[
    cargo
    rustc
  ];
  cargoSha256 = "sha256-s5ejGEFMxDg+ENLg0Y1ZXgk2bDyy4H5C7tNMjVEp8kY=";
}
```
### Overlays

Nix overlays are a mechanism in the Nix package manager that allows you to extend or modify the package set provided by the Nixpkgs repository. They are a way to add, replace, or customize packages and configurations without altering the global Nixpkgs repository. For example to use a custom fork of st we can make an overlay like this

```nix
{ inputs }:
{
  additions = final: _prev: import ../pkgs { pkgs = final; inherit inputs; };
  modifications = final: prev: {
    # WE WILL ADD OUR OVERLAYS HERE
    st = prev.st.overrideAttrs (oldAttrs: {
      buildInputs = oldAttrs.buildInputs ++ [ prev.harfbuzz ];
      src = prev.fetchFromGitHub {
        owner = "chadcat7";
        repo = "st";
        rev = "3d9eb51d43981963638a1b5a8a6aa1ace4b90fbb";
        sha256 = "007pvimfpnmjz72is4y4g9a0vpq4sl1w6n9sdjq2xb2igys2jsyg";
      };
    });
  };
}
```

For this to work, you need `./nixpkgs.nix` and `./pkgs/default.nix`

```nix
# A nixpkgs instance that is grabbed from the pinned nixpkgs commit in the lock file
# This is useful to avoid using channels when using legacy nix commands
let lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
in
import (fetchTarball {
  url = "https://github.com/nixos-unstable/nixpkgs/archive/${lock.rev}.tar.gz";
  sha256 = lock.narHash;
})

```


```nix
# Custom packages, that can be defined similarly to ones from nixpkgs
# You can build them using 'nix build .#example' or (legacy) 'nix-build -A example'
{ pkgs ? (import ../nixpkgs.nix) { }, inputs }: {
  # example = pkgs.callPackage ./example { };
}
```

We can even use our deriviations to create an overlay

```nix
modifications = final: prev: {
  ...
  imgclr = prev.callPackage ../derivs/imagecolorizer.nix {
    buildPythonPackage = prev.python310Packages.buildPythonPackage;
  };
  lutgen = prev.callPackage ../derivs/lutgen.nix { };
};
```

## Configuring the system

First we will make two directories, `./hosts/frostbyte` and `./hosts/shared`. The first one would be for the main system itself and shared would be for setttings that would be common for each system (if you have more than 1 devices)

Also copy your existing `hardware-configuration.nix` to  `./hosts/frostbyte/hardware-configuration.nix`

### Shared Settings

This will consist of:
+ Setting the timezone
+ Enabling network, bluetooth, sudo, polkit
+ User Config 
+ Installing Fonts
+ Nix Settings
+ Actually Installing the overlays

```nix
{ pkgs, outputs, overlays, lib, inputs, ... }:
let
  # DEFINING VARIABLES
  flake-compat = builtins.fetchTarball "https://github.com/edolstra/flake-compat/archive/master.tar.gz";
in
{
  # USING SYSTEMD-BOOT INSTEAD OF GRUB
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;
  boot.loader.efi.efiSysMountPoint = "/boot/efi";
  programs.zsh.enable = true;

  # REQUIRED TO ENABLE SWAYLOCK TO AUTHENTICATE PASSWORDS
  security.pam.services.swaylock = {
    text = ''
      auth include login
    '';
  };
  networking = {
    networkmanager.enable = true;
    firewall.enable = false;
  };
  security = {
    sudo.enable = true;
  };
  services.blueman = {
    enable = true;
  };

  # SETTING TIME ZONE
  time = {
    hardwareClockInLocalTime = true;
    timeZone = "Asia/Kolkata";
  };
  i18n.defaultLocale = "en_US.UTF-8";
  console = {
    font = "Lat2-Terminus16";
    useXkbConfig = true;
  };
  users = {
    users.namish = {
      isNormalUser = true;
      extraGroups = [ "wheel" "networkmanager" "audio" "video" "libvirtd" ];
      packages = with pkgs; [ ];
    };
    defaultUserShell = pkgs.zsh;
  };
  fonts.packages = with pkgs; [
    inter
    dosis
    rubik
    ibm-plex
    (nerdfonts.override { fonts = [ "Iosevka" "CascadiaCode" "JetBrainsMono" ]; })
  ];
  # STILL USING PULSEAUDIO (SOWWY)
  sound.enable = true;
  hardware.pulseaudio.enable = true;
  hardware.pulseaudio.extraConfig = "load-module module-native-protocol-tcp auth-ip-acl=127.0.0.1";
  security.rtkit.enable = true;
  virtualisation = {
    libvirtd.enable = true;
  };
  services.dbus.enable = true;
  xdg.portal = {
    enable = true;
    wlr.enable = true;
    # gtk portal needed to make gtk apps happy
    extraPortals = [ pkgs.xdg-desktop-portal-gtk ];
  };

  environment.systemPackages = with pkgs; [
    nodejs
    lutgen
    home-manager
    blueman
    inotify-tools
    udiskie
    rnix-lsp
    xorg.xwininfo
    pulseaudio
    libnotify
    xdg-utils
    gtk3
    jq
    st
    spotdl
    discord
    firefox
    unzip
    imgclr
    grim
    eww-wayland
    wayland
    swaylock-effects
    swaybg
    git
    pstree
    mpv
    xdotool
    spotify
    brightnessctl
    pamixer
    nix-prefetch-git
    python3
    brillo
    slop
    ripgrep
    maim
    wirelesstools
    xorg.xf86inputevdev
    xorg.xf86inputsynaptics
    xorg.xf86inputlibinput
    xorg.xorgserver
    xorg.xf86videoati
  ];
  # SETTING THE DEFAULT SHELL
  environment.shells = with pkgs; [ zsh ];

  programs.dconf.enable = true;
  qt = {
    enable = true;
    platformTheme = "gtk2";
    style = "gtk2";
  };

  # PRINTING  // BLUETOOTH
  services.printing.enable = true;
  hardware.bluetooth = {
    enable = true;
    powerOnBoot = false;
  };
  services.xserver = {
    layout = "us";
    xkbVariant = "us,";
  };
  security.polkit.enable = true;
  nix = {
    settings = {
      experimental-features = [ "nix-command" "flakes" ];
      trusted-users = [ "root" "@wheel" ];
      auto-optimise-store = true;
      warn-dirty = false;
    };
    gc = { ## AUTOMATICALLY DELETEING OLD BUILDS AFTER 5 DAYS
      automatic = true;
      options = "--delete-older-than 5d";
    };
    optimise.automatic = true;
  };
  system = {
    copySystemConfiguration = false;
    stateVersion = "22.11";
  };
}
```

### System Specific configurations 

Now this is used for:

+ Overlays
+ Networking
+ Services

**BONUS** -- also learn how to install awesomewm

```nix
{ inputs, outputs, config, pkgs, lib, self, ... }:
{

  imports = [
    ./hardware-configuration.nix
    ../shared
  ];

  nixpkgs = {
    overlays = [
      outputs.overlays.modifications
      outputs.overlays.additions
      inputs.nixpkgs-f2k.overlays.stdenvs
      inputs.nixpkgs-f2k.overlays.compositors
      (final: prev:
        {
          awesome = inputs.nixpkgs-f2k.packages.${pkgs.system}.awesome-git;
        })
    ];
    config = {
      # Disable if you don't want unfree packages
      allowUnfreePredicate = _: true;
      allowUnfree = true;
    };
  };

  networking.hostName = "frostbyte";
  networking.useDHCP = false;
  networking.interfaces.wlo1.useDHCP = true;
  boot.kernelPackages = pkgs.linuxPackages_5_15;
  environment.systemPackages = lib.attrValues {
    inherit (pkgs)
      brightnessctl
      wayland
      android-tools;
  };

  services = {
    gvfs.enable = true;
    power-profiles-daemon.enable = false;
    tlp.enable = true;
    upower.enable = true;
    xserver = {
      enable = true;
      videoDrivers = [ "amdgpu" ];
      libinput = {
        enable = true;
        touchpad = {
          tapping = true;
          middleEmulation = true;
          naturalScrolling = true;
        };
      };
      displayManager = {
        defaultSession = "none+awesome";
        startx.enable = true;
      };
      windowManager.awesome = {
        enable = true;

      };
      desktopManager.gnome.enable = false;
    };
  };
}
```

If you are wondering, we will be installing `hyprland` via `home-manager`

## Home Manager

[Home Manager](https://github.com/nix-community/home-manager) gives you control over the user's home environment. Home-Manager fosters user-specific profiles, accommodating distinct configurations for different users on the same system. Each user can have their own tailored environment, making it versatile and adaptable for both personal and multi-user systems

For this too, we will make two directories  `./home/USER` (replace USER with your username) and `./home/shared`. The first one would be for user itself and shared would be for setttings that would be common for each user


### Handling Colors

```nix
{}:
rec {
  wallpaper = "birdseye.jpg";
  foreground = "d7e0e0";
  cursorColor = "d7e0e0";
  background = "0a1011";
  darker = "080c0d";

  color0 = "0d1617";
  color8 = "253336";

  color1 = "df5b61";
  color9 = "f16269";

  color2 = "6ec587";
  color10 = "8dd5a0";

  color3 = "de8c6a";
  color11 = "e59575";

  color4 = "659bdb";
  color12 = "739bdf";

  color5 = "c167d9";
  color13 = "d16ee0";

  color6 = "6fd1d5";
  color14 = "7bd3e0";

  color7 = "c5d7d7";
  color15 = "cedcd9";

  bg2 = "101a1b";
  mbg = "0e1718";

  contrast = "111a1b";
  cursorline = "111a1b";

  comment = "505758";
  name = "wave";
  neofetchpic = "verycool.png";
}
```

<img src="/wavescrot.png" />

This is how the theme looks on awesome

<br/> 

`rec` is the programming equivalent of `return`. Now we can use this file to get the colors, whereever we want

```nix
{ inputs, config, pkgs, lib, ... }:
let
  spicetify-nix = inputs.spicetify-nix;
  colors = import ../shared/cols/wave.nix { };
  hyprland = inputs.hyprland;
  hyprland-plugins = inputs.hyprland-plugins;
  unstable = import
    (builtins.fetchTarball "https://github.com/nixos/nixpkgs/archive/master.tar.gz")
    {
      config = config.nixpkgs.config;
    };
  nixpkgs-f2k = inputs.nixpkgs-f2k;
in
{
  # some general info
  home.username = "namish";
  home.homeDirectory = "/home/namish";
  home.stateVersion = "22.11";
  programs.home-manager.enable = true;
  home.file.".icons/default".source =
    "${pkgs.phinger-cursors}/share/icons/phinger-cursors";


  nixpkgs.overlays = [
  ];
  imports = [
  ];
  ## THIS CODE IS RUN WHENEVER HOME MANAGER REBUILDS THE HOME DIRECTORY
  home = {
    activation = {
      installConfig = ''
        if [ ! -d "${config.home.homeDirectory}/.config/nvim" ]; then
          ${pkgs.git}/bin/git clone --depth 1 https://github.com/chadcat7/kodo ${config.home.homeDirectory}/.config/nvim
        fi
      ''; ## if there is no ~/.config/nvim, git clone my nvim config
    };
    packages = with pkgs; [
      bc
      chromium
      dunst
      wl-clipboard
      sway-contrib.grimshot
      xss-lock
      htop
      recode
      gcc
      go
      gopls
      playerctl
      scc
      cinnamon.nemo
      neofetch
      rust-analyzer
      hsetroot
      notion-app-enhanced
      mpc-cli
      pfetch
      ffmpeg_5-full
      neovim
      xdg-desktop-portal
      imagemagick
      procps
      killall
      moreutils
      cava
      mpdris2
      socat
      pavucontrol
      fzf
      feh
      exa
    ];
  };

  ## IMPORTANT
  nixpkgs.config = {
    allowUnfree = true;
    allowBroken = true;
    allowUnfreePredicate = _: true;
  };
}
```

### Xresources

```nix
{ colors }:

with colors; {
  xresources = {
    path = ".Xresources";
    extraConfig = ''
    '';
    properties = {
      "*.background" = "#${background}";
      "*.darker" = "#${darker}";
      "*.color0" = "#${color0}";
      "*.color8" = "#${color8}";
      "*.color7" = "#${color7}";
      "*.color15" = "#${color15}";
      "*.foreground" = "#${foreground}";
      "*.color1" = "#${color1}";
      "*.color9" = "#${color9}";
      "*.color2" = "#${color2}";
      "*.color10" = "#${color10}";
      "*.color3" = "#${color3}";
      "*.color11" = "#${color11}";
      "*.color4" = "#${color4}";
      "*.color12" = "#${color12}";
      "*.color5" = "#${color5}";
      "*.color13" = "#${color13}";
      "*.color6" = "#${color6}";
      "*.color14" = "#${color14}";
      "*.contrast" = "#${contrast}";
      "*.cursorline" = "#${cursorline}";
      "*.comment" = "#${comment}";
      "st.borderpx" = 32;
    };
  };
}
```
I think this piece of code pretty self explanantory

```nix
imports = [
    (import ../shared/xresources.nix { inherit colors; })
];
```

And then also import it

### Hyprland

The only thing we have to do for installing hyprland is enable it. (now that i read this sentence again, it feels really dumb). Anyways create file `./home/USER/conf/ui/hyprland/default.nix`

```nix
{ config, lib, pkgs, hyprland, colors, ... }:

{
  systemd.user.targets.hyprland-session.Unit.Wants = [ "xdg-desktop-autostart.target" ];
  wayland.windowManager.hyprland = with colors; {
    enable = true;
    package = hyprland.packages.${pkgs.system}.hyprland;
    systemdIntegration = true;
    extraConfig = ''
      .....
      exec = swaybg -i ~/.wallpapers/${name}/${wallpaper} &
      .....
    '';
  };
}
```

This was done considering you have wallpapers in `.wallpapers/wave/wallpaper.png` or something like that. And also do not forget to add in your extra config duh. 😑 

And then import this file in your `home.nix`

```nix
imports = [
   ...
   (import ./conf/ui/hyprland/default.nix { inherit config pkgs lib hyprland colors; })
];
```

### Waybar

Instead of using the waybar in official nix packages repositories, we will use the one available from `hyprland` flake input we used.

```nix
{ config, lib, pkgs, hyprland, colors, ... }:

{
  programs.waybar =
    with colors; {
      enable = true;
      package = hyprland.packages.${pkgs.system}.waybar-hyprland;
      systemd = {
        enable = false;
        target = "graphical-session.target";
      };
      style = ''
        window#waybar {
          background-color: #${background};
          color: #${foreground};
          border-bottom: none;
        }
        * {
          font-size: 16px;
          min-height: 0;
          font-family: "Iosevka Nerd Font", "Material Design Icons Desktop";
        }
        ....
      '';
      settings = [{
        height = 35;
        layer = "top";
        position = "top";
        tray = { spacing = 10; };
        modules-center = [ "clock" ];
        modules-left = [ "hyprland/workspaces" ];
        modules-right = [
          "network"
          "tray"
        ];
        "hyprland/workspaces" = {
          on-click = "activate";
          all-outputs = true;
          format = "{icon}";
          disable-scroll = true;
          active-only = false;
          format-icons = {
            default = "󰊠 ";
            persistent = "󰊠 ";
            focused = "󰮯 ";
          };
          persistent_workspaces = {
            "1" = [ ];
            "2" = [ ];
            "3" = [ ];
            "4" = [ ];
            "5" = [ ];
          };
        };
        clock = {
          format = "{:%d %A %H:%M}";
          tooltip-format = "{:%Y-%m-%d | %H:%M}";
        };
        network = {
          interval = 1;
          on-click = "eww open --toggle control";
          format-disconnected = "󰤮 ";
          format-wifi = "󰤨 ";
        };
        "custom/power" = {
          on-click = "powermenu &";
          format = " ";
        };
      }];
    };
}
```

and ofc import this too 

```
imports = [
    ...
    (import ./conf/utils/dunst/default.nix { inherit colors pkgs; })
];
```

### Wezterm

Wezterm may not be the most minimal terminal, but is still better than [that bloated piece of shit](https://github.com/kovidgoyal/kitty). Yes I have changed my terminal yet again. (future blog probably soon). Wezterms font rendering is the most alike to ST and it can also change colorschemes instantly, also has ligature support, and much more. (and i am not even using the git version). Create a directory `./home/USER/conf/term/wezterm` and create two files `default.nix` and `colors.nix`

```nix
{ colors, ... }:
with colors; {
  followSystem = {
    # basic colors
    background = "#${background}";
    cursor_bg = "#${foreground}";
    cursor_border = "#${foreground}";
    cursor_fg = "#${color8}";
    foreground = "#${foreground}";
    selection_bg = "#${color15}";
    selection_fg = "#${background}";
    split = "#${mbg}";

    # base16
    ansi = [
      "#${color0}"
      "#${color1}"
      "#${color2}"
      "#${color3}"
      "#${color4}"
      "#${color5}"
      "#${color6}"
      "#${color7}"
    ];
    brights = [
      "#${color8}"
      "#${color9}"
      "#${color10}"
      "#${color11}"
      "#${color12}"
      "#${color13}"
      "#${color14}"
      "#${color15}"
    ];

    # tabbar
    tab_bar = {
      background = "#${color8}";
      active_tab = {
        bg_color = "#${background}";
        fg_color = "#${foreground}";
      };
      inactive_tab = {
        bg_color = "#${color8}";
        fg_color = "#${foreground}";
      };
      inactive_tab_hover = {
        bg_color = "#${color0}";
        fg_color = "#${foreground}";
      };
      inactive_tab_edge = "#${color0}";
      new_tab = {
        bg_color = "#${color8}";
        fg_color = "#${color7}";
      };
      new_tab_hover = {
        bg_color = "#${color0}";
        fg_color = "#${foreground}";
      };
    };
  };
}
```

```nix
{ pkgs, colors, ... }:

with colors; {
  programs.wezterm = {
    enable = true;
    colorSchemes = import ./colors.nix {
      inherit colors;
    };
    extraConfig = ''
      local wez = require('wezterm')
      return {
        default_prog     = { 'zsh' },
        cell_width = 0.85,
        front_end        = "OpenGL",
        enable_wayland   = true,
        scrollback_lines = 1024,
        font         = wez.font_with_fallback({ 
          "Iosevka Nerd Font",
          "Material Design Icons",
        }),
        dpi = 96.0,
        bold_brightens_ansi_colors = true,
        font_rules    = {
          {
            italic = true,
            font   = wez.font("Iosevka Nerd Font", { italic = true })
          }
        },
        font_size         = 14.0,
        line_height       = 1.15,
        harfbuzz_features = { 'calt=1', 'clig=1', 'liga=1' },
        color_scheme   = "followSystem",
        window_padding = {
          left = "24pt", right = "24pt",
          bottom = "24pt", top = "24pt"
        },
        default_cursor_style = "SteadyUnderline",
        enable_scroll_bar    = false,
        warn_about_missing_glyphs = false,
        enable_tab_bar               = true,
        use_fancy_tab_bar            = false,
        hide_tab_bar_if_only_one_tab = true,
        show_tab_index_in_tab_bar    = false,
        window_close_confirmation = "NeverPrompt",
        inactive_pane_hsb         = { 
          saturation = 1.0, brightness = 0.8
        },
        check_for_updates = false,
      }
    '';
  };
}
```

```
imports = [
    ...
    (import ./conf/term/wezterm/default.nix { inherit pkgs colors; })
];
```

### Dunst 

For notifications, there are now many options for wayland but I am still going to be using [dunst](https://github.com/dunst-project/dunst) because I had the least trouble configuring it. Nix considers `dunst` to be service instead of program, therefore, `programs.dunst` becomes `services.dunst`. The configuration file for dunst will be at `./home/USER/conf/utils/dunst/default.nix`

```nix
{ colors, pkgs }: with colors;{
  services.dunst = {
    enable = true;
    settings = {
      global = {
        follow = "mouse";
        width = 500;
        origin = "top-center";
        alignment = "left";
        vertical_alignment = "center";
        ellipsize = "middle";
        offset = "15x15";
        padding = 15;
        horizontal_padding = 15;
        text_icon_padding = 15;
        icon_position = "left";
        min_icon_size = 48;
        max_icon_size = 64;
        progress_bar = true;
        progress_bar_height = 8;
        progress_bar_frame_width = 1;
        progress_bar_min_width = 150;
        progress_bar_max_width = 300;
        separator_height = 2;
        frame_width = 2;
        frame_color = "#${mbg}";
        separator_color = "frame";
        corner_radius = 8;
        transparency = 0;
        gap_size = 8;
        line_height = 0;
        notification_limit = 0;
        idle_threshold = 120;
        history_length = 20;
        show_age_threshold = 60;
        markup = "full";
        font = "Iosevka Nerd Font 12";
        word_wrap = "yes";
        sort = "yes";
        shrink = "no";
        indicate_hidden = "yes";
        sticky_history = "yes";
        ignore_newline = "no";
        show_indicators = "no";
        stack_duplicates = true;
        always_run_script = true;
        hide_duplicate_count = false;
        ignore_dbusclose = false;
        force_xwayland = false;
        force_xinerama = false;
        mouse_left_click = "do_action";
        mouse_middle_click = "close_all";
        mouse_right_click = "close_current";
      };

      fullscreen_delay_everything = { fullscreen = "delay"; };
      urgency_low = {
        timeout = 3;
        background = "#${background}";
        foreground = "#${foreground}";
        highlight = "#${color4}";
      };
      urgency_normal = {
        timeout = 6;
        background = "#${background}";
        foreground = "#${foreground}";
        highlight = "#${color4}";
      };
      urgency_critical = {
        timeout = 0;
        background = "#${background}";
        foreground = "#${foreground}";
        highlight = "#${color9}";
      };
    };
  };

}
```

and the tradition continues

```
    imports = [
        ...
        (import ./conf/ui/waybar/default.nix { inherit config pkgs lib hyprland colors; })
    ];

```
### ZSH

zsh is my preferred shell (sorry fish noobs) so i will cover how to configure use zsh, create a file `./home/USER/conf/shell/zsh/default.nix`

```

{ config, colors, pkgs, lib, ... }:

{
  programs.zsh = {
    enable = true;
    enableAutosuggestions = true;
    syntaxHighlighting.enable = true;
    enableCompletion = true;
    history = {
      expireDuplicatesFirst = true;
      save = 512;
    };
    initExtra = ''
      bindkey  "^[[H"   beginning-of-line
      bindkey  "^[[4\~"   end-of-line
      bindkey  "^[[3\~"  delete-char
      export PATH=${config.home.homeDirectory}/.local/bin:${config.home.homeDirectory}/.local/share/nvim/mason/bin:$PATH
    '';
  };

}
```

** ADDING PLUGINS\- **

<br/>

Instead of using something like zplug, zgen, zpm, we will just install them, through, you guessed it, nix. We will use this to install powerlevel10k

```
programs.zsh = {
    plugins = [
      {
        name = "powerlevel10k";
        src = pkgs.zsh-powerlevel10k;
        file = "share/zsh-powerlevel10k/powerlevel10k.zsh-theme";
      }
      {
        name = "powerlevel10k-config";
        src = lib.cleanSource ./conf;
        file = "powerlevel.zsh";
      }
    ];
};
```

You will also need `./conf/shell/zsh/conf/powerlevel.zsh` which tou can get from [here](https://github.com/chadcat7/crystal/blob/main/home/namish/conf/shell/zsh/conf/powerlevel.zsh)

**SHELL ALIASES-**

Instead of adding aliases in `initExtra` we will do this 

```
programs.zsh = {
    shellAliases = {
      la = "exa -l";
      ls = "ls --color=auto";
      v = "nvim";
      nf = "neofetch";
      suda = "sudo -E -s";
      nix-pkgs = "nix --extra-experimental-features 'nix-command flakes' search nixpkgs";
    };

}
```

And in the end:

```nix
    imports = [
        ...
        (import ./conf/shell/zsh/default.nix { inherit config colors pkgs lib; })
    ];
```

<img src="/wezzsh.png"/>

### Music

This section will teach you how to set up mpd, ncmpcpp and playerctl on your system.  It plays audio files, organizes playlists and maintains a music database, all while using very few resources. In order to interface with it, a separate client is needed. The client here is `ncmpcpp`

Here is the mpd configuration in nix

```nix
{ config, pkgs }:

{
  services.mpd = {
    enable = true;
    musicDirectory = "${config.home.homeDirectory}/Music";
    dataDir = "${config.home.homeDirectory}/.config/mpd";
    extraConfig = ''
      auto_update           "yes"
      restore_paused        "yes"
      audio_output {
        type "pulse"
        name "Pulseaudio"
        server "127.0.0.1" # add this line - MPD must connect to the local sound server
      }

      audio_output {
      	type                "fifo"
      	name                "Visualizer"
      	format              "44100:16:2"
      	path                "/tmp/mpd.fifo"
      }
      audio_output {
      	type		            "httpd"
      	name		            "lossless"
      	encoder		          "flac"
      	port		            "8000"
      	max_client	        "8"
      	mixer_type	        "software"
      	format		          "44100:16:2"
      }
    '';
    network.startWhenNeeded = true;

    # Allows mpd to work with playerctl.
    home.packages = [ pkgs.playerctl ];
    services.mpdris2.enable = true;
    services.playerctld.enable = true;
  };
}
```

According to the **arch wiki**: Ncmpcpp is an mpd client (compatible with mopidy) with a UI very similar to ncmpc, but it provides new useful features such as support for regular expressions for library searches, extended song format, items filtering, the ability to sort playlists, and a local filesystem browser.

For ncmpcpp 

```nix
{config, pkgs }:
{
  programs.ncmpcpp = {
    enable = true;
    package = pkgs.ncmpcpp.override {
      visualizerSupport = true;
      clockSupport = true;
      taglibSupport = true;
    };
    mpdMusicDir = "${config.home.homeDirectory}/Music";
    settings = {
      # Miscelaneous
      ncmpcpp_directory = "${config.home.homeDirectory}/.config/ncmpcpp";
      ignore_leading_the = true;
      external_editor = "nvim";
      message_delay_time = 1;
      playlist_disable_highlight_delay = 2;
      autocenter_mode = "yes";
      centered_cursor = "yes";
      allow_for_physical_item_deletion = "no";
      lines_scrolled = "0";
      follow_now_playing_lyrics = "yes";
      lyrics_fetchers = "musixmatch";

      # visualizer
      visualizer_data_source = "/tmp/mpd.fifo";
      visualizer_output_name = "mpd_visualizer";
      visualizer_type = "ellipse";
      visualizer_look = "●●";
      visualizer_color = "blue, green";

      # appearance
      colors_enabled = "yes";
      playlist_display_mode = "classic";
      user_interface = "classic";
      volume_color = "white";

      # window
      song_window_title_format = "Music";
      statusbar_visibility = "no";
      header_visibility = "no";
      titles_visibility = "no";
      # progress bar
      progressbar_look = "━━━";
      progressbar_color = "black";
      progressbar_elapsed_color = "blue";

      # song list
      song_status_format = "$7%t";
      song_list_format = "$(008)%t$R  $(247)%a$R$5  %l$8";
      song_columns_list_format = "(53)[blue]{tr} (45)[blue]{a}";

      current_item_prefix = "$b$2| ";
      current_item_suffix = "$/b$5";

      now_playing_prefix = "$b$5| ";
      now_playing_suffix = "$/b$5";

      # colors
      main_window_color = "blue";

      current_item_inactive_column_prefix = "$b$5";
      current_item_inactive_column_suffix = "$/b$5";

      color1 = "white";
      color2 = "blue";
    };
  };
}
```

### Bonus - Creating Files

to write on a file on rebuilding home manager you can use `home.file`

for eg writing the `.xinitrc` file 

```nix
{}:
{
  home.file.".xinitrc".text = ''
    #!/usr/bin/env bash
    exec dbus-run-session awesome
  '';
}
```

you can also make bin files with this, just set `executable` to `true`

```nix
{ colors }:
{
  home.file.".local/bin/lock" = {
    executable = true;
    text = ''
      #!/bin/sh
      playerctl pause
      sleep 0.2
      swaylock -i ~/.config/awesome/theme/wallpapers/${colors.name}/${colors.wallpaper} --effect-blur 10x10
    '';
  };
}
```

## Dynamic GTK Theming

For dynamic themeing, we will just use [phocus](https://github.com/phocus/gtk) and then just subsititute the colors with patches. You are recommended to get your patches from [here](https://github.com/chadcat7/crystal/tree/main/patches). So this is the derivation we need for it - 

```nix
{ stdenvNoCC
, fetchFromGitHub
, nodePackages
, colors
,
}:
stdenvNoCC.mkDerivation rec {
  pname = "phocus";
  version = "0cf0eb35a927bffcb797db8a074ce240823d92de";

  src = fetchFromGitHub {
    owner = "phocus";
    repo = "gtk";
    rev = version;
    sha256 = "sha256-URuoDJVRQ05S+u7mkz1EN5HWquhTC4OqY8MqAbl0crk=";
  };

  patches = [
    ../patches/npm.diff
    ../patches/gradients.diff
    ../patches/substitute.diff
  ];

  postPatch = ''
    substituteInPlace scss/gtk-3.0/_colors.scss \
      --replace "@bg0@" "#${colors.background}" \
      --replace "@bg1@" "#${colors.contrast}" \
      --replace "@bg2@" "#${colors.color8}"\
      --replace "@bg3@" "#${colors.color0}" \
      --replace "@bg4@" "#${colors.comment}" \
      --replace "@red@" "#${colors.color1}" \
      --replace "@lred@" "#${colors.color9}" \
      --replace "@orange@" "#${colors.color3}" \
      --replace "@lorange@" "#${colors.color11}" \
      --replace "@yellow@" "#${colors.color3}" \
      --replace "@lyellow@" "#${colors.color11}" \
      --replace "@green@" "#${colors.color2}" \
      --replace "@lgreen@" "#${colors.color10}" \
      --replace "@cyan@" "#${colors.color6}" \
      --replace "@lcyan@" "#${colors.color15}" \
      --replace "@blue@" "#${colors.color4}" \
      --replace "@lblue@" "#${colors.color12}" \
      --replace "@purple@" "#${colors.color5}" \
      --replace "@lpurple@" "#${colors.color14}" \
      --replace "@pink@" "#${colors.color5}" \
      --replace "@lpink@" "#${colors.color14}" \
      --replace "@primary@" "#${colors.foreground}" \
      --replace "@secondary@" "#${colors.color15}"
  '';

  nativeBuildInputs = [ nodePackages.sass ];
  installFlags = [ "DESTDIR=$(out)" "PREFIX=" ];
}
```

And then we can call it in the home.nix file

```nix
  gtk = {
    enable = true;
    gtk3.extraConfig.gtk-decoration-layout = "menu:";
    theme.name = "phocus";
  };
```
## Nix Shell

Nix-shell is a command-line tool and concept within the Nix package manager that enables the creation of isolated development environments for software projects. It allows developers to specify the exact dependencies and environment required for a project, ensuring consistency and reproducibility across different systems and projects.

Here is an example `shell.nix`

```nix
{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs.buildPackages; [ lua52Packages.lua cmake gcc pam gnumake ];
}
```

To enter the shell we will use 

```bash
$ nix-shell shell.nix
```

## Conclusion

whew, if you made it there, i hope you learned something from this. if there is an error you can contact me at discord @ `chadcat7`. For a reference you can check out [my dots](https://github.com/chadcat7/crystal)


### Some sites and cool dotfiles

+ [https://zero-to-nix.com](https://zero-to-nix.com)
+ [https://github.com/fortuneteller2k/nix-config](https://github.com/fortuneteller2k/nix-config)
+ [https://github.com/shaunsingh/nix-darwin-dotfiles](https://github.com/shaunsingh/nix-darwin-dotfiles)
+ [https://github.com/javacafe01/dotfiles](https://github.com/javacafe01/dotfiles)
