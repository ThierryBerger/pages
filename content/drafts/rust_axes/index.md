+++
title = "The axis you were already drawing"
description = "Rust against C, C++, C#, JS and the rest — on axes that admit what Rust costs."
date = 2026-08-28T09:00:00+00:00
updated = 2026-08-28T09:00:00+00:00
draft = false
template = "drafts/page.html"

[extra]
lead = "Every language comparison draws one line. That line is hiding two more."
+++

Draft scaffold — the diagram below is the finished piece; the prose around it is
still to be written.

{{ diagram(name="lang-axes") }}

## Notes toward the article

- The single "low level ↔ high level" axis everyone draws is really `control`.
  Nothing about the ordering changes when you rename it, which is why stage 0
  and stage 1 share an x coordinate: it is the same axis, honestly relabelled.
- `safety` is the thing that axis was hiding. Splitting it out is what makes
  C and C++ stop being the same point.
- The dashed line is the folk model: control and safety trade off, pick one.
  Rust is visibly off it. So, for that matter, is Go — less dramatically.
- `cost` is where Rust does worst, and it is on the chart on purpose. A chart
  where the subject wins on every axis reads as advocacy, not measurement.
- The finding worth writing up: **cost tracks distance from the trade-off line**
  — except for C++, which pays close to Rust's price without buying Rust's
  escape from the line.
