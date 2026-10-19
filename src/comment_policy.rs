use regex::{RegexSet, RegexSetBuilder};

#[derive(Default)]
pub(crate) struct CommentPolicy {
    matchers: RegexSet,
    prefixes: Vec<String>,
    suffixes: Vec<String>,
}

impl CommentPolicy {
    pub(crate) fn new(
        matchers: &[String],
        prefixes: &[String],
        suffixes: &[String],
    ) -> Result<Self, regex::Error> {
        let is_case_insensitive = true;
        let matchers = RegexSetBuilder::new(matchers)
            .case_insensitive(is_case_insensitive)
            .build()?;
        let prefixes = normalize_identifiers(prefixes);
        let suffixes = normalize_identifiers(suffixes);
        Ok(Self {
            matchers,
            prefixes,
            suffixes,
        })
    }

    pub(crate) fn is_unmatched(&self, body: &str) -> bool {
        let value = normalize_body(body);
        let is_allowed = is_required_header(&value) || self.matchers.is_match(&value);
        if is_allowed {
            return false;
        }
        let value = value.to_lowercase();
        let has_prefix = self
            .prefixes
            .iter()
            .any(|prefix| matches_prefix(&value, prefix));
        let has_suffix = self
            .suffixes
            .iter()
            .any(|suffix| matches_suffix(&value, suffix));
        !has_prefix && !has_suffix
    }
}

fn normalize_identifiers(identifiers: &[String]) -> Vec<String> {
    identifiers
        .iter()
        .filter_map(|identifier| {
            let identifier = identifier.trim();
            if identifier.is_empty() {
                return None;
            }
            Some(identifier.to_lowercase())
        })
        .collect()
}

pub(crate) fn is_required_header(value: &str) -> bool {
    let value = normalize_line(value);
    let prefixes = [
        "spdx-license-identifier:",
        "spdx-filecopyrighttext:",
        "copyright ",
        "@ ",
        "~",
    ];
    prefixes.iter().any(|prefix| {
        value
            .get(..prefix.len())
            .is_some_and(|start| start.eq_ignore_ascii_case(prefix))
    })
}

fn normalize_body(body: &str) -> String {
    let lines: Vec<_> = body.lines().map(normalize_line).collect();
    lines.join("\n").trim().to_owned()
}

fn normalize_line(line: &str) -> &str {
    let line = line.trim();
    line.strip_prefix('*').unwrap_or(line).trim_start()
}

fn matches_prefix(value: &str, prefix: &str) -> bool {
    value
        .strip_prefix(prefix)
        .is_some_and(|rest| is_boundary(rest.chars().next()))
}

fn matches_suffix(value: &str, suffix: &str) -> bool {
    value
        .strip_suffix(suffix)
        .is_some_and(|rest| is_boundary(rest.chars().next_back()))
}

fn is_boundary(character: Option<char>) -> bool {
    character.is_none_or(|character| !character.is_alphanumeric() && character != '_')
}
