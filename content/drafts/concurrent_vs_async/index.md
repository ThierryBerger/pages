+++
title = "Concurrent is not async"
description = "Two diagrams: what a second CPU actually adds, and where the waiting goes."
date = 2026-08-28T10:00:00+00:00
updated = 2026-08-28T10:00:00+00:00
draft = false
template = "drafts/page.html"

[extra]
lead = "Both words mean 'more than one thing at a time'. They mean it very differently."
+++

Draft scaffold — the diagrams are the finished pieces; the prose is still to be
written.

## Concurrency: a second line

{{ diagram(name="concurrency") }}

The build: one timeline, then a second one — and the axis down the page is now a
CPU, not more time. Both lines get work. Zoom in and the blocks turn out to be
runs of instructions. Project them onto one shared ruler and they interleave.

Press **Run again**: same program, same code, different order. Nothing chose
that order. That is the whole problem, and it is why the pair under the bracket
needs a lock rather than an argument about who is faster.

## Async: where the waiting went

{{ diagram(name="async") }}

The build starts identically — one line, two calls — and then goes somewhere
else entirely. Each call is really a handful of units of work with waiting
between them; those units are the awaits. Lift the functions off the timeline
and they stop being schedules and become descriptions. Introduce a runtime, and
it owns the only timeline there is. The units land on it interleaved, and the
waiting overlaps away.

Then the two enums, which is where async stops being magic:

- `Poll<T>` is what the runtime asks. `Pending` means put it back in the queue;
  `Ready(T)` means advance.
- `LoadUser` is what the async fn *became*: one variant per `.await`. The
  function didn't keep a stack — it got rewritten as a state machine.

## The distinction

Concurrency is about **order** — several lines, no guarantee between them.
Async is about **waiting** — one line, and the gaps filled in. You can have
either without the other, which is why a single-threaded runtime is genuinely
async and genuinely not parallel.
