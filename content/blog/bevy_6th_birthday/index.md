+++
title = "Happy 6th Bevybirthday"
description = "Sixth year of bevy."
date = 2026-09-01T09:19:42+00:00
updated = 2026-09-01T09:19:42+00:00
draft = false
template = "blog/page.html"

[extra]
lead = "Happy Bevybirthday!"
+++

This post is an answer to [Bevy's 6th birthday post](https://bevy.org/news/bevys-sixth-birthday/), I missed out last year, so let this be a 2 year recap!

## A bit about me and Bevy

I'm an established freelance now! Mostly Rust, some teaching, tech events, and helping local businesses when I can be useful.

Recently I've teamed up with [Mainmatter](https://mainmatter.com/) for a C to Rust migration, exciting project!

I'm active in the Bevy community, I:
- Organized a "Bevy Birds of a Feather" at FOSDEM [2025](https://archive.fosdem.org/2025/) & [2026](https://archive.fosdem.org/2026/) (& join us in [2027](https://fosdem.org/2027/)?)
- Helped maintain the [Dimforge](https://www.dimforge.com/), Rapier ecosystem.
- Contributed to [Jackdaw](https://github.com/jbuehler23/jackdaw/) at [Rustweek](https://2026.rustweek.org/): to fix error handling and surfacing it to users, and how to tackle translation with [Fluent](https://projectfluent.org/)
- Added a dozen entries to my [curated list of awesome Bevy projects in production](https://github.com/ThierryBerger/bevy_awesome_prod)
- Upgraded a Bevy hack around rustdoc to an [official feature](@/blog/rustdoc_contribution/index.md)
- Brought my Rustlang/Bevy card game in events: [Techycards](https://github.com/ThierryBerger/techycards)
- And looking forward to more!

## Bevy: Current status

### Good

Rust success stories are plenty now, with the TL;DR being:
- fewer bugs increase productivity
- good performance
- Rewrites in Rust more and more ambitious

Those success stories often come from "system" (operating systems, embedded, scripts...) or backend (servers, databases, low level libraries...).

And Bevy can do all that! Because it's modular, it can be headless, and toying around data first is a great use of the engine, so it translates well into "backend" projects: a server to dispatch shared logic, a robotic simulation, a data store in memory...

### Difficult

Bevy's powerful rendering capabilities feel like a bonus currently, because rendering heavy projects are often interesting when:
- there's a lot of work put into pre-processing (light baking, homemade maps...), and currently Bevy needs quite some experience to pull these off, it's possible already, but it takes... **time**.
- leveraging procedural generation, which also requires quite serious skills.

Compare with a Unity/Godot project where the editor *can* guide the developer **and the team** by the hand, choosing Bevy puts pressure on the developer to build a whole bunch of tooling if there's any kind of team involved.

### Lacking

Bevy and Rust alike have fewer success stories in GUI projects.

Of course in Rust we have our champions: check out <https://blog.wybxc.cc/blog/rust-gui-survey-2026/> and <https://areweguiyet.com/> to get a feeling of the ecosystem!

But these projects often lack maturity, maintenance power, funding stability, or suffer from cross-language communication.

In the Bevy ecosystem, attractive options are:

- Bevy UI: Made in Bevy for Bevy by Bevy, honestly a strong option with the recent [Bevy Scene Notation (BSN)](https://github.com/bevyengine/bevy/pull/23413), but still very early, very "custom" feel, where developers need an active engagement and motivation to get into: Coming from web development, the flexbox model powered by [Taffy](https://github.com/DioxusLabs/taffy) helps, but devtools are not at the same place as web's.
- [egui](https://github.com/emilk/egui): immediate mode UI you can use with Bevy
  - often seen as a tech-oriented UI
- And a lot of enthusiasts making their own interesting projects, but risky to depend on, check out <https://bevy.org/assets/#ui>!

## To the future

### Team eXperience

My last post mentioned improving the "Team eXperience", the experiment on the editor has started (multiple times), and BSN was merged, which I believe will be a great foundation for quicker iteration on those topics: getting data-oriented, shorter (less noisy) syntax is an important step for communication.

### Audience eXperience

In the intro, I mentioned FOSDEM and Rustweek, I'd love to keep on gathering enthusiasts and builders to help Bevy reach the interested parties: I believe some projects would benefit choosing Bevy, and I believe some companies would benefit [donating to Bevy](https://bevy.org/donate/).

For that to be possible, developer advocacy should be thought out a bit more than "we're building a great product".

The battle for attention is becoming even greater with the whole LLM topic (and that's all I'll say).

*And If you have or spot a project using Bevy in production, file a PR at [bevy_awesome_prod](https://github.com/ThierryBerger/bevy_awesome_prod)!*

## What now?

Tell me!

Get in touch, really. I want to know what you're building, let's bring people together, make Rust and Bevy reach their audience, let's make our software approachable and reliable.

<br />
<br />

- [Linkedin](https://www.linkedin.com/in/thierry-berger-614aa79a/)
- [Mastodon](https://mastodon.gamedev.place/@Vrixyz)
- [BlueSky](https://bsky.app/profile/thierryberger.bsky.social)
