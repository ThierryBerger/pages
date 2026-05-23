//! Build-time renderer for every diagram.
//!
//! Emits the static image used as the pre-JS paint, the no-script fallback and
//! the feed representation, from the same core the wasm module runs.
//!
//!   diagrams-svg <out-dir>
//!   diagrams-svg <out-dir> --frames <diagram> <dir>   dump a whole sweep

use diagrams_core::{async_tasks, concurrency, lang_axes};
use std::{env, fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let root: PathBuf = args
        .get(1)
        .filter(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| "static/diagrams".into())
        .into();

    write_one(&root.join("lang-axes"), "fallback.svg",
              &lang_axes::svg(lang_axes::Params { t: 2.0, ..Default::default() }))?;
    fs::write(root.join("lang-axes").join("table.html"), table())?;

    write_one(&root.join("concurrency"), "fallback.svg",
              &concurrency::svg(concurrency::Params { t: 4.0, ..Default::default() }))?;

    write_one(&root.join("async"), "fallback.svg",
              &async_tasks::svg(async_tasks::Params { t: 6.0, ..Default::default() }))?;

    if let Some(i) = args.iter().position(|a| a == "--frames") {
        let which = args.get(i + 1).cloned().unwrap_or_default();
        let dir = PathBuf::from(args.get(i + 2).cloned().unwrap_or_else(|| "frames".into()));
        fs::create_dir_all(&dir)?;
        let (last, render): (f32, Box<dyn Fn(f32) -> String>) = match which.as_str() {
            "concurrency" => (4.0, Box::new(|t| concurrency::svg(concurrency::Params { t, ..Default::default() }))),
            "async" => (6.0, Box::new(|t| async_tasks::svg(async_tasks::Params { t, ..Default::default() }))),
            _ => (2.0, Box::new(|t| lang_axes::svg(lang_axes::Params { t, ..Default::default() }))),
        };
        // Two frames per stage plus the endpoints: enough to catch a transition
        // that only looks wrong halfway through.
        let steps = (last * 2.0) as usize;
        for k in 0..=steps {
            let t = k as f32 * last / steps as f32;
            fs::write(dir.join(format!("{which}{k}.svg")), render(t))?;
        }
        println!("dumped {} frames for {which} to {}", steps + 1, dir.display());
    }

    // `--runs <dir>` dumps the final concurrency stage once per schedule, to
    // confirm the called-out pair really does change order between runs.
    if let Some(i) = args.iter().position(|a| a == "--runs") {
        let dir = PathBuf::from(args.get(i + 1).cloned().unwrap_or_else(|| "runs".into()));
        fs::create_dir_all(&dir)?;
        for r in 0..concurrency::RUNS {
            let body = concurrency::svg(concurrency::Params {
                t: 4.0,
                run: r as f32,
                ..Default::default()
            });
            fs::write(dir.join(format!("run{r}.svg")), body)?;
        }
        println!("dumped {} runs to {}", concurrency::RUNS, dir.display());
    }

    Ok(())
}

fn write_one(dir: &PathBuf, name: &str, body: &str) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    fs::write(dir.join(name), body)?;
    println!("{:>7} bytes  {}", body.len(), dir.join(name).display());
    Ok(())
}

fn table() -> String {
    use diagrams_core::lang_axes::{data::Family, LANGS};
    let mut s = String::from(
        "<table class=\"la-table\">\n<caption>Scores behind the diagram. \
         These are judgements against a written rubric, not measurements.</caption>\n\
         <thead><tr><th scope=\"col\">Language</th><th scope=\"col\">Control</th>\
         <th scope=\"col\">Safety</th><th scope=\"col\">Cost to wield</th>\
         <th scope=\"col\">Memory</th><th scope=\"col\">Why</th></tr></thead>\n<tbody>\n",
    );
    for l in LANGS {
        let mem = match l.family {
            Family::Manual => "manual",
            Family::Managed => "garbage collected",
        };
        s.push_str(&format!(
            "<tr><th scope=\"row\">{}</th><td>{:.2}</td><td>{:.2}</td><td>{:.2}</td>\
             <td>{}</td><td>{}</td></tr>\n",
            l.name, l.control, l.safety, l.cost, mem, l.note
        ));
    }
    s.push_str("</tbody>\n</table>\n");
    s
}
