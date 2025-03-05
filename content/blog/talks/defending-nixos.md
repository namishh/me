---
title: Defending Nix OS 
description: (Small rant). Defending Nix OS from the funny criticism it gets sometimes.
date: 13 Mar 2024
draft: false
author: Namish 
category: talks
---

![wall](https://github.com/NixOS/nixos-artwork/blob/master/wallpapers/nix-wallpaper-gear.png?raw=true)

Now I have been using Nix OS for almost 1 year now. A crazy journey and kind of glad I took it. But some people in the unixporn discord server and some of my friends call my deicision of using Nix OS "dumb" and a "waste of time". Well this post will be my trying my best to defend NixOS.

### Configuration Via A Functional Language Is Dumb.

First of all, no it's not. The concept and idea of a functional language is not only "calculus" and "lambda functions" like the haskell bros portray it to be. The concept of functional language is simple, nothing affects the output. In a function, you input a value and no matter what, the output will always be the same. There is no "state" that can vary the output of the configuration you write. Either it works, or it is a skill issue. No matter which system it is running on, a NixOS configuration will produce the same system, thus making it truly reproducible.

### Why Not Use Some Other Language Specifically For Configuration?

Now one of my friends actually said that NixOS should use something like [pkl](https://pkl-lang.org/) or yaml to configure nix, which is very dumb. They really think of NixOS as something which can be configured only via booleans and string values. The nix language is turing complete. It can be used to do complex things. I do not see any string interpolation in yaml, or any type of import statement, or functions.

### None Of My Shit Works! Help!

Yea, it will not. If you use NixOS for the hype train and then just quit within 4 hours because shit does not work out of the box, yea it will not work. Learn how home-manager works instead of "lx-appearance does not work". Learn how dev shells work instead of crying about a LSP not working. Learn about derivations instead of "it is not available for nix". This is just a skill issue.
 
### Can't I Just Make A Shell Script For Reproducible Setups?
You can but at the end for how many distros. You cannot cover each distro in your shell script? Nix on the other hand works on every distro. NixOS manages dependencies explicitly, ensuring that the system's configuration includes all necessary packages and versions. This eliminates the risk of missing dependencies or conflicts that may occur with shell scripts. With Nix OS, configuration changes are atomic and can be rolled back easily. If a change causes issues, you can revert to the previous state reliably. Shell scripts lack this built-in capability for atomicity and rollback. 

Yea that's just it, if I find someone doing any other dumb argument, I will add it here. For Now, Goodbye!
