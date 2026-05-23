+++
title = "Rustdoc: Fuck it I'm helping."
description = "Who else?"
date = 2026-05-23T19:33:19+00:00
draft = false
template = "blog/page.html"

[extra]
lead = "Afraid of contributing to a programming language? Don't be!"
+++

Because we usually think about writing something once it's done, articles depicting the actual experience from a novel contributor are rather rare, this document is written as a journal, as I get through my contribution.

# Finding something

I'm helping, but what?

Well well well... look at who wrote something actionable !? Let's look at [making important traits more discoverable](../rustdoc_notable_trait/).

To be clear, that article merely put together the hard work of a handful of motivated individuals! It's not rocket science, just patient listening, or... reading.

If you have something that bothers you, do your research, and gather important resources: details get lost in cross references so it's helpful to have a smaller document listing a few points of entry to the topic, and restate the need... and keep the scope manageable.

# Discovering

Honestly, <https://rustc-dev-guide.rust-lang.org/> is well written!

With links at the top of each pages to orient you to laser-focused user objectives, you quickly get to the point where your developer environment is ready to go.

I've been told that build times would be long, brace yourselves...

*When we're more familiar with the beast, we'll help make it faster!*

# Get the shit done

Got some disk space to spare?

Count 20 Go to build on rustc, honestly, not too bad, do a `cargo clean` on any other rust project and you'll be fine 🧌.

`git clone blabla` -> `./x build` -> *go* **brr** -> it works!

Wow! So easy that now you have no excuses, embarrassing!

`./x test`

```txt
failures:
    [ui] tests/ui/allocator/regression-abort-on-free-issue-150898.rs
```

Turns out this one's on me! brb upgrading my machine... Nice touch to leave breadcrumbs in the code 👍.

> // The bug was resolved by macOS 26.4.

[source](https://github.com/rust-lang/rust/blob/54333ff079780f803f65dcee30c544050b35f544/tests/ui/allocator/regression-abort-on-free-issue-150898.rs#L12)



## Actually get the shit done.

A small fairy (i.e. an enthusiastic Guillaume Gomez) pointed me towards `rustc_attr_parsing`:

🧚
> That's where the magic happens... 💨

<details>
<summary><b>Don't click me</b> If you have any PTSD from software development time estimation.</summary>

😱

Let's take a note on what day we are now: **23 May 2026**.

If you are an experienced software engineer like me, you probably have a well defined process to estimate accurately how much time such a project will take.

I suggest we both do the calculation and compare at the end, you'll tell me if "The result will surprise YOU!©️"

Let's get into software duration estimation (or voodoo, whatever)

### Split in rough tasks

- add an attribute to be parsed (the `rustc_attr_parsing` 🧚)
- make examples
- verify it's parsed correctly
- fix bugs
- is there tests at that point? or later?
- read that attribute in render
    - find out where exactly 😅
- update the rendering
- discover more test suite using some obscure *(for now! the fairy told it was approachable 🤞)* custom accessibility API I'll have to learn
- yak shave naming and realize you needed a RFC before doing this work
  - restart with the RFC, bikeshed it and wait some more.
- Resolve code conflicts

That + some *surprise buffer*™️... **6 months.** Obviously.

Did you get the same number? Let's see if that was correct !?

> If you're not an "experienced software engineer", this section anchored in reality borders a joke about estimations being requested even when the context doesn't allow us to come up with a committable answer:
>
>  Here I'm joining just now and have no idea how the project works, nor whether I'll have any availability to work on it whatsoever.

</details>

### Get to it already...

## #[...]

When looking in the code, I stumble on `hir::AttributeKind`, but that's not what we want, that one is for top level [attributes](https://doc.rust-lang.org/reference/attributes.html).

Our change is specific to rustdoc, so we want to find [**doc** attributes](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html#at-the-item-level).

## #[doc(...)]

By searching for something we now is a **doc** attribute (but specific: e.g. `html_favicon_url`), we'll quickly find 
`rustc_attr_parsing::context::ATTRIBUTE_PARSERS`.

a.k.a: a "big ass list".

DocParser feeds a Symbol (some index) through `parse_single_doc_attr_item` ; for example `sym::alias`

:warning: no clue how each `sym::value` are generated, rust analyzer refuses to go to definition, it's probably through some macro, and `rustc_hir::attrs::DocAttribute` seems involved.
Let's try something simple, a simple "word", to know if the trait should be "labeled". We'll add the color later. We can look into the implementation of "hidden" `#[doc(hidden)]`.

```rs
Some(sym::label_trait) => no_args!(label_trait),
```

Cool, I'm done!

```
error[E0531]: cannot find unit struct, unit variant or constant `label_trait` in module `sym`
   --> compiler/rustc_attr_parsing/src/attributes/doc.rs:506:23
    |
506 |             Some(sym::label_trait) => no_args!(label_trait),
    |                       ^^^^^^^^^^^ not found in `sym`

error[E0609]: no field `label_trait` on type `DocAttribute`
   --> compiler/rustc_attr_parsing/src/attributes/doc.rs:506:48
    |
441 |                 self.attribute.$ident = Some(path.span());
    |                                ------ due to this macro variable
...
506 |             Some(sym::label_trait) => no_args!(label_trait),
    |                                                ^^^^^^^^^^^ unknown field
    |
    = note: available fields are: `first_span`, `aliases`, `hidden`, `inline`, `cfg` ... and 17 others
```

Sure.

Ok `DocAttribute` is straightforward! The other is about the symbol, nice, breadcrumbs to follow!

Oh. Now THAT's a big ass struct. `compiler::rustc_span::symbol` has more than 2000 values! No wonder rust analyzer struggles with parsing that.

> // There is currently no checking that all symbols are used; that would be nice to have.

-- Nicholas Nethercote, when the list was ~1000 symbols, [in 2020](https://github.com/rust-lang/rust/commit/fd8f1772347d122b223ef573aeaa34cfa93ceec5).

---

Back to our horse.

I'm tempted to add `label_trait` in there, but I notice there is also `doc_notable_trait`, I'll probably have to add a `doc_label_trait`. We'll see.

