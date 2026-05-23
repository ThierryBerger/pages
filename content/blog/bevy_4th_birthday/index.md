+++
title = "Happy Bevybirthday"
description = "Fourth year of bevy."
date = 2024-08-13T09:19:42+00:00
updated = 2024-08-13T09:19:42+00:00
draft = false
template = "blog/page.html"

[extra]
lead = "Happy Bevybirthday!"
+++

This post is an answer to [bevy's birthday post](https://bevyengine.org/news/bevys-fourth-birthday), now that I am an active part of the ecosystem, I can't miss out!

## A bit about me

I’ve been involved in mobile development for ~10 years, most of the time in the gaming industry, and since discovering Rust in 2015 and bevy 4 years ago, I’m convinced this enbody the future of game development I want to be part of.

Now, yes, bevy is not mature yet, I’m maintaining a [curated list of bevy projects in production](https://github.com/Vrixyz/bevy_awesome_prod), and while I’m very excited to see it grow, and wish them success with all my heart, the « execution level » is (mostly) not yet comparable to the existing giants of the gamedev ecosystem.

But that’s to be expected: most projects are made with limited resources, Rust is young in gamedev, and bevy is still a "baby".

## To the future!

Now why do I believe this "baby" bevy is the future ?

I can’t spare you the rust-y « reliability and performance » reasons, but I’ll try to give more under-appreciated reasons to believe in Rust and bevy:

- Rust’s « open source by default » mentality helps with sharing resources to make the whole ecosystem better, which bevy embraces with its modularity.
- Rust’s ability to be tested or spun off into smaller projects helps with consolidating parts of any projects, which is a godsend when investigating a bug, or teach a feature.

Why do I care about bugs and teaching ?

## More about me...

I’m glad you asked! I believe so much in Rust (and bevy) that I did quit my unity gamedev job earlier this year to hop on the bevy train.

Special thanks to the fellow tech enthusiasts who listened to [my](https://www.youtube.com/watch?v=pzVMvTHspPw&list=PLbvvWoCXmXkJHyozyLtWo83Y9B-6qqs0B&index=3&t=1s) few [talks](https://ftp.belnet.be/mirror/FOSDEM/video/2024/h2215/fosdem-2024-1776-journey-to-an-open-source-contribution.av1.webm) about bevy [ecosystem](https://www.linkedin.com/posts/thierry-berger-614aa79a_rust-activity-7207465947945676800-xeeR?utm_source=share&utm_medium=member_desktop), you empowered me.

And what a turn of events! I’m now a freelance open source developer, with my first 1 year mission to maintain [Rapier](https://rapier.rs/) and its surrounding ecosystem.

## To the community

So of course, I’m excited about physics engines, and seeing [avian](https://github.com/Jondolf/avian) make such a fulgurant entry is both humbling and a great sign for production projects. Congrats Jondolf !

Physics engines is not the only area where high quality libraries are growing: UI (bevy birthday post has a great list), tooling, rendering…

Now, these are mostly « tech-oriented » projects, and my wish for this next bevy year is to get more non-tech projects: Let bevy reach the end user!

## To the team

> 🌹: Bevy is genuinely delightful to have multiple programmers
> 
> 🌹: But for artists... eh not so much

It’s a bit too early for most non-tech to feel welcome into bevy, and not only artists: every question asked is at risk of being answered by « it depends »:

- How do I contribute assets ?
- How to collaborate on a bevy project ?
- Which platform can we target ?
- How can I get funded ?
- Do I need to know Rust ?

It’s a proof of solid foundation to be able to provide multiple answers to a given question, but it’s clearly overwhelming, and unfit to quick assesment of the engine.

Where you can build a game « naïvely » in Unity or Godot, without knowing exactly where you’re going, it’s not advisable to do that in bevy if you’re not a veteran (or at all).

So my wish for next bevy year, is to improve the « naïve » proofness of bevy: 

- reduce surprises: add tests, documentation, listen to and act on user experiences
- ensuring tools work and help each other: bevy mod system dump is one of my personal favorite, that I wish all projects would include. [Mockersf initiative](https://www.youtube.com/watch?v=BwZGbZQdpec) of screenshot comparison ([Pixel Eagle](https://pixel-eagle.vleue.com/)) is another great example of tooling which most projects would benefit from.
- Crates compatibility: [avian collider creation](https://github.com/Jondolf/avian/pull/326) ; nalgebra compatiblity with glam...
- Everything developer experience, which must now trickle down (up ?) to project-team experience, in order to make bevy take its flight.
  - Let's not settle only for Developer eXperience (DX), let's popularize the term Team eXperience (TX),

## What now?

I've already started this goal of improving the ecosystem, but I have to admit the most difficult part is identifying
the lowest possible hanging fruit where I'll be able to help, without crunching.

I'm proud of having led the effort for a CI test for [conflicting systems ordering ambiguities](https://github.com/bevyengine/bevy/pull/13950), and I'd love more visual tooling around bevy development. I'd love to take a closer look at the [source location in change detection](https://github.com/bevyengine/bevy/pull/14034/) info from aevyrie, and [Blenvy](https://github.com/kaosat-dev/Blenvy) from kaosat.

<br />
<br />

[I have a sponsor page ❤️](https://github.com/sponsors/Vrixyz)
