---
title: "Cat Tools"
post-date: 2026-04-07
update-date:
slug: cat-tools
tags: [dev, indie-web, rustlang]
summary: A template for a post
cover: ./cover.jpg
draft: true
featured: false
---

# Cat Tools

So my previous post mentioned making stuff and writing stuff. That post was also a test post so that I'd have something to aim at when I had built a tool I'd been working on up to a testable point. That's right, I'm building my very own Cat Tools. 

![A poorly edited joke version of the 'Cow tools' comic from The Far Side by Gary Larson](.\cat-tools.png)

## The Tools in Question

### md_convert

I have harnessed the power of evil science and knowing a small amount of Rust to allow us to automagically convert markdown files to html pages that you can read on my website. The first full-ish version will even link up the css file so it looks nice.

Don't look at [my code](https://github.com/samulated/nukekitty/tree/main/tools/md_convert) it absolutely needs to be refactored (also, thinking on it, I forgot to add hyperlinks to my inline formatting pass... Oops) but it works enough for the initial version. I'm pretty happy. I built it myself!

Now to wrap my head around automating it, adding in some diff checking (I'll honestly be lazy and just see if the md files changed recently) and maybe even one day I'll refactor it. I think that doing what I have done in Rust is considered a sin, but I think I could definitely pull off whichever scarlet `char` they'd want me to wear for it. (oops, I also didn't factor in inline code...)

So many things to do when you write your own tools, how does one keep track of it all?

### Streetmaps

In the process of writing this I have come up with a very silly concept. Sub-roadmap roadmaps, aka: streetmaps.

Since the website has a few different things going on, and I have a few websites and other related things going on (multiple roads, you see, perhaps they need to be collected into a boulevardmap) and also some things with some crossover (obviously that would be an alleymap or lanemap) it would be highly useful to be able to think about the direction that these are going in.

You can also be damn sure that if I ever become deranged enough to get into open source mapping and gis stuff (I've already come pretty close before), I'd definitely call it MapStreets so I could have the MapStreets Streetmap Roadmap.

## Further tools?

Now that I'm making Cat Tools, it's very difficult to stop. If I need to do something, I can in fact make something that does that. Pairing that with my brand new coding diva (picture below) I am hopeful that I'll get a bunch done.

![My new* Lenovo Thinkpad x131 laptop. *probably at least third-hand by now](./thinkpad.jpg)

Hopefully other people find them helpful but even if they don't, they were made for me.

~ Verx