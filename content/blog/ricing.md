---
title: The Ricing Guide
description: An Overview into how to rice, from the very beginning till the "end".
date: 18 Feb 2024
author: Namish 
draft: false
category: guide 
---

![freosan](/static/images/freosan.png)

## Introduction

Ricing, a term commonly used for making your boring ass linux desktop into a treat for your eyes. But how do you even begin one? Well while this is not a _masterclass_ of some sorts, this should at the very least give you some idea as to how to begin your perfect rice. Here are a list of things we will go through while doing this.

```md
// todo.md 
## Ricing Todo
- [ ] Choosing a window manager
- [ ] Terminal
- [ ] The Aesthetics
- [ ] Colorschemes
- [ ] Icons and Cursors
- [ ] Wallpapers
- [ ] Compositing
- [ ] The Bars And Widgets
- [ ] Terminal Eyecandy
```

So before we begin here is a small disclaimer

> This is not a **STEP BY STEP TUTORIAL** on how to rice. This is a _GUIDE_ and only aims to assist you.

## Choosing A Window Manager
First, what is a window manager? In contrast to a DE, which provides a whole user experience with panels, taskbars, systrays, desktop icons, menus and stuff, windows managers usually just come with the ability to do basic window operations such as moving, resizing, and minimizing // maximizing (except one). Now I assuming you know what display protocols are and what is the difference between `Wayland` and `Xorg // X11`. If you do not, I would suggest you to [read this](https://www.howtogeek.com/900698/what-is-wayland-on-linux-and-how-is-it-different-from-x/). In wayland, a window manager if referred to as a `wayland compositor` or just `compositor` in short. Now here are the different WMs / compositors you can choose as an beginner.

### Xorg

- [awesome](https://awesomewm.org/): May be the best  X11 WM. Probably the most featureful WM that comes with an inbuilt bar, systray, appmenu, context menu, and various layouts. What makes AwesomeWM truly stand out is its Lua scripting support, allowing users to tailor every aspect of their desktop experience to their liking. Be aware of the steep learning climb though. If you are starting your own awesomewm configuration from scratch it is highly recommended to use [this kit](https://github.com/Gwynsav/modular-awm-default) as a good default configuration.

- [bspwm](https://github.com/baskerville/bspwm): My recommendation for beginners. The unique approach of dividing the screen into binary space partitions enables users to create a well-organized and intuitive workspace. It offers users a lightweight and highly responsive window manager that focuses on functionality without unnecessary frills.

- [dwm](https://dwm.suckless.org/): Now this WM is a bit unique. It follows a bit controversial [suckless philosophy](https://suckless.org/philosophy/), which aims at the software being as barebones and minimal as possible. The only way to customize any suckless software like dwm is to directly edit its source code. While this can be a fun little activity, I will not recommend dwm to absolute newbies. A shortcut to easily patch dwm would be to use [dwm-flexipatch](https://github.com/bakkeby/dwm-flexipatch).

- [qtile](https://qtile.org/): Qtile is simple and very user friendly. The use of Python as a configuration language makes Qtile accessible to a broader audience, enabling users to personalize their workspace with ease. It has a strong builtin bar that can be molded to almost anything requested by the user. Also it is one the rare WMs on the planet that actually work on both wayland and x11.

### Wayland 

- [hyprland](https://hyprland.org/): Probably the most popular wayland compositor in the unixporn circle and for the right reasons. Its light, low on resources, but can be configured endlessly and with ease. Comes with _smoooth_ window movement and workspace changing transitions that for some is a quite a deal breaker. Can be extended to have more fancy border, titlebars and loads of other things via community made plugins.

- [swayfx](https://github.com/WillPower3309/swayfx): A fork of sway but much better. Sway is just a pure [i3](https://i3wm.org) replacement for wayland. But sway never adds any extra feature on their own, they just do what i3 does. Swayfx is a fork that adds eyecandy such as rounded-corners that are anti aliased, blurring, shadows, dimming of unfocused windows, **per application saturation control** and possible fade and window animations.

> These are just my recommendations for beginners, there are many other compositors/wms available and you are free to use those.

## Terminals
Almost any temrminal can look good with your rice. It does not really matter but here are my only two recommendations for a terminal emulator if you ever feel confused.

- [wezterm](https://github.com/wez/wezterm) (written in rust btw 🚀) : My favourite terminal. Very easy to configure (it is in `lua`) and comes with a lot of features built in. Terminal hot reloads if the config is changed. Comes with image support, automatic powerglyph rendering, ligature support and has near perfect font rendering. Works on both wayland and x11, though for wayland and some more features you will need to use the git version. The only downside is that its a bit resource extensive, though it is still miles better than kitty.

- [alacritty](https://github.com/alacritty/alacritty) (also, written in rust btw 🚀): Very easy to use and understand and is also easy to configure. Alacritty is configured via `toml`. Does not support ligatures and powerglyphs, but has good font rendering. It is significantly more resource-ful than wezterm so its a good alternative for low end systems.

## The Aesthetics

Alright now we get into actual ricing stuff. Now to choose the "looks" or "aesthetic" of the rice. This is what will makes your rice truly unique. Here are some of the common aesthetics that I have come by so far -

### Modern // DE Derivative
Now what defines a 'modern' theme. A modern theme usually plays with small rounded corners (or none), two to three colors for background and only 1 accent color. They usually have very dark background colors and try to replicate the looks of a desktop environment. Anything like buttons or components on widgets have a alternate background color which is usually 10-15% lighter than the background color. Usage of other colors like "red" or "green" are only done where its "widely accepted", for example red for close button and green for battery. They prefer to have walls that are a bit towards the lighter side.

For this, its to preferred to use sans serif fonts like [Lexend](https://fonts.google.com/specimen/Lexend), [Gabarito](https://fonts.google.com/specimen/Gabarito) or [Rubik](https://fonts.google.com/specimen/Rubik).

![syndrizlle/fvvm](https://i.imgur.com/7lG8QRE.jpg)

<div align="center">

  [syndrizzle/hotfiles at fvwm](https://github.com/Syndrizzle/hotfiles/tree/fvwm)

</div>

![chadcat7/crystal](/static/images/crystal.png)
<div align="center">

  [chadcat7/crystal at aura](https://github.com/chadcat7/crystal/tree/aura)

</div>

### Material

Now material rices are based on Google's [Material UI](https://m3.material.io).

![material design](https://lh3.googleusercontent.com/pIWcLbKzn27wEbtlCB2X9Tkd_N5YeVF4YXFjIitrd_dIq8fwIJQD8CLpeKNdrZoTBPk8lMnkRBeB-IgUfaCXLj_qpd1ogFTokmKL1mV4TXXA=s0)

The number of colors included in this are significantly more than your average DE derivative, with there being many shades of background colors and foreground colors. Apart from once accent color, material schemes also have a secondary color and a tertiary color. Buttons and components are much more rounded but it still has the capability to retain its 'modern look'. Best font to use with it is again, Lexend. The below rice is probably one of the cleanest executions of material in ricing we'll ever see.

![end-4/dots-hyprland](https://end-4.github.io/dots-hyprland-wiki/screenshots/i-i.1.png)

<div align="center">

  [end-4/dots-hyprland at illogical-impulse](https://github.com/end-4/dots-hyprland)

</div>

For material its often recommended to generate your colors using [matugen](https://github.com/InioX/matugen) instead of cherry picking the colors. You can do this by -

```bash
$ matugen --dry-run image path/to/wallpaper.png --json hex
```

### Cozy // Comfy
Now this kind of rice is subjective to all. Each person has thier own view on a comfy rice. The colorschemes used in these rices are rather simple and not vibrant. Stardust's AwesomeWM rice is a good example of a comfortable and cozy rice. 

![sakura](https://star.is-a.dev/src/sakura.png)

<div align="center">

  [Stardust-kyun/sakura](https://github.com/Stardust-kyun/dotfiles)

</div>

Sometimes use of a "pixelated" design is also seen. Pixelated rices are bit tricky to pull off because of the lack of good pixelated fonts. Beck's pixel rice is a good example of this

![beck](https://preview.redd.it/awesomewm-pixels-v0-k7rebm5h0h7c1.png?width=1080&crop=smart&auto=webp&s=9f68842d85a7281b34c105e7fe1462b83ff201f3) 

### Minimalism
Now these rices are the bare minimum and mostly just a bar with workspaces and some indicators. Mostly driven by the words, "less is more". Does not contain widgets, but if they do, it is mostly some settings buttons and a music indicator.

![neroz](https://i.imgur.com/nvXb9KN.png)

### Minimalist+
These rices go full on minimalism. They only contain a bar (mostly waybar / polybar or the dwm bar) and mostly are "everything is bloat" kind of guys. Mostly made by people who run artix (not arch, systemd is bloat)

![my dwm rice](https://raw.githubusercontent.com/dark-Jedi2108/bedwm/main/.github/screenshots/n1.png)
<div align="center">

  [my dwm rice](https://github.com/chadcat7/dwm)

</div> 

## Colorschemes

The next big task to be done is choosing a colorscheme for your rice. Now you are free to make your own colorscheme and its ports for different applications, but if you are just starting out to rice, better alternative would be to use an existing colorschemes. The colorscheme to use mainly depends on what aesthetic you want your rice to be. If you want a modern DE rice, its better to have rices with backgrounds that are either very dark, like [carburetor](https://github.com/ozwaldorf/carburetor) or light themed like in this [screenshot](https://star.is-a.dev/src/awm/bidule.png). Wanna have something warm and cozy, use colorschemes with browner shades, like [biscuit](https://github.com/Biscuit-Colorscheme), [gruvbox-material](https://github.com/sainnhe/gruvbox-material) or [swamp](https://github.com/masroof-maindak/swamp.nvim).
<br/>
There are some themes which I feel goes with every kind of rice, like [decay](https://github.com/decaycs), [catppuccin](https://github.com/catppuccin/catppuccin) (heavily overused in the unixporn community), [everblush](https://github.com/Everblush) and javacafe's [ghost](https://github.com/chadcat7/crystal/blob/main/home/shared/cols/ghost.nix). To make your rice more unique, it's better to use colorschmes that are rarely used. For that you might have to make ports yourself but it pays off. I have already talked about custom themes in nix [in this blog](/blog/nixos). For other distros, mainly you only need to make additional ports for GTK and Neovim. Custom GTK themes can be easily made by tweaking around the `_colors.scss` file in [phocus](https://github.com/phocus/gtk). Neovim themes can be made easily with my own plugin [prism](https://github.com/chadcat7/prism). Here are some underrated colorschmes I will recommend. 

```md title="Good Themes"
1. ghost by javacafe01
2. biscuit by tsukki9696
3. fullerene by gw
4. oxocarbon by shaunsingh
5. swamp by masroof-maindak 
6. everblush by mangshrex
```

### Generating Colors

![pywal](https://preview.redd.it/26971w3g82501.png?width=1080&crop=smart&auto=webp&s=91ef72a0c6e8739c9ea71b7a86f086afadcee2a4)

You can also have dynamic colorschemes based on the wallpaper you have. Currently, the most popular tool for this is [pywal](https://github.com/dylanaraps/pywal). Using it is as simple as 

```bash
$ pip3 install pywal
$ wal -i /path/to/wallpaper.png
```

There is another program called [matugen](https://github.com/InioX/matugen/wiki) which generates colors based on the material. While pywal also generates multiple templates for the colorscheme it generated, matugen will only provide you with values, it is up to your job to make a [script](https://github.com/chadcat7/crystal/blob/main/home/shared/bin/theme/material.nix) to use them.

The problem with using generated colors is that often, they either generate very generic colorschmes or very monochrome colorschemes.

## Icons and Cursors

Choosing an icon pack and cursor theme is very subjective to the rice you want. For most rices, [papirus icon theme](https://github.com/PapirusDevelopmentTeam/papirus-icon-theme) works just fine and there is no need to search for another. However if you want some "modern" looking icons, you can go for [reversal](https://github.com/yeyushengfan258/Reversal-icon-theme) or just the [whitesur icon theme](https://github.com/vinceliuice/WhiteSur-icon-theme).

For cursors, unless you have a super specific niche theme in mind, you can just use [phinger cursors](https://github.com/phisch/phinger-cursors) or [bibata](https://github.com/ful1e5/Bibata_Cursor), they work for most aesthetics / themes.

## Wallpapers 
Now this will be a very short section. Because a good wallpaper is a very subjective topic. Wallpaper bascially adds character to your rice. You are free to use whatever wallpaper you like as long as it matches the colorscheme. If you can not find a good wallpaper, here are some places to find them. 

```md title="Good Walls"
1. https://reddit.com/r/wallpaper (was very good earlier, now is filled with AI trash)
2. https://pixiv.net
3. https://wallhaven.cc 
4. https://github.com/dharmx/walls
5. https://github.com/Gwynsav/walls
6. https://github.com/FrenzyExists/wallpapers 
7. The unixporn discord server (very good source)
``` 

If you still cant a wall matching your scheme, you can make the wall you like match your scheme with [lutgen](https://github.com/ozwaldorf/lutgen-rs). 

Keep in mind that it is not always necessary to use a very artistic, beautiful wall, sometimes a tiling wallpaper is all you need.

![nuxsh](https://github.com/nuxshed/dotfiles/wiki/media/cafe2-1.png)
<div align="center">

  [nuxshed/dotfiles](https://github.com/nuxshed/dotfiles)

</div> 

Or sometimes, just a plain color.

![kizu](/static/images/kizu.png)
<div align="center">

  old rice by [janleigh](https://github.com/janleigh/dotfiles)

</div> 


## Compositing
Now I am not a graphics expert or something and do not know the literal defination of compositing. But compositing in ricing usually stands for adding efects like blurs, shadows, corner radius and animations to windows and workspaces. This generally happens with software called `compositors`. In wayland, compositors are in build with the WM you are running (hence are called wayland compositors). In X, you need to install a software for that, most commonly, [picom](https://github.com/yshui/picom). Now if you only want stuff like blurring, corners and shadows, the orignal picom should work fine. But now there are many forks that implement window animations. Window animations many a times come with the cost of slow performance. I was able to find [this fork of picom](https://github.com/fdev31/picom/tree/animation-pr) which was much more resource efficient than others.

<br/>

Its always recommended to also use blur if you are decreasing the opacity of windows, as that makes the text easier to read. It is difficult to find the sweet spot where it should neither look too opaque or too transparent. 

![unixporn](https://preview.redd.it/hyprland-endeavour-os-bussin-v0-jl29adbqwnkc1.jpeg?width=1080&crop=smart&auto=webp&s=41e4f43b5a690983aefaac906b3f9d03c43b1625)

This terminal hits a good spot and is pleasant to look at.

## The Bars And Widgets
You know 'em, You lov 'em. Widgets. Widgets are a way to actually elevate your rice from plain, bland and ordinary to something extraordinary. Here are your options for bars and widgets.

### Wibox 
The wibox library is inbuilt in AwesomeWM, and is only accessible via awesome only. With other builtin libraries like `naughty`, `awful`, wibox becomes insanely powerful and can be used to built almost anything. Kasper's config might the best example of that.

![kwesome](https://github.com/Kasper24/KwesomeDE/raw/main/.github/assets/9.png)
<div align="center">

   [kasper24/kwesomede](https://github.com/Kasper24/KwesomeDE)

</div> 

People have used wibox to build lockscreens, docks, application launchers, bars, wifi menu, right click menus, notifications, financial ledger and even add custom titlebars to make minimalist programs like NCMPCPP, feh more usable. Has a bit of a steep learning curve and docs may not be enough for newbies.

### EWW (Elkowar's Wacky Widgets)
The OG program that brought widgets to other WMs. Use it only if you are using a X11 WM that is not AwesomeWM. Has a very limited range of GTK components which should be enough to make a good looking desktop. The widgets are defined in its own lisp like language called [yuck](https://github.com/elkowar/yuck.vim) and styled via SASS. The only downsides are no systray (yet) and poor documentation.

<Callout type="warning"> 
As of 30th March 2024, the systray pull request is merged  
</Callout>

![eww dash](https://github.com/dharmx/vile/raw/main/.github/readme/demo.png)
<div align="center">

   [dharmx/vile](https://github.com/dharmx/vile)

</div> 


### AGS (Aylur's Gtk Shell)

> this part about ags is outdated, please checkout [astal](https://github.com/Aylur/astal) for the latest updates.

The new cool kid in town which should take the wayland ricing community by storm. Its fairly new and is being actively worked upon. Learning curve is easy because javascript is easy. Has multiple inbuilt widgets, but really it can give you access to all GTK widgets possible, a major plus side against EWW. Also has systray built in, and more number of CSS properties supported. But the main thing that makes it miles better than EWW is the number of inbuilt services and utility functions. 

<br/>
Currently fully suppports Hyprland. There is a fork by [ozwaldorf](https://github.com/ozwaldorf/ags) which brings support for Sway but is yet to be merged. No other wayland compositor is yet fully supported with services.

![end-4/dots-hyprland](https://raw.githubusercontent.com/chadcat7/crystal/freosan/.github/image.png)

<div align="center">

  [chadcat7/crystal at freosan](https://github.com/chadcat7/crystal/tree/freosan)

</div>

### Special Mention - Fabric

In dead simple words, [Fabric](https://github.com/Fabric-Development/fabric) is like AGS but with python. It is still not fully ready to be used and I myself have not tried it yet. So I cannot judge it right now. It claims to work on both X11 and wayland and Python can make it very easy to configure. This looks promising but cannot say much yet. 

![fabric](https://github.com/Fabric-Development/fabric/raw/main/assets/example-files-bar-showcase.png)
<div align="center">

  [fabric](https://github.com/Fabric-Development/fabric)

</div> 
<br/>
### Polybar // Waybar 

What if you do not want the complexity of these widgets and just want a simple normal rice. You can simply use polybar for x11 and waybar for wayland. They are very limited in terms of functionality, (only colored text for that matter) but might be enough to get your work done. Waybar has an advantage with tasklist, usage of css to style and the functionality to make vertical bars. They can also be used to execute well polished and modern bars as seen below -

![polybar by syndrizzle](https://i.imgur.com/EMuaeIv.png)
<div align="center">

   [polybar by syndrizzle](https://github.com/syndrizzle/hotfiles/tree/bspwm)

</div> 

![waybar](https://preview.redd.it/hyprland-glassmorphism-v0-uu7qkonc4az91.png?width=1080&crop=smart&auto=webp&s=4671305853af21742b1d87ffe0d4a3b9b6eb150a)
<div align="center">
waybar by rxyhn
</div>

### DWM bar

![chadwm](https://github.com/siduck/chadwm/blob/screenshots/screenshots/initial_look.png?raw=true)
<div align="center">

   [chadwm by siduck](https://github.com/siduck/chadwm)

</div> 

If you use dwm, then you should strictly stick to the inbuilt dwmbar and extend it with patches and stuff. Here are a list of patches you can apply to make your dwm bar look good and functional -

```md title="Good Patches"
 1. status2d (must)
 2. systray (must) 
 3. awesomebar 
 4. statusbutton 
 5. statuspadding
 6. alt-tags-decoration 
 7. statuscmd-nosignal (click functionality) 
 8. hidevacant (i3 like functionality) 
 9. underlinetags 
10. rainbowtags
```

## Terminal Eyecandy

![manas/sh](https://github.com/Manas140/sh/blob/main/preview.png?raw=true)
<div align="center">

  [Manas140/sh](https://github.com/Manas140/sh)

</div>

Another big part of your next r/unixporn post is how you display the terminals you have in there. Now you can do that in a number of ways. 

### Color Scripts
These are usually bash scripts written to display the different colors in the terminal. You can find a bunch of them in this [repo by Manas](https://github.com/Manas140/sh) or in [colorscripts](https://github.com/stark/Color-Scripts).

### Fetch Programs
I do not think we need to discuss what a `fetch` program does. It is a staple of all screenshots in unixporn because it looks cool and is actually a pretty 
good way to display information about your system. 

![neofetch](https://i.imgur.com/GFmC5Ad.png) 
<div align="center">

  [dylanaraps/neofetch](https://github.com/dylanaraps/neofetch)

</div>

Here are some of the popular fetch scripts. 

1. [neofetch](https://github.com/dylanaraps/neofetch) - the classic fetch script. also has great support for images.
2. [fastfetch](https://github.com/fastfetch-cli/fastfetch) - neofetch written in c instead of bash, makes it significantly more faster. 
3. [pfetch](https://github.com/dylanaraps/pfetch) - a smaller version of neofetch by the same author. 
4. [nitch](https://github.com/ssleert/nitch) - a fetch script written in nim.
5. [bunnyfetch](https://github.com/Rosettea/bunnyfetch) - cutest fetch ever.

### Other CLI Programs

Here are a list of other cli programs that you might have come by in unixporn screenshots -
1. [ncmpcpp](https://github.com/ncmpcpp/ncmpcpp) - mpd player with inbuilt visualizer.
2. [cava](https://github.com/karlstav/cava) - most popular terminal visualizer.
3. [cmatrix](https://github.com/abishekvashok/cmatrix) - terminal based matrix implmentation.
4. [cbonsai](https://gitlab.com/jallbrit/cbonsai) - bonsai trees in your terminal .
5. [tty-clock](https://github.com/xorg62/tty-clock) - clock program.

### Prompt
Terminal prompts can massively enhance the look of your terminal. There is a common myth that a temrinal prompt is based on what shell you use. No, you can make your prompt look like anything with any shell. By setting the prompt you are basically just editing the `$PS1` environment variable. 

```zsh
$ export PS1="$ > "
$ > echo "HI"
```

There are utilities like [oh-my-zsh and powerlevel10k](https://earthly.dev/blog/powerline10k/) which can make your prompt look "cool" but the tool I recommend for beginners is [starship](https://starship.rs/). It is pretty easy to setup and has support for almost all shells.

### Special Mention
**Terminal padding**: Terminal padding can make your terminal look 100x better instantly. I recommend having atleast 16px padding on terminals (my preference is 32px).

## Some Good Dotfile Repos and Resources 

Here is the [orignal ricing guide](https://nes.is-a.dev/ricing-guide/) of unixporn. Pretty outdated but is still worth a read!

This list is made keeping in mind that the reader is new to ricing and the repository and dotfiles are easy to read and understand. 

### Awesome
1. [Stardust-kyun/dotfiles](https://github.com/Stardust-kyun/dotfiles) - A very solid awesomewm configuration which is not hard to read and understand, perfect for beginners to get a better understanding of awesomewm.
2. [chadcat7/crystal](https://github.com/chadcat7/crystal) - Self shilling here (/s). I do belive my awesome code is pretty easy to understand and execute into your own dotfiles since I really do not have any complex or crazy file structure in which each file depends on 10 different files.
3. [AlphaTechnolog/dotfiles](https://github.com/AlphaTechnolog/dotfiles) - Another really good awesome dotfile repo, cleanly organized and easy to understand. 
### Polybar Configs
1. [adi1090x/polybar-themes](https://github.com/adi1090x/polybar-themes) - Huge repo with tons of premade themes for polybar.

### EWW Configs
1. [saimoomedits/eww-widgets](https://github.com/saimoomedits/eww-widgets) - A very good repo for just learning the basics of eww
2. [Syndrizzle/hotfiles](https://github.com/Syndrizzle/hotfiles/) - Very helpful and advanced widgets
3. [dharmx/vile](https://github.com/dharmx/vile) - Not very easy to read but a good place to get scripts for your rice
4. [Failedex/CarbonMonoxide](https://github.com/Failedex/CarbonMonoxide) - Currently the best eww rice, with a lot of advanced widgets

### AGS Configs
1. [end-4/dots-hyprland](https://github.com/end-4/dots-hyprland) - Already mentioned in here.
2. [Aylur/dotfiles](https://github.com/Aylur/dotfiles) - Dotfiles by the creator of AGS himself. Pretty good repo to learn the basics of ags, and all the things that are available in it.

### Good Custom Schemes 
1. [Biscuit](https://github.com/Biscuit-Colorscheme)
2. [Carburetor](https://github.com/ozwaldorf/carburetor)
3. [Oxocarbon](https://github.com/nyoom-engineering/oxocarbon.nvim)
4. [Rosepine](https://github.com/rose-pine/neovim)
5. [Swamp](https://github.com/masroof-maindak/swamp.nvim)
6. [Everblush](https://github.com/Everblush)

## Conclusion
Well that was long. But hopefully this makes you aware about the options you have when starting out to rice and makes it easy to choose your _ricing stack_. Till next time. See ya!
