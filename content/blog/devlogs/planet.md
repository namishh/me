---
title: Generating Planets
date: 04 April 2025
draft: true 
---

## Introduction

> not a tutorial, just a devlog

This is a small project I am working on to generate planets using voronoi graphs. The idea is to use the voronoi graph to create tectonic plates and then add some noise to make it look more realistic. I will be using the [Odin](https://odin-lang.com/) language (sorry for no syntax highlighting) and raylib for this project.

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


### Tectonic Plates 

Now here is where the actual voronoi graph comes it. The idea is to pick 8 to 10 points at random from the points we have generated, and then create a voronoi graph around them. These regions will indicate tectonic plates.


![continents](https://u.cubeupload.com/namishhhh/Screenshot2025040521.png)

To do this, first I created a function that generates n random points to act as the centers of the tectonic plates. In the above screenshot, I have only used 4 points, so the tectonic plates look kinda well defined in the image.

The way the function work if that before picking, it divides the points into regions to ensure that points are evenly distributed. It's pseudo code would kinda look like:

```odin
region_size := len(points) / num_centers

for i in 0 ..< num_centers {
    region_start := i * region_size
    region_end := min(region_start + region_size, len(points))

    if region_start >= region_end {
        continue 
    }

    // use the random number generator to pick a point from the region and add it to the centers
}
```

After this, all we are left to do to "define" the tectonic plates is to assign each point to the nearest center. This was a relatively simple task, although my approach in slower and not recommmended for serious projects. It is a nearest neighbour's approach, so for each point, we just check the distance to all the centers with the classic distance formula and assign `point.tectonic_plate_id` to the id of the nearest center. With this we also change the color of the point to the color of the center and we have successfully recreated the screenshot.

### Tectonic Edges

The last thing to create a basic planet is to create the edges of the tectonic plates, which I will admit took me some time to figure out. My first few attempts were just drawing lines between spheres of the same continent, but that resulted in a lot of edges crossing each other, and some points just being left alone. Also to remind you guys, I run this on a 4gb ram laptop, so drawing these many lines was not good for my poor laptop.

My next approach was to create a list of edges, which is calculated by seeing how far it is from the center of the plate. This attempt worked more or less, I was getting distinct edges that one can visually decipher to be a plate, but there was still a problem.  


After a lot and lot of deliberating, and pleading to the free tiers of claude, I got to a this point:

![edges-but-broken](https://u.cubeupload.com/namishhhh/Screenshot2025040922.png)

Now not a bad result, but there are gaps between the plates, and some edges were just distorted. I tried reading what claude had done but well I will just say that it was a complete mess of a code that I could not understand. Given up I decided to just start again, but with a different approach.

## Goldberg Polyhedra

![goldberg](https://u.cubeupload.com/namishhhh/Screenshot2025041014.png)

This time instead of making points, I will just directly make edges in form of a sphere because I learnt about Goldberg polyhedra. The idea is to just take a sphere and divide it into smaller triangles, and for more detailing, you can subdivide it again with its own vertices how many times you want. And if you group these triangles together, you can form smaller hexagons and pentagons, which I will treat as a "location" or a part of plate.

So, now instead of a Point struct I have a Planet struct, which in turn contains arrays of strcuts Edges, Vertices and Faces.

```odin title="New-structure"
Vertex :: struct {
    position: rl.Vector3,
    normal: rl.Vector3,
}

Face :: struct {
    vertices: [dynamic]int,
    center: rl.Vector3,
    normal: rl.Vector3,
    color: rl.Color,
    is_pentagon: bool,
    region_id: int,
}

Edge :: struct {
    v1, v2: int, // vertex indices
    face1, face2: int, // face indices this edge belongs to (-1 if only belongs to one face)
}

Planet :: struct {
    faces: [dynamic]Face,
    vertices: [dynamic]Vertex,
    edges: [dynamic]Edge,
    radius: f32,
}
```

### Icosahedron 

The first step is to create a base [icosahedron](https://en.wikipedia.org/wiki/Icosahedron). It is a shape with 20 faces, 12 vertices and 30 edges. Now while I do not know how they were derived mathematically, I do know that the vertices are the permutations of `(±1, ±φ, 0)`, `(0, ±1, ±φ)` and `(±φ, 0, ±1)`, where `φ` is the golden ratio. The faces are just the combinations of these vertices. Golden ratio can be calculated with the simple formula `φ = (1 + sqrt(5)) / 2`. From now on, I will refer to the golden ratio as `t` or `phi`.

```odin
append(&planet.vertices, Vertex{
    position = rl.Vector3{normalized.x * radius, normalized.y * radius, normalized.z * radius},
    normal = normalized,
})
```

Then we can also just normalize them and extend them to the radius we want. Next I take the predefined 20 faces of the sphere which look like this:

```odin
faces := [?][3]int{
    {0, 11, 5}, {0, 5, 1}, {0, 1, 7}, {0, 7, 10}, {0, 10, 11},
    {1, 5, 9}, {5, 11, 4}, {11, 10, 2}, {10, 7, 6}, {7, 1, 8},
    {3, 9, 4}, {3, 4, 2}, {3, 2, 6}, {3, 6, 8}, {3, 8, 9},
    {4, 9, 5}, {2, 4, 11}, {6, 2, 10}, {8, 6, 7}, {9, 8, 1}
}
```

We just normalize the vertices according to the radius and make a face struct. The normal of the face can be just calculated by taking the cross product of two edges and then normalizing it. The center of the face can be calculated by taking the average of the vertices. 

For storing edges we create a `map[[2]int]int`. Now I took a lot of help for this and was kind of hard for me intuitively at first. So an edge can only be a part of 2 faces or at minumum 1 face. There are no edges that just floating and its not physically possible to get more than 2 faces without them intersecting. If there is only 1 face, we set the face2 as -1.

For each face, we iterate over it's three vertices in a loop. For each pair, v1 and v2, we sort them and check for an existing edge. If the edge exists, we set the face2 to the current face index. If it doesn't exist, we create a new edge and set the face1 to the current face index.

### Subdividing the faces

![subdividing](https://u.cubeupload.com/namishhhh/Screenshot2025041020.png)

The aim is to create a function that takes in a planet, and returns a new planet with each face subdivider. The idea is that then we can use this function multiple times to create more and more detailed planets.

The first step is to have a map to save the midpoints of the edges, `edge_midpoints := make(map[[2]int]int)`. Then we just loop through the faces, get the three midpoints of the edges and create the news faces. The four new faces have these coordinates

+ `v1`, `m12`, `m31`
+ `m12`, `v2`, `m23`
+ `m31`, `m23`, `v3`
+ `m12`, `m23`, `m31`

```odin
for face in planet.faces {
    v1 := face.vertices[0]
    v2 := face.vertices[1]
    v3 := face.vertices[2]
    
    m12 := get_or_create_midpoint(&result.vertices, &edge_midpoints, v1, v2, planet.radius)
    m23 := get_or_create_midpoint(&result.vertices, &edge_midpoints, v2, v3, planet.radius)
    m31 := get_or_create_midpoint(&result.vertices, &edge_midpoints, v3, v1, planet.radius)
    
    create_face(&result.vertices, &result.faces, v1, m12, m31, face.color, face.region_id)
    create_face(&result.vertices, &result.faces, m12, v2, m23, face.color, face.region_id)
    create_face(&result.vertices, &result.faces, m31, m23, v3, face.color, face.region_id)
    create_face(&result.vertices, &result.faces, m12, m23, m31, face.color, face.region_id)
}
```

+ The `get_or_create_midpoint` function checks if the midpoint already exists in the map. If it does, it returns the index of the midpoint. If it doesn't, it creates a new vertex at the midpoint and adds it to the planet's vertices. The midpoint is calculated by taking the average of the two vertices and normalizing it.
+ The `create_face` function creates a new face with the given vertices and adds it to the planet's faces. It also calculates the normal and center of the face.

Then we generate the edges for the new planet again, using the same method. Now we can just call this function multiple times to create more and more detailed planets. The more times you call it, the more detailed the planet will be.

```odin
PLANET_RADIUS :: 3.0

icosahedron := generate_icosahedron(PLANET_RADIUS)

subdivided := subdivide(&icosahedron)
subdivided = subdivide(&subdivided) 
subdivided = subdivide(&subdivided) 
subdivided = subdivide(&subdivided) 
```

### Dividing into hexagons and pentagons

This is the last step in generating a Goldberg polyhedra. For this the first step is to place new vertices at the center of faces. Then we build a mapping from each vertex index in the original planet to the indices of faces that share that vertex. We iterate over each face, and then for each vertex inside the face we create or add a mapping to the vertex. Then, each vertex in the original planet becomes a face in the dual graph.

The next step is to order the vertices of the new dual face (which are the centers of the original faces adjacent to the original vertex) to form a proper polygon. These vertices lie on the sphere, and connecting them in the wrong order would create a twisted or invalid face. Add this new face to the new planet's faces. And then yet again, we generate the edges for the new planet again, using the same method.

Well, this is the final result:

![image](https://u.cubeupload.com/namishhhh/Screenshot2025041023.png)

And then we can use our voronoi implmentations and slightly tweak them to make them work with this. Frist we shuffle all our faces and select 8 of them to be centers of tectonic plates. I am using the [Fisher-Yates shuffle](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle) algorithm to shuffle the faces. Then we just assign each face to the nearest center and set the color of the face to the color of the center.

```osin
select_random_plate_centers :: proc(planet: ^Planet, num_plates: int) -> []int {
    plate_count := num_plates
    if plate_count > len(planet.faces) {
        plate_count = len(planet.faces)
    }
    
    indices := make([]int, len(planet.faces))
    for i := 0; i < len(planet.faces); i += 1 {
        indices[i] = i
    }
   
    for i := len(indices) - 1; i > 0; i -= 1 {
        j := rand_int_max(i + 1)
        indices[i], indices[j] = indices[j], indices[i]
    }
    
    plate_centers := make([]int, plate_count)
    for i := 0; i < plate_count; i += 1 {
        plate_centers[i] = indices[i]
    }
    
    delete(indices)
    return plate_centers
}
```

Well and now we get beautifully colored tectonic plates and out original problem is now solved.

![image](https://u.cubeupload.com/namishhhh/cc4Screenshot2025041023.png)

The next steps will be realistically simulate the tetonic plates to create mountain ranges.