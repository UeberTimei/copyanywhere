/// GitHub release notes are markdown; the window shows them as plain lines.
pub fn plain_notes(markdown: &str) -> String {
    markdown
        .lines()
        .map(|line| match line.trim_start() {
            heading if heading.starts_with('#') => heading.trim_start_matches('#').trim(),
            item if item.starts_with("* ") => &item[2..],
            other => other,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests;
