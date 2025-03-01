---
title: How To Not Suck At Making Quizzes
description: All the things that went wrong while making a shitty website for a serious quiz
draft: false
date: 09 Oct 2023
author: Namish 
category: talks
---
<img src="/quizapp.png"/>
[play it here, username and pass below](https://cyquest.vercel.app)


## How Did I Get Here?

Well, I was pretty busy over the last month building a quiz app for my school's tech fest. Because I was a fool, I spent most of my time making the ui look good instead of implementing the actual quiz system. I rushed this thing in 3 days and did not get enough time for testing the website, so the website was mostly in development while the actual competition was going on.  But here are a set of mistakes that I made which made me encounter sleepless nights


### 1. Use An Actual Backend

This code had no actual backend (probably because I did not know anything about backend at that time). Yea right, no `backend`. I literally stored questions in a javascript file and then called them. You can see how this will cause problems. Yup, the answers would literally be visible in the `main.js` javascript file. I would have actually not noticed this until [my friend, ni5arga](https://github.com/ni5arga) pointed it out. This was 30 minutes before the competition was going to start. In a last ditch attempt solve this problem, I made my next mistake.

### 2. Hash Your Answers

Instead of hashing my answers, I genuinely thought it was a good idea to put some 2-3 ciphers on each answers. It sounds like an improvement to the earlier situation. That is until you realize that the code for encrypting the answers is just plain visible in the `main.js` file. And one guy, who was just there to mess around with us, actually leaked the ciphers that were used. But luckily, no one actually took out the time to decipher each answer that was leaked.

**Easy Fix**

An easy fix for this is to use `bcrypt`. It has a npm repository available [here](https://www.npmjs.com/package/bcryptjs).

This is an easy example of how it works


```js
const bcrypt = require('bcrypt');

const storedSalt = "$2a$10$iwkIBgVZDdADUSLFewMBJu"; // random value, can be exposed
const storedHash = '$2a$10$iwkIBgVZDdADUSLFewMBJu86oEYjRnnxUR9Nli2pfeQyRaIzr5kMS'; // hashing the password with storedSalt,
// this can be exposed in your js and it's fine

const userPassword = getPassword();
const generatedHash = bcrypt.hashSync(userPassword, storedSalt);

if (generatedHash === storedHash) {
  console.log('Password is correct.'); // will return this since the hashes match, since same salt
} else {
  console.log('Password is incorrect.');
}
```

### 3. Using a better database.

The only reason I picked up `firebase` for this project was because I only had a little experience with it during developing [marshmallow](https://github.com/chadcat7/marshmallow-rewrite). But it is the worst decision that I could have made regarding the database. 

1. Well, whats the best way to put it? I underestimated the amount of requests that would actually be made during the quest. `68k` read request were made in 2 days. If I would fetch questions from the database, then the amount of reads would only skyrocket. I actually had to upgrade my firebase plan for this hunt to continue (its free for 90 days btw)
2. Due to some exploit in firebase, some dudes were able to access a list of all the participants in the competition. While this was mostly harmless, this is still something that someone would prefer not to have on their quiz site

A better alternative would be a local `mongodb server` if you are using `express` as background. Or, if you are making your app with `flask`, just use `sql`

### 4. Better Testing

It is very important to have a couple of days to actually test your site. I got 12 minutes to test this and that is why there were a bunch of exploits and data leaks all over the place. If you are making a quiz that will be played by many people, it is very important that there are NO bugs which can potentially ruin a player's progress. Due to a very very very minor bug, I had three reports of players losing all their progress. You need to have an actual pentester and playtesters on your side to help you with this. A very much thanks to [ni5arga](https://github.com/ni5arga) and `rex` for helping me fix the vulnerabilities in the website 

### 5. Prioritise backend over frontend

Very self explanatory. First focus on making the site secure for the users and making sure that there are no exploits. The Frontend can be done later.

Fun Fact: 95% of this website actually went useless


**NOTE**
To play this game, here are the details


```txt
ind_namish@cyquest.com
namish
```
