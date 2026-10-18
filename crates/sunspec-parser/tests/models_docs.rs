//! Generates the "Supported SunSpec models" pages of the docs site from the bundled
//! definitions, and checks the committed pages are up to date.
//!
//! Regenerate after changing the bundled models:
//!
//! ```sh
//! UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
//! ```

use std::fmt::Write as _;
use std::ops::RangeInclusive;
use std::path::PathBuf;

use sunspec_parser::{catalog, ModelInfo, PointInfo};

/// One page per range of model IDs, as the SunSpec specification groups them.
const CATEGORIES: &[(&str, &str, RangeInclusive<u16>)] = &[
    ("common", "Common and network", 1..=99),
    ("inverters", "Inverters and inverter controls", 100..=199),
    ("meters", "Meters", 200..=299),
    ("environmental", "Environmental sensors", 300..=399),
    ("string-combiners", "String combiners", 400..=499),
    ("modules", "Solar modules", 500..=599),
    ("trackers", "Trackers", 600..=699),
    ("der", "DER (IEEE 1547-2018)", 700..=799),
    ("storage", "Storage", 800..=899),
    ("other", "Test and vendor models", 900..=u16::MAX),
];

const GENERATED: &str = "<!-- Generated from the bundled SunSpec definitions by\n     \
UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs\n     \
Do not edit by hand. -->";

const DIR: &str = "docs/reference/models";

#[test]
fn model_pages_are_up_to_date() {
    let pages = render();
    let dir = repo_path(DIR);
    if std::env::var_os("UPDATE_MODEL_DOCS").is_some() {
        std::fs::create_dir_all(&dir).unwrap();
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|ext| ext == "md") {
                std::fs::remove_file(path).unwrap();
            }
        }
        for (name, content) in &pages {
            std::fs::write(dir.join(name), content).unwrap();
        }
        return;
    }

    let hint = "run `UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs`";
    for (name, content) in &pages {
        let path = dir.join(name);
        let on_disk = std::fs::read_to_string(&path).unwrap_or_default();
        assert!(on_disk == *content, "{DIR}/{name} is out of date; {hint}");
    }
    let mut committed: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".md"))
        .collect();
    committed.sort();
    let mut expected: Vec<String> = pages.iter().map(|(name, _)| name.clone()).collect();
    expected.sort();
    assert_eq!(committed, expected, "unexpected files in {DIR}; {hint}");
}

/// Every generated page must be in the book's table of contents, or mdBook skips it.
#[test]
fn every_model_page_is_in_the_summary() {
    let summary = std::fs::read_to_string(repo_path("docs/SUMMARY.md")).unwrap();
    for (name, _) in render() {
        let link = format!("(reference/models/{name})");
        assert!(
            summary.contains(&link),
            "docs/SUMMARY.md does not list {link}"
        );
    }
}

fn render() -> Vec<(String, String)> {
    let models = catalog();
    let mut pages = vec![("README.md".to_string(), index(&models))];
    for (file, title, ids) in CATEGORIES {
        let in_category: Vec<&ModelInfo> = models
            .iter()
            .filter(|model| ids.contains(&model.id))
            .collect();
        if !in_category.is_empty() {
            pages.push((format!("{file}.md"), category(title, &in_category)));
        }
    }
    pages
}

fn index(models: &[ModelInfo]) -> String {
    let mut out = String::new();
    let points: usize = models.iter().map(|model| model.points.len()).sum();
    writeln!(out, "# Supported SunSpec models\n\n{GENERATED}\n").unwrap();
    writeln!(
        out,
        "The collector bundles {} official SunSpec model definitions ({points} points) and decodes \
         every one of them. Each page below lists a group of models with their points: the name \
         records use, the type, units, label and description from the definitions, and the \
         names of enum values and bitfield bits. In a point name, `[*]` stands for a repeating \
         group's index (`module[*].DCA` covers `module[0].DCA`, `module[1].DCA` and so on).\n",
        models.len()
    )
    .unwrap();
    writeln!(
        out,
        "The same data is available from a running collector at `/models` and from \
         `collector-app --print-models`.\n"
    )
    .unwrap();
    for (file, title, ids) in CATEGORIES {
        let in_category: Vec<&ModelInfo> = models
            .iter()
            .filter(|model| ids.contains(&model.id))
            .collect();
        if in_category.is_empty() {
            continue;
        }
        writeln!(out, "## [{title}]({file}.md)\n").unwrap();
        writeln!(out, "| Model | Name in records | Label | Points |").unwrap();
        writeln!(out, "|-------|-----------------|-------|--------|").unwrap();
        for model in in_category {
            writeln!(
                out,
                "| [{}]({file}.md#{}) | `{}` | {} | {} |",
                model.id,
                anchor(&heading(model)),
                model.name,
                cell(model.label.as_deref().unwrap_or("")),
                model.points.len()
            )
            .unwrap();
        }
        out.push('\n');
    }
    finish(out)
}

fn category(title: &str, models: &[&ModelInfo]) -> String {
    let mut out = String::new();
    writeln!(out, "# {title}\n\n{GENERATED}\n").unwrap();
    writeln!(
        out,
        "Point names are the ones records use; `[*]` stands for a repeating group's index. See \
         [Supported SunSpec models](README.md) for every group.\n"
    )
    .unwrap();
    writeln!(out, "| Model | Name in records | Points |").unwrap();
    writeln!(out, "|-------|-----------------|--------|").unwrap();
    for model in models {
        writeln!(
            out,
            "| [{}](#{}) | `{}` | {} |",
            model.id,
            anchor(&heading(model)),
            model.name,
            model.points.len()
        )
        .unwrap();
    }
    for model in models {
        writeln!(out, "\n## {}\n", text(&heading(model))).unwrap();
        writeln!(out, "Name in records: `{}`.\n", model.name).unwrap();
        if let Some(description) = &model.description {
            writeln!(out, "{}\n", text(description)).unwrap();
        }
        if model.points.is_empty() {
            writeln!(out, "This model has no points besides its header.").unwrap();
            continue;
        }
        writeln!(out, "| Point | Type | Units | Label | Description |").unwrap();
        writeln!(out, "|-------|------|-------|-------|-------------|").unwrap();
        for point in &model.points {
            writeln!(
                out,
                "| `{}` | {} | {} | {} | {} |",
                point.name,
                kind(point),
                cell(point.units.as_deref().unwrap_or("")),
                cell(point.label.as_deref().unwrap_or("")),
                cell(point.description.as_deref().unwrap_or(""))
            )
            .unwrap();
        }
        let with_symbols: Vec<&PointInfo> = model
            .points
            .iter()
            .filter(|point| !point.symbols.is_empty())
            .collect();
        if !with_symbols.is_empty() {
            writeln!(out, "\nValue names:\n").unwrap();
            for point in with_symbols {
                let bitfield = kind(point).starts_with("bitfield");
                let names: Vec<String> = point
                    .symbols
                    .iter()
                    .map(|symbol| {
                        if bitfield {
                            format!("bit {} `{}`", symbol.value, symbol.name)
                        } else {
                            format!("{} `{}`", symbol.value, symbol.name)
                        }
                    })
                    .collect();
                writeln!(
                    out,
                    "- `{}` ({}): {}",
                    point.name,
                    kind(point),
                    names.join(", ")
                )
                .unwrap();
            }
        }
    }
    finish(out)
}

fn heading(model: &ModelInfo) -> String {
    format!(
        "Model {}: {}",
        model.id,
        model.label.as_deref().unwrap_or(&model.name)
    )
}

fn kind(point: &PointInfo) -> String {
    serde_json::to_value(point.kind)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_default()
}

/// Text outside tables: one line, with the characters Markdown or HTML would interpret
/// escaped.
fn text(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace('\\', "\\\\")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('`', "\\`")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// A table cell: as `text`, with pipes escaped too.
fn cell(value: &str) -> String {
    text(value).replace('|', "\\|")
}

/// The id mdBook gives a heading (`id_from_content` / `normalize_id` in mdBook 0.4).
fn anchor(heading: &str) -> String {
    heading
        .chars()
        .filter_map(|ch| {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                Some(ch.to_ascii_lowercase())
            } else if ch.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}

fn finish(mut out: String) -> String {
    while out.ends_with("\n\n") {
        out.pop();
    }
    out
}

fn repo_path(relative: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../..");
    path.push(relative);
    path
}
