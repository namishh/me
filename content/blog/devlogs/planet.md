---
title: Generating Planets
date: 04 April 2025
draft: true 
---

## Introduction

> not a tutorial, just a devlog

This is a small project I am working on to generate planets using voronoi graphs. The idea is to use the voronoi graph to create continents and then add some noise to make it look more realistic. I will be using the [Odin](https://odin-lang.com/) language (sorry for no syntax highlighting) and raylib for this project.

## Vonoroi Graph

![voronoi](https://www.kdnuggets.com/wp-content/uploads/arya_quick_overview_voronoi_diagrams_5.jpg)

It has a very simple definition, given a set of points, the goal is to create a polygon around each point such that it encloses the region of space closest to that point.

So while developing, here was my first primitive idea, generate a bunch of random points in the shape of a sphere, and then connect them with lines to kind of form a big vornoi sphere. 

### Side Quest - Randomness

Since I wanted to go a step further, I wanted to even create my own randomness. Now since computational randomness is not really worth the effort. I decided to rather write a well known pseudorandom number generator. LCG is well known, so I wanted to learn something new. I went with the [xorshift](https://en.wikipedia.org/wiki/Xorshift) algorithm. It is a very simple algorithm and is very fast.

```odin
rand_state: u64 = 1234567890

rand_next :: proc() -> u64 {
    x := rand_state
    x ~= x << 13
    x ~= x >> 7
    x ~= x << 17
    rand_state = x
    return x
}

// some example of how we can use it
rand_u32 :: proc() -> u32 {
    return u32(rand_next())
}

rand_float32 :: proc() -> f32 {
    return f32(rand_u32()) / 4294967296.0 // 2^32
}

rand_float32_range :: proc(min, max: f32) -> f32 {
    return min + (max - min) * rand_float32()
}

rand_int_max :: proc(max: int) -> int {
    return int(rand_u32() % u32(max))
}
```

Now since I am using a PRNG, it means that "randomness" here is just a bunch of arbritrary mathematical transformations and using the resultant number as the seed for the next number. This means that for a specific seed, the output will always be the same. Now many projects use the current time as the seed, so if you want to generate a different sphere every time, just use

```odin
import "core:time" 

rand_state: u64

init_random :: proc() {
    now := time.now()
    nano := time.duration_nanoseconds(time.diff(time.Time{}, now))
    rand_state = u64(nano) | 1 
}
```

<br>

The operations are very simple, I just shift the bits of the number and then xor them. First I shift the bits to the left by 13, then I shift the bits to the right by 7, and then I shift the bits to the left by 17.

### Back to the sphere

Well, let us just generate a bunch of random points. The very first thing was to just types called Point and Edge.

```odin
Point :: struct {
    position: rl.Vector3,
    color:    rl.Color,
    index:    int,
}

Edge :: struct {
    start, end: int,
}
```

So to not make everything seem like a white blob, I, for now will give them a randomly generated color. 

<br>

We want the points to be evenly distributed in the sphere, so for that we will be using the [Fibonacci Sphere](https://stackoverflow.com/questions/9600801/evenly-distributing-n-points-on-a-sphere) method, which is a efficient way to generate evenly distributed n points on a sphere.


In the coordinate system, any point of the sphere can be represented as a mixture of radius, theta (azimutal angle) and phi (polar angle). So we create a function that takes in the gap between the points, and the radius of the sphere, generates the points in the polar coordinate system, and the converts them into the cartesian coordinate system, and returns an array of points.

The formula for converting polar coordinates to cartesian coordinates is:

```odin
x := radius * math.sin(phi) * math.cos(theta)
y := radius * math.cos(phi)
z := radius * math.sin(phi) * math.sin(theta)
```

and here is the code for the whole function: 

```odin
generate_sphere_points :: proc(radius: f32, min_gap: f32) -> [dynamic]Point {
    points := make([dynamic]Point)
    approximate_points := int(4 * rl.PI * radius * radius / (min_gap * min_gap))
    phi := (1.0 + math.sqrt_f16(5.0)) / 2.0 
    reserve(&points, approximate_points)
    i := 0
    for {
        y := 1.0 - (2.0 * f32(i) + 1.0) / f32(approximate_points)
        r := math.sqrt(1.0 - y * y)
        
        phi_angle := 2.0 * rl.PI * f32(i) / f32(phi)
        
        x := r * math.cos(phi_angle)
        z := r * math.sin(phi_angle)
        
        new_point := Point {
            position = rl.Vector3{x * radius, y * radius, z * radius},
            color = rl.Color{
                u8(rand_int_max(200) + 55),
                u8(rand_int_max(200) + 55),
                u8(rand_int_max(200) + 55),
                255,
            },
            index = i,
        }
        
        too_close := false
        for j in 0..<len(points) {
            if distance(new_point.position, points[j].position) < min_gap {
                too_close = true
                break
            }
        }
        
        if !too_close {
            append(&points, new_point)
        }
        
        i += 1
        
        if i >= approximate_points * 2 || len(points) >= approximate_points {
            break
        }
    }
    
    return points
}
```



For now lets draw a small sphere in place of the points. Add this in the main loop:

```odin add={4-6, 9-10}
// define points above
rl.BeginMode3D(camera)
    
// DO STUFF HERE 
for point in points { // assume points is a slice of Point
    rl.DrawSphere(point.position, 0.1, point.color)
}

rl.EndMode3D()
rl.DrawFPS(10, 10)
rl.DrawText(fmt.ctprintf("Points: %d", len(points)), 10, 40, 20, rl.WHITE)
```

![/static/images/planet-simple-spheres.png](https://u.cubeupload.com/namishhhh/Screenshot2025040502.png)


### Rotating the sphere

Now this is a very simple task

```odin add={3-9}
rl.UpdateCamera(&camera, .ORBITAL)

rotation_speed := 0.005
for i in 0..<len(points) {
    x := f64(points[i].position.x)
    z := f64(points[i].position.z)
    points[i].position.x = f32(x * math.cos(rotation_speed) - z * math.sin(rotation_speed))
    points[i].position.z = f32(x * math.sin(rotation_speed) + z * math.cos(rotation_speed))
}

rl.BeginDrawing()
```

What this does is just constantly update the x and z coordinates of the points. The y coordinate is not changed, so it will just rotate around the y axis.


### Continents

Now here is where the actual voronoi graph comes it. The idea is to pick 8 to 10 points at random from the points we have generated, and then create a voronoi graph around them. These regions will indicate continets.


![continents](https://u.cubeupload.com/namishhhh/Screenshot2025040521.png)

To do this, first I created a function that generates n random points to act as the centers of the continents. In the above screenshot, I have only used 4 points, so the continents look kinda well defined in the image.







#### roadmap (only for writing purposes, delete later)

```md
- [x] start with odin template
- [x] 3d vonoroi graph

- [ ] generate points to make continents
- [ ] draw the outlines of the continents
- [ ] generate small subcontinents
  
- [ ] add layers of noise to make it look more realistic
- [ ] add clouds and atmosphere

- [ ] make slider for threshold of noise
```