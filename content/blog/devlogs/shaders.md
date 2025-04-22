---
title: Trying out Shaders
date: 19 April 2025
draft: true
---

## Introduction

> not a tutorial, just a devlog

This is going to be a different kind of post, because I will be trying out a different type of learning strategy. For context, I have almost 0 knowledge of shaders, and the ones I have used before are copy pasted from the internet. I will be using the top down approach, where I will decide what I want to do, and then search for the solution. The reason for it, is because shaders is simply too vast of a domain for me. For the stack, I will be using webgl for it.

### The Most Basic Shader

The most basic shader will be to just display a solid color. Well we actually need two different kinds of shaders, the vertex shader and the fragment shader.

```glsl title="vertex.glsl"
attribute vec2 a_position;
varying vec2 v_position;
void main() {
    gl_Position = vec4(a_position, 0.0, 1.0);
    v_position = a_position;
}
```

**Vertex Shader** handles the positions of points that define the shape you want to draw. In our case, we are just drawing the full viewport. It runs once for each vertex, and the output is passed to the fragment shader. Its main job is to figure out where that point should appear on the screen by transforming its position into a special coordinate system called clip space, which the GPU uses to decide what’s visible


```glsl title="fragment.glsl"
precision mediump float;

void main() {
    vec3 paperColor = vec3(0.94, 0.88, 0.71);
    gl_FragColor = vec4(paperColor, 1.0);
}
```

**Fragment Shader** is responsible for the color of each pixel. It runs once for each pixel, and the output is the color of that pixel. The fragment shader takes the output from the vertex shader and uses it to determine the color of each pixel in the shape. In our case, we are just setting the color to a solid color.

And this is what we have for now.

![basic](https://u.cubeupload.com/namishhhh/Screenshot2025042200.png)

## Fantasy Map Shader

Next, now we try to convert this simple solid color into something like a fantasy map. The idea is to create this old map like texture and add things like lakes, mountains, forests, villages, islands and meadows. I will also try to add text to the map, as to indicate the names of the places. The first thing to do is -

### Old Paper Texture

The first thing to get the old paper-y texture is to create random darkened spots on the paper, "stains" being a better word. So first of all, in the previous sentence alone, I used the word "random", and for that I'm using a very simple hash function to return pseudo random numbers.

```glsl title="fragment.glsl"
float random(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898, 78.233))) * 43758.5453123);
}
```

This combination of dot, sin and fract is a very cheap way to generate pseudo random numbers. If you are confused, the fract function just returns the fractional part, so the output only ranges from 0 to 1.

<br>

Now we can this random function to create a function for noise, more specifically, Perlin noise. All we do is, for a cell of size 1, we take the random value of the four corners of the cell, and then interpolate between them. The interpolation is done using a smoothstep function, which is a built in function in GLSL. And in the end, we use bilenear interpolation using the inbuilt mix function to get the final value.

```glsl title="fragment.glsl"
float noise(vec2 st) {
    vec2 i = floor(st);
    vec2 f = fract(st);

    float a = random(i);
    float b = random(i + vec2(1.0, 0.0));
    float c = random(i + vec2(0.0, 1.0));
    float d = random(i + vec2(1.0, 1.0));
    vec2 u = smoothstep(0.0, 1.0, f);
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}
```

Now to make a stain mark, we will just make a circle, and then use the noise function to warp it. It is a pretty simple function.

```glsl title="fragment.glsl"
float stain(vec2 uv, vec2 center, float size, float irregularity) {
    float dist = length(uv - center);
    float noise_val = noise(uv * 3.0) * irregularity;
    return smoothstep(size + noise_val, size - 0.1 + noise_val, dist);
}
```

`length(uv - center)` computes the Euclidean distance from the current fragment’s UV coordinate to the blotch’s center, yielding a perfect circle when plotted. To break that perfection and simulate natural irregular edges, the function samples `noise(uv * 3.0)` multiplying uv by 3.0 zooms the noise, so the irregularities occur at a finer scale.

<br>

Then I created a function to randomly generate a radius, center and irregularity and then call the stain function. In the main loop, I just call a for loop `30` times to create multiple random stains. To apply the stains, I just store the sum of all the stains in a variable, and then use the mix function to mix the color of the paper with the color of the stain.

```glsl title="fragment.glsl"
float totalStain = 0.0;

for (int i = 1; i <= 30; i++) {
    float seed = float(i) * random(vec2(0, 100));
    totalStain += randomStain(uv, seed);
}

vec3 stainColor = vec3(0.34, 0.23, 0.05);
vec3 finalColor = mix(paperColor, stainColor, totalStain);
```

Now we finally have a paper that looks old. But we can make it look older. 

![image](https://u.cubeupload.com/namishhhh/Screenshot2025042100.png)

First thing you can do is to add a grainy texture to it. This is simply just giving a random value to each pixel and multiplying it with an intensity factor. Consider it like a tv static. And then like before, just add it to the final color.

<br>

The next thing I did was to add these line like structures to the paper which happen due to aging. Now I tried looking into this a bit, and found there is a whole field for this called anisotropy and there are literal research papers on the anisotropy of a paper. I, well will not go into that. But with the help of my friend, claude 3.7 I was able to find a simpler alternative. The idea is simple, create very long horizontal streaks, create vertical streaks and the mix them together. This is how the function for it looked like.

```glsl title="fragment.glsl"
float fibers(vec2 uv) {
    float hPattern = noise(vec2(uv.x * 100.0, uv.y * 10.0)) * 0.5 + 0.5;
    float vPattern = noise(vec2(uv.x * 10.0, uv.y * 100.0)) * 0.5 + 0.5;
    
    return mix(hPattern, vPattern, 0.3);
}
```

Then I added some age spots to the paper, which are just random skewed figures with a slightly darker color. And then as a final touch for my old paper look, I darkened the edges and the corners of the paper. Now, we have a pretty decent and "semi-realistic" old paper texture.

![save](https://u.cubeupload.com/namishhhh/Screenshot2025042202.png)

### Basic Terrain

For terrain, we will layer multiple fractorial brownian motion noise functions at decreasing frequencies. The idea is to create a base layer of noise, and then add more layers on top of it. The base layer will be the largest, and the top layer will be the smallest. The result is a more complex noise pattern that looks more like terrain.

<br>

In fractal noise, we use the same noise function, but we scale the input coordinates by a factor of 2.0 for each layer, and then we multiply the output by a factor of 0.5 for each layer. The result is a more complex noise pattern that looks more like terrain.
You can read more about fractal noise [in the book of shaders](https://thebookofshaders.com/13/).

```glsl title="fragment.glsl"
float fbm(vec2 st) {
    float value = 0.0;
    float amplitude = 0.5;
    float frequency = 1.0;
    for (int i = 0; i < 7; i++) {
        value += amplitude * noise(st * frequency);
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    return value;
}
```

Now we can generate the terrain value by 

```glsl
float terrainValue = noise1 * 0.5 + noise2 * 0.3 + noise3 * 0.2 + noise4 * 0.1;
```

where `noise1`, `noise2`, `noise3` and `noise4` are the four layers of noise. The result is a value between 0 and 1, which we can use to determine the color of the terrain. For now, the threshold is set to 0.5, so anything above that will be considered land, and anything below that will be considered water. I also made the center of the map is emphasized by blending in a radial focus, making the center look more "developed" or "landmass-heavy." This is common in terrain generation to focus visual interest.

<br>

The last thing I did was to add a outline to the coastline. This is done by using the `smoothstep` function, which takes in 3 paramters and creates a smooth transition between two values. Then we take the modulus of the coastline value and sharpen it, we just apply an exponent.

```glsl title="fragment.glsl"
float coastline = smoothstep(0.5 - 0.02, 0.5 + 0.02, terrainValue);
coastline = abs(coastline - 0.5) * 2.0;
coastline = pow(coastline, 0.3);
```

Now we have a very basic map!

![basicmap](https://u.cubeupload.com/namishhhh/Screenshot2025042211.png)


### Adding details

First, we can add some lattitude and longitude lines to the map. To add a repeating pattern, we use the absolute value of `sin` functions. And then we can distort the lines just a bit to make the irregular, to give it a hand drawn look. I also made the grid lines fade out at the edges of the map, which is done by the `smoothstep` function. 

```glsl title="fragment.glsl"
float gridLines(vec2 uv, float lineWidth, float irregularity, float fadeEdges) {
    vec2 distortUV = uv;
    
    distortUV.x += noise(uv * 5.0) * irregularity;
    distortUV.y += noise((uv + vec(42.0, 17.0)) * 5.0) * irregularity;
    
    const float GRID_DENSITY = 30.0;
    
    float xGrid = abs(sin(distortUV.x * GRID_DENSITY * 3.14159));
    float yGrid = abs(sin(distortUV.y * GRID_DENSITY * 3.14159));
    
    xGrid = smoothstep(1.0 - lineWidth, 1.0, xGrid);
    yGrid = smoothstep(1.0 - lineWidth, 1.0, yGrid);
    
    float edgeFade = smoothstep(0.0, fadeEdges, uv.x) * 
                     smoothstep(0.0, fadeEdges, uv.y) * 
                     smoothstep(0.0, fadeEdges, 1.0 - uv.x) * 
                     smoothstep(0.0, fadeEdges, 1.0 - uv.y);
    
    float grid = max(xGrid, yGrid);
    
    return grid * edgeFade;
}
```

Another thing I did is to randomly place green patches on the map, to represent forests, and brown patches to represent hills and mountains. There is no actual logic to it, it is all random. And we have a pretty decent fantasy map now.

![image](https://u.cubeupload.com/namishhhh/Screenshot2025042214.png)


## Synthwave Shader

Now that I have some experience with 2D shaders, it is time to one up the number of dimensions. I want to make one of those animated synthwave mountains + road + sun style shaders. To make them animated, we can use pass in the time variable from javascript. The time variable tells the amount of time elapsed since the start. For the most basic animated shader, we can just use the time variable to smoothly lighten and darken the color. Here is the basic code for that:

```glsl title="fragment.glsl"
uniform float u_time; 

void main() {
    vec3 paperColor = vec3(0.94, 0.88, 0.71);
    
    float variation = 0.2 * sin(u_time);
    gl_FragColor = vec4(paperColor + variation, 1.0);
}
```


### Skybox

The first step is to create the skybox. Here the skybox is just a gradient that goes from purple to blue. Vertical gradients can be done by using the mix function and passing in `uv.y` in the third parameter.

<br>

The next thing I did was to randomly place stars in the upper half of the sky. We divide the upper half into cells, and then convert less than 3% into stars. The stars are just white dots with a random size and position. The stars are created by using the `smoothstep` function to create a circle. To make them twinkle, we can use the time variable to oscillate the brightness of the stars.

```glsl title="fragment.glsl"
float twinkle(vec2 gridCoord, float time) {
    float speed = 1.0 + random(gridCoord + 3.0) * 2.0;
    float phaseOffset = random(gridCoord + 4.0) * 6.28;
    return 0.5 + 0.5 * sin(time * speed + phaseOffset);
}

```

To not make it look uncanny, each star has it's own time period. 

![image](https://u.cubeupload.com/namishhhh/Screenshot2025042301.png)

### CRT Effect

To add more of the "retro" effect, we can add a CRT effect to the shader.

```glsl
scanlineY = fract(uv.y * scanlineCount + time * speed)
```

If it is not obvious scanlineCount is the number of lines you want (100 in tihs case) and speed is how fast you want the lines to move (10 in this case). Here, uv.y (0 to 1) is scaled by 100, so it ranges from 0 to 100 across the screen. Adding time * speed shifts this upward over time, and fract keeps it between 0 and 1, creating a repeating pattern of lines that scroll.

```glsl
float scanlineIntensity = 0.14;
return 1.0 - scanlineIntensity * smoothstep(0.4, 0.6, scanlineY);
```

And then we can use the smoothstep function to smooth out this pattern. This defines a band where the scanline is strongest (around 0.5), fading at the edges.

<br>

CRT monitors also have a dark vingette effect, which is just a radial fade, enhancing focus on the center. I added that as well. It takes uv (0 to 1) and remaps it to -1 to 1 with uv * 2.0 - 1.0. At the center (0.5, 0.5), this becomes (0, 0), at the corners it’s (-1, -1) or similar. This centers the effect. The expression uv.x * uv.x + uv.y * uv.y computes the squared distance from the center, ranging from 0 at the center to 2 at the corners. Multiplying by `vignetteStrength = 0.35` scales this, and subtracting from 1.0 inverts it, creating the desired effect.

```glsl title="fragment.glsl"
float vignette(vec2 uv) {
    uv = uv * 2.0 - 1.0;
    float vignetteStrength = 0.35;
    return 1.0 - (uv.x * uv.x + uv.y * uv.y) * vignetteStrength;
}
```

That is step 2 complete, we not have a shader that looks like this:

![image](https://u.cubeupload.com/namishhhh/da0Screenshot2025042301.png)