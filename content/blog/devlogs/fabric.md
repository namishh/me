---
title: Fabric From First Principles
date: 10 May 2025
draft: true
---

## Introduction

So I just got bored and then I decided to start some kind of a new hobby. The aim was to monthly, take a random paper on physics, or mathematics, and try to implement it and visualise it. Since I am too busy to continue work on my bigger projects, I thought this is a good way to keep in touch. 

<br>

Anyways, the first paper I chose is [Simulation of Clothing with Folds and Wrinkles](https://www.cs.ubc.ca/~rbridson/docs/cloth2003.pdf). Another thing I will do is to try my best to derive the equations from the first principles of physics, which in this context, is Newton's laws of motion.

### Setup

I will be using raylib for the rendering, and odin for the programming language (apologies for no syntax highting). So the first thing is to setup a "cloth", and that for now, will be just particles connected by triangles.

![image](https://u.cubeupload.com/namishhhh/Screenshot2025050916.png)

Structs for the particles and triangles are as follows:


```odin
Particle :: struct {
	position:    rl.Vector3,
	velocity:    rl.Vector3,
	forced:      rl.Vector3,
	mass:        f32,
	inverseMass: f32,
	fixed:       bool,
	index:       i32,
	color:       rl.Color,
}

Triangle :: struct {
	particles: [3]i32,
	normal:    rl.Vector3,
	area:      f32,
}
```

After that I just created a basic function, that generates a mesh and draws it. This is what I have for now.

![image](https://u.cubeupload.com/namishhhh/Screenshot2025050917.png)

## Derivation

Before deriving, I will just be describing the most basic laws and rules, so that the reader and I are on the same page.

### The Groundwork

So cloth simulation, just like every other newtonian system, is based on the laws of motion. The first law, or the law of interia mathematicaly states that:

$$ \sum F=0 \Rightarrow \frac{dv}{dt}=0 $$

In layman terms, if there is no force acting on an object, there is no change in its velocity. 

The second law, or the law of acceleration, states that:

$$ F=ma = m \frac{dv}{dt} = m \frac{d^2x}{dt^2} $$

This implies that the acceleration of an object is directly proportional to the net force acting on it, and inversely proportional to its mass.

The third law simply states that for every action, there is an equal and opposite reaction.

<br>

Another critical law in physics is the law of conservation of energy, which states that energy remains constant over time in a closed system. When it comes to cloth simulation, we will consider

1. Kinetic Energy, which is the energy of motion, and is given by the formula:

$$ KE = \frac{1}{2}mv^2 $$

2. Potential Energy, which in this case, instead of being configured by gravity and height, will be configured by a spring constant and the displacement. The formula is:

$$ PE = \frac{1}{2}kx^2 $$

Where $k$ is the spring constant, and $ x $ is the displacement from the rest position.
