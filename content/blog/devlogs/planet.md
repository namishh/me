---
title: Generating Planets
date: 14 April 2025
draft: false 
---

![img](https://u.cubeupload.com/namishhhh/744Screenshot2025041401.png)

## Introduction

> not a tutorial, just a devlog

This is a small project I am working on to generate planets using voronoi graphs. The idea is to use the voronoi graph to create tectonic plates and then add some noise to make it look more realistic. I will be using the [Odin](https://odin-lang.com/) language (sorry for no syntax highlighting) and raylib for this project.

## Vonoroi Graph

![voronoi](https://www.kdnuggets.com/wp-content/uploads/arya_quick_overview_voronoi_diagrams_5.jpg)

It has a very simple definition, given a set of points, the goal is to create a polygon around each point such that it encloses the region of space closest to that point.

So while developing, here was my first primitive idea, generate a bunch of random points in the shape of a sphere, and then connect them with lines to kind of form a big vornoi sphere. 

### Side Quest - Randomness

Since I wanted to go a step further, I wanted to even create my own randomness. Now since computational randomness is not really worth the effort. I decided to use any existing pseudo-random number generator. LCG is well known, so I wanted to learn something new. I went with the [xorshift](https://en.wikipedia.org/wiki/Xorshift) algorithm. It is a very simple algorithm and is very fast.

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

```odin
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

## World Modelling

Okay so now we actually get to the fun part. This will not be the most realistic simulation because I will be taking in account < 2% of the actual geography that exists. First thing, we divide the plates into 2 types, oceanic and continental, right now, using the proportions of earth, I have set it to be 60% oceanic and 40% water. 

### Mountain Ranges

Each plate moves in a random direction, and if two plates collide, we create a mountain range. The first thing we do is to give each plate a angular velocity and a rotation axis, for now, the rotation axis will be randomly generated unit vector.

```odin
rotation_axis := rand_unit_vector()

max_angular_velocity := 0.01
for i in 0 ..< CONTINENTS {
    if rand_float32() < 0.6 {
        plates[i].plate_type = .OCEANIC
    } else {
        plates[i].plate_type = .CONTINENTAL
    }
    plates[i].rotation_axis = rotation_axis
    plates[i].angular_velocity = rand_float32_range(0, f32(max_angular_velocity))
}
```

Then for each face, we give it a velocity based on the angular velocity and the rotation axis. The velocity is calculated using the cross product of the rotation axis and the position of the face. Thanks, [Euler](https://en.wikipedia.org/wiki/Euler%27s_rotation_theorem) for this. 

```odin
for plate in plates {
    axis := plate.rotation_axis
    omega := plate.angular_velocity
    for face_idx in plate.faces {
        face := &goldberg.faces[face_idx]
        p := face.center
        face.velocity = omega * cross(axis, p)
    }
}
```

Now we can just check if a face is at the border and calculate the amount of stress that is being applied on it. Now my approach of calculating stress is just taking the maximum difference between the velocities of the neigbours. Then we can use these stress values to create a height map. But for now we will color the faces based on the stress values, if the stress is high, we will color it red, else blue.

![image](https://u.cubeupload.com/namishhhh/Screenshot2025041115.png)

### A Basic World

Now that we have atleast three terrains, namely water, mountain, and not a mountain, we can start creating the most basic world. The first step is to generate a height map according to the stress values. Lower stress equals lower height, and higher stress equals higher height.

Generating the heightmap was somewhat a challenge. First step was to just generate the initial heightmap, which just consisted of giving faces of CONTINENTAL plates a height of 0.5 and OCEANIC plates a height of -0.5. Then on top of this initial height, we calculate the stress height according to the stress factor and add it to the initial heights.

The formula of the stress factor is

```odin
stress_factor := max_stress > 0 ? stress_values[face_idx] / max_stress : 0
stress_height := math.pow(stress_factor, 1.5) * 1.5 // add this to the initial height
```

The next step was to assign colors to the faces based on the height. Right now it is a very simple function - 

1. if the plate is oceanic, color it blue
2. else if stress factor is greater than `THRESHOLD`, color it red/orange else green


Another thing I wanted to do was to draw the rotation axis of the planet, and not just make it a random vector. So with some maths, I made a function that takes in a tilt_value (angle in degrees) and spews back an axis

```odin
x := math.sin(tilt_angle)
y := math.cos(tilt_angle)
z := 0.0

return normalize(rl.Vector3{f32(x), f32(y), f32(z)})
```

Drawing the axis is a trivial task, it is just drawing a line passing through the cneter of the screen. And now we have got this:

![image](https://u.cubeupload.com/namishhhh/Screenshot2025041201.png)

Due to me having a low end setup, I have removed one level of subdivision, so as to not waste a lot of time rendering. We are still far away from a semi-realistic planet but we are getting there.

### Perlin Noise

![perlin](https://u.cubeupload.com/namishhhh/perlinnoiseterrainme.png)

Now it is time to add some noise to the planet. And of course, like any other terrain generator video/article on the planet, we will use Perlin noise. This will help us in making the terrain look more realistic and varied. We are not using random 1 and 0 white noise because, well terrain is not just random. Perlin noise generated smooth curves and is a lot more realistic for our use case.

I will not be going deep into the math of Perlin noise, but I will just give a brief overview of how it works. The idea is to take a grid of points and assign each point a random value. Then for each point in the grid, we interpolate between the values of the surrounding points to get a smooth value. This is done using a fade function, which smooths out the values. 

The fade function is defined as:

```odin
fade :: proc(t: f32) -> f32 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
```

Now the idea is to create multiple octaves of noise, and them combine them to get a more detailed noise. This will make it even more realistic. So for now I have three layers

```odin
NOISE_LAYERS :: []NoiseLayer{
    {scale = 1.0, influence = 0.3, octaves = 4, persistence = 0.5},
    {scale = 3.0, influence = 0.15, octaves = 6, persistence = 0.5},
    {scale = 8.0, influence = 0.05, octaves = 2, persistence = 0.5},
}
```

The scale is the size of the noise, the influence is how much it will affect the height, and the octaves are how many layers of noise we will use. The persistence is how much each layer will affect the next layer. 

Now when we calculating height, we just add the noise to the height. And now we have some more realistic terrain.

<br>

But it still is not realistic enough. That is because I realized that Voronoi graphs are not the best way to randomly distribute plates. So for now, I updated this function to just take the center points and randomly expand them. To make it even more random, I gave each plate a random strength factor and random directions, in which they will expand. Now we have a more realistically distributed planet.

<br>

Adding another layer of realism, I made it so that there is noise even in the colors. The idea is to just add some noise to the color of the faces, so that they are not all the same color. 

```odin
noise_for_color :: proc(value: u8, range: u8) -> u8 {
	noise := rand_int_max(int(range * 2 + 1)) - int(range)
	result := int(value) + noise
	return u8(math.clamp(result, 0, 255))
}
```

To even add even more realism, we can go wayyyy back and add some noise to the position of the points. This will make the polyhedron look like as if it was made of irregular hexagons and pentagons.

<br>

And now after these small changes, we have a drastically improved planet. It is also fun to see that, we started with perfectly placed points and a voronoi graph, and now we are at a completely different solution.

![image](https://u.cubeupload.com/namishhhh/Screenshot2025041322.png)

### Adding height to the faces

Well now, its time add height to the faces, to make the mountains.... actually look like mountains. It can be simply done by changing the height of the vertices of the face. But we don't want to add the height to water, so we will just add the height to the continental plates. It is fairly trivial (after some help with claude)

```odin
for vertex_idx := 0; vertex_idx < len(planet.vertices); vertex_idx += 1 {
    if vertex_counts[vertex_idx] > 0 {
        avg_height := vertex_heights[vertex_idx]
        displacement: f32
        normalized_height := (avg_height - height_map.min_height) / (height_map.max_height - height_map.min_height)
        if normalized_height < WATER_THRESHOLD {
            displacement = water_displacement
        } else {
            displacement = avg_height * height_scale
        }
        vertex := &planet.vertices[vertex_idx]
        base_position := normalize(vertex.position) * planet.radius
        vertex.position = base_position + vertex.normal * displacement
    }
}
```

After moving vertices, the faces need updated centers and normals. The face’s center is recalculated as the average position of its vertices. The normal is recalculated using the cross product of two edges of the face. The updated center and normal are then assigned to the face.

![image](https://u.cubeupload.com/namishhhh/746Screenshot2025041322.png)

### Climate and Biomes

The climate and biomes of the planet are determined by two main factors: temperature and precipitation. Temperature is in turn, determined by the height, and the distance from equator. Well so this was the easier of the two, the only trouble I got was keeping the TILT value in mind, but not that hard.

```odin
abs_latitude := math.abs(latitude_dot)

temp := lerp(POLE_TEMP, EQUATOR_TEMP, equatorial_factor)

normalized_height := (height_map.values[face_idx] - height_map.min_height) / 
                    (height_map.max_height - height_map.min_height)

if normalized_height > WATER_THRESHOLD {
    altitude_factor := (normalized_height - WATER_THRESHOLD) / (1.0 - WATER_THRESHOLD)
    temp = temp - f32(altitude_factor * ALTITUDE_TEMP_FACTOR)
}
```

Precipitation on the other hand, was a lot more complex. I created a very simple model for it, which for now is

1. near equator no rain, decrease as we go up for the first 30 degrees and then increase. no rain in the poles.
2. near ocean means more rain
3. near mountains means more rain

There are a lot of other factors that affect precipitation, but for now, this is good enough. Checking for water nearby is easy, we just check if the height is under the water threshold. For checking for mountains, we just the get the highest faces and check if the height is greater than a certain threshold. The final precipitation logic is as follows:

```odin
if scaled_latitude < 0.12 {
    precip = lerp(0.8, 1.0, 1.0 - scaled_latitude/0.12)
} else if scaled_latitude < 0.3 {
    t := (scaled_latitude - 0.12) / (0.3 - 0.12)
    precip = lerp(0.8, 0.2, t)
} else if scaled_latitude < 0.6 {
    t := (scaled_latitude - 0.3) / (0.6 - 0.3)
    precip = lerp(0.2, 0.7, t)
} else {
    t := (scaled_latitude - 0.6) / (1.0 - 0.6)
    precip = lerp(0.7, 0.1, t)
}

if is_coastal[face_idx] {
    precip += COASTAL_PRECIP_BONUS
}

if near_mountain[face_idx] {
    precip += MOUNTAIN_PRECIP_BONUS
}

if normalized_height <= WATER_THRESHOLD {
    precip = 0.0
}

precip = math.clamp(precip, 0.0, 1.0)
```

Now we can just assign the biomes based on the temperature and precipitation. So I listed out biomes, and defined their preferred (temperature, precipitation) values. Then for each face, we get the temperature and precipitation values, and assign the biome based on the closest preferred value. For now there are 13 biomes

```odin
Biome :: enum {
	OCEAN,
	DESERT,
	SAVANNAH,
	TAIGA,
	RAINFOREST,
	TUNDRA,
	POLAR,
	TEMP_FOREST,
	MEDITERRANEAN,
	STEPPE,
	GRASSLAND,
	MOUNTAIN,
	SNOW_CAP,
}
```

The final result is a planet with different biomes, but there are still many visible patterns, mostly because our logic is very simple. To help with that, I added some layers of perlin noise to the temperature and precipitation values. This will help in making the biomes look more natural and less uniform.

<br>

And then I added the finishing touches, for example making the ocean blue, and the mountains gray, and the peak of the mountains white. I also made it so that near poles, the ocean is more blue (glaciers). I also added some noise to the colors of biomes like before and we are essentially done?


![final](https://u.cubeupload.com/namishhhh/Screenshot2025041401.png)

## Conclusion

And.... cut. Now I am well aware my code is not in fact the least bit optimized, and neither I am using any of the best practices. But this was a fun little project to work on, I surprisingly learnt a LOT of maths and I'm very lowkey wanting to go deeper into maths. Also a shoutout to [claude](https://claude.ai/) 3.7 sonnet for helping me with the math and the code. Also it is very interesting to see, how different this blog started and how different it ended. We ended up ditching both of our first starting steps to do something different, but better. It was also my first time creating something with Odin, and honestly I really like this language? It is very simple and almost reads like pseudo code. Definitely needs more love and attention. 

<br>

The source code is available at [namishh/planet](https://github.com/namishh/planet) and I encourage you to have fun playing with the config file. Until next time, bye!