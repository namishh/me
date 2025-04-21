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

The next thing I did was to add these line like structures to the paper. Now I tried looking into this a bit, and found there is a whole field for this called anisotropy and there are literal research papers on the anisotropy of a paper. I, well will not go into that. But with the help of my friend, claude 3.7 I was able to find a simpler alternative. The idea is simple, create very long horizontal streaks, create vertical streaks and the mix them together. This is how the function for it looked like.

```glsl title="fragment.glsl"
float fibers(vec2 uv) {
    float hPattern = noise(vec2(uv.x * 100.0, uv.y * 10.0)) * 0.5 + 0.5;
    float vPattern = noise(vec2(uv.x * 10.0, uv.y * 100.0)) * 0.5 + 0.5;
    
    return mix(hPattern, vPattern, 0.3);
}
```

Then I added some age spots to the paper, which are just random skewed figures with a slightly darker color. And then as a final touch fo r my old paper look, I darkened the edges and the corners of the paper. Now, we have a pretty decent and "semi-realistic" old paper texture.

![save](https://u.cubeupload.com/namishhhh/Screenshot2025042202.png)