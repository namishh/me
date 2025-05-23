---
title: Year in Retrospective 
description: crazy year huh. 
date: 15 Oct 2024
draft: false
author: Namish 
category: talk
---

![accelerate](/static/images/accelerate.jpg)
<div class="mb-2" align="center">
creds to [0xluffyb](https://x.com/0xluffyb) for the wall
</div>

This year definitely was my best year when it comes to programming, only. Yea I built a bunch of cool shit but overall as a student not much of a great year (although I am working on improving that). This post though, only details the stuff I built or did this year starting from January to October. And I am pretty sure I am not starting anything new till like the June of 2025. 

### Lovbyte

![lb](https://i.imgur.com/v8FxPyT.png)

[Lovbyte](https://github.com/namishh/lovbyte) was my first serious step in making full stack web applications with javascript. It uses `remix` as the framework and `prisma` as the orm. Honestly, I really did not dislike remix as a framework, it was _fine_ to work with. I have always had issues with typescript apps because then I have to run `tsserver` and it painfully freezes my `4gb ram beast of a laptop`. Lovbyte was a `dating app` kind of thing but specifically made for programmer where programmers can customise their pages with projects they made or their tech stacks and so. I also added a chat feature as well. After this I got tired of web dev for some time and moved on to rust. 

### Linear

My best and recommended method of learning any new language is to some kind of discord bot with it. And [so i did](https://github.com/namishh/linear). Linear is a discord bot which can be used to host cryptic hunts and quizzes in your server. Sadly, now I can trigger people, but instead of sqlite, I used mongodb for storing data (**I'm sorry**). I also integrated the power to take and create hints. Overall I got a basic understanding of rust. I do not prefer rust for programming because it _is_ kinda too hard for my dumb brain and I'll respectfully take the skill issue here.

### Scuffword 

This is yet another discord bot made for learning yet another new language, this time C. [This bot](https://github.com/namishh/scuffword) is an implementation of the infamous password game by [neal](https://neal.fun). Of course this is not an 100% implementation as all levels are not realistically possible inside discord's ui, but I tried my best. There are about 20 questions or so and it is pretty fun to play. I also learned how to make fetch requests in C and then manually string parsing json (pain). Also this project made me fell in love with sqlite, and it is now the only database I opt for everytime I'm building something no matter how complex the project is.

### Webby // Shawty

This was when I had the most fun with C. These are web servers written in scratch in C without any external modules. I HAD so much fun making them and I'm honestly grateful that I did these. It made me learn more about how requests and responses work. With [webby](https://github.com/namishh/webby), I only had some basic routing, static files like images and gifs, and a fully functional sqlite todo app. [Shawty](https://github.com/namishh/shawty) is a url shortener written in the same stack, but replacing my javascript files with htmx. Kinda fascinates me how easy things really are when you fully understand them.

### Pound

![pound](https://i.imgur.com/q5I6zJt.png)

[Pound](https://github.com/namishh/pound) is a text editor I made following [snaptoken's amazing tutorial](https://viewsourcecode.org/snaptoken/kilo/), but I extended it myself with cloning some common `vim emulation`, fancier ui and dashboard, and working line numbers. These things sound easy to make, but to me back then, these were some insanely tough things. This honestly taught me a LOT about terminals, I did not even know something called raw mode also existed before this tutorial.

### Neuing 

[Neuing](https://github.com/namishh/neuing) was a simple neural network simulation written from the ground up in golang, so that means I had no numpy or anything. Now this was a painstaking task because I am a very weak maths student and at the grassroots level, neural networks ARE just some matrix multiplications and it was a pain getting them right, but in the end I was satisfied with _some_ convincing accuracy so it works I guess.

### Biotrack 

![bt](https://i.imgur.com/NNaq5Lp.png)

I wanted to get back to web dev but I did not want to go back to writing mindless `jsx`. So I decided to use `echo` and `templ` to create [biotrack](https://git.new/biotrack), a all in one health tracker. I also experimented with the gemini api (actually quiet good) and also added an chat assistant that can find anomalies in your data and make recommendations. The good thing is that all of it is done entirely within golang and templ itself so no react bs or something. This was a very fresh breath of air from all the web dev slop that had been feed into my mind for the past 4 years.

### Holmes 

![holmes](https://i.imgur.com/m87qXdv.png)

[Holmes](https://github.com/namishh/holmes) was another golang/echo web app I made but it is a bit more complex than bio track. It is a website where you can host cryptic hunts / quizzes and also add media like images, videos, audio to your questions via an admin panel. It was a fun challenge to create an admin panel and I also learnt about csrf protection. This was an app that was actually used to host a hunt and it went pretty smoothly with 0 downtime and errors. I also learned how to use s3 buckets for storing stuff. Overall it was pretty fun to make and I honestly love this type of web development more where I am not bounded by some insane file structure which results in 100s of files for the most mundane things.

### Pixie

![pixie](https://i.imgur.com/rNaopDH.png)

[Pixie](https://github.com/namishh/pixie) is a web based image editor built with NextJs for the frontend and Rust for all the backend image processing in the form of web assembly. I have not used any libraries for image processing and all of it is painful pixel manipulation that was fun to learn. Similary the onlything actually used in the frontend is the html canvas. You can crop, resize, scale and rotate images as some basic transformations or tweak with a bunch of filters. Of course you can load and save images. This was very fun to make as well and it was also a bit of the pain in the frontend but using `zustand` for state management made things a lot more easy.

### Lock In Cafe

![lockin](https://i.imgur.com/qv3Lkkl.png)

[Lock In Cafe](https://github.com/namishh/lockin) is another one of those 24/7 lofi websites. When I learnt that [lofi.cafe](https://lofi.cafe) is just embedding youtube iframes and cycling through them, I could not resist building my own. I added a todo list and a pomo timer on top of that and I have actually been using this non stop. Rare, one of the things I built that I actually use. This website is made with nextjs because I didn't really want to spend much time making this and this was done under 2 hours of development time spanned across 4 days. You can visit lock in cafe at [https://cafe.namishh.me](https://cafe.namishh.me).

### Zenote 

![zenote](https://i.imgur.com/ryA9Aq6.png)

I also fiddled around with Tauri to create [zenote](https://github.com/namishh/zenote), a no bullshit, keyboard oriented, markdown note taking app. It is really simple, has a buffer explorer, ability to save and open files, dark and light mode, and a nice looking welcome screen as well. So there is nothing extra in this app and it was made with the mindset of just "open and type".

### Notes 

![notes](https://i.imgur.com/SnFRveH.png)

I also got interested in a bit of DSA this year and decided to upload all of my notes on the web. Right now, there are only a handful of topics but be prepared to see it get large in the next couple of years. You can pay thema visit at [https://notes.namishh.me](https://notes.namishh.me).

### Smaller Things 

1. [Captivus](https://github.com/namishh/captivus): A Prisoner's Dillema Simulator 
2. [Motus](https://github.com/namishh/motus): A very tiny basic physics engine in C 
3. [Venusta](https://github.com/namishh/venusta): A tiny utility to extract colorscheme from images
4. [Fetch.rs](https://github.com/namishh/fetch.rs): A small customisable linux fetch script
5. [Hacknio](https://hacknio.vercel.app): Re-designed the whole ui for this hackernews client

### Floww 

[Floww](https://git.new/floww) is one of my bigger projects that I have put on hold/rarely work on because it is too big and will take me a couple of years to release. I plan to make it as a small SaSS startup and I really do believe it can just work out if executed perfectly which will require time and that is why it is on hold for now.


## Studies and rest

Well this is my last year as a school student and my prolly my last blog post of this year. I have my college entrance exams in Jan and Apr but I very much know I'm not anywhere near the race. I will be fighting kids who worship this exam and have been religiously preparing for it for 4 years and sometimes even 6-7 years. Many people will be attempting the paper for the second time who might have gotten a very good rank last year but just not good enough for their desired branch, and then there is me, a mostly dumb kid whose not even that serious about the exam with 1 year of preparation. And to get my desired branch (computer science), I need be in the top 0.5% of the kids, and I'm battling 2M of them. Pretty hopeless and this journey could have gone right at multiple steps but yet here I am. I might make "Learn to excel in JEE from a failure" next year because there are some things that I regret but programming is NOT one of them.

In my country, people still have this broad thinking that "your college defines you". And it is true that jobs are more likely to be given to students in better colleges, but that is not because of the college. It is because the student is smart and the chance of a kid being smart in a top tier college is high as well.

Well JEE, might not be destined for me, I could still give it a fair shot, but I know that all I want and WILL do is computer science no matter which college I land in. Maybe the result of this hard work will come late, but it will be sweet.
