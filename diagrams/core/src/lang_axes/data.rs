//! The scores.
//!
//! These are judgements, not measurements, so the rubric is written down and
//! each axis is defined by what the *compiler* guarantees rather than by what
//! the culture around the language believes. Disagreeing with a number should
//! be easy; that is the point.
//!
//! `control` — how much of the machine the language lets you decide:
//!     memory layout, allocation strategy, absence of a runtime, teardown
//!     timing, raw pointers. 1.0 = you can decide all of it.
//!
//! `safety` — what the toolchain rejects *at compile time*, not at review time:
//!     memory safety, freedom from data races, absence of UB, null handling.
//!     1.0 = the class of bug cannot reach a running program.
//!
//! `cost` — what it takes to wield: learning curve, compile times, iteration
//!     friction, ceremony on the way to a first working version.
//!     1.0 = expensive. This is the axis where Rust does worst, and it is here
//!     precisely so the chart can be wrong in Rust's disfavour.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    /// No garbage collector: the program decides when memory goes away.
    Manual,
    /// A collector (or an interpreter) decides for you.
    Managed,
}

#[derive(Clone, Copy, Debug)]
pub struct Lang {
    pub name: &'static str,
    pub control: f32,
    pub safety: f32,
    pub cost: f32,
    pub family: Family,
    /// The subject of the article: painted in the accent slot.
    pub subject: bool,
    /// One line of justification, surfaced in the table view.
    pub note: &'static str,
}

use Family::{Managed, Manual};

pub const LANGS: &[Lang] = &[
    Lang {
        name: "C",
        control: 0.95,
        safety: 0.10,
        cost: 0.35,
        family: Manual,
        subject: false,
        note: "Total control, no guarantees, and UB that invalidates reasoning about the whole program.",
    },
    Lang {
        name: "C++",
        control: 0.95,
        safety: 0.25,
        cost: 0.75,
        family: Manual,
        subject: false,
        note: "RAII and smart pointers are real gains, but nothing is enforced; the surface area is enormous.",
    },
    Lang {
        name: "Zig",
        control: 0.95,
        safety: 0.40,
        cost: 0.45,
        family: Manual,
        subject: false,
        note: "Runtime safety checks and explicit allocators, but no borrow checker and no data-race protection.",
    },
    Lang {
        name: "Rust",
        control: 0.90,
        safety: 0.95,
        cost: 0.80,
        family: Manual,
        subject: true,
        note: "Alone in rejecting data races at compile time without a GC. Not 1.0: unsafe exists, and logic bugs remain.",
    },
    Lang {
        name: "Go",
        control: 0.45,
        safety: 0.60,
        cost: 0.20,
        family: Managed,
        subject: false,
        note: "Memory safe and famously cheap to learn, but data races are possible and easy to write.",
    },
    Lang {
        name: "C#",
        control: 0.35,
        safety: 0.70,
        cost: 0.30,
        family: Managed,
        subject: false,
        note: "More control than its reputation (structs, Span<T>, stackalloc); nullable refs closed a real hole.",
    },
    Lang {
        name: "JS",
        control: 0.10,
        safety: 0.45,
        cost: 0.15,
        family: Managed,
        subject: false,
        note: "Safe from crashes, unsafe from nonsense: no data races because there are no threads to race.",
    },
    Lang {
        name: "Python",
        control: 0.03,
        safety: 0.50,
        cost: 0.10,
        family: Managed,
        subject: false,
        note: "Cheapest to start; the type system asks nothing of you and catches nothing for you.",
    },
];

impl Lang {
    /// Signed distance above the assumed trade-off line `control + safety = 1`.
    /// Positive means the language is doing something the folk model says is
    /// impossible.
    pub fn escape(&self) -> f32 {
        self.control + self.safety - 1.0
    }
}
