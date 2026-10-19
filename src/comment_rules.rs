use std::collections::{BTreeMap, HashSet};
use std::ops::Range;

use rustc_errors::DiagDecorator;
use rustc_hir::{Attribute, Expr, ExprKind, HirId, Mod};
use rustc_lexer::{FrontmatterAllowed, TokenKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{BytePos, FileName, Span};

use crate::comment_policy::{CommentPolicy, is_required_header};

rustc_session::declare_lint! {
    pub(crate) NO_STACKED_COMMENTS,
    Allow,
    "LEG061: avoid consecutive standalone ordinary line comments"
}

rustc_session::declare_lint! {
    pub(crate) NO_UNMATCHED_COMMENTS,
    Allow,
    "LEG039: require ordinary comments to match the configured policy"
}

rustc_session::declare_lint! {
    pub(crate) NO_AUTOMATED_COMMENT_ATTRIBUTION,
    Warn,
    "LEG040: reject explicit automated authorship signatures in ordinary comments"
}

rustc_session::declare_lint! {
    pub(crate) PREFER_LINE_COMMENTS,
    Warn,
    "LEG041: prefer line comments for ordinary prose"
}

rustc_session::impl_lint_pass!(CommentRules => [
    NO_STACKED_COMMENTS,
    NO_UNMATCHED_COMMENTS,
    NO_AUTOMATED_COMMENT_ATTRIBUTION,
    PREFER_LINE_COMMENTS
]);

const IDENTIFIERS: &[&str] = &[
    "ai", "chatgpt", "claude", "codex", "copilot", "gemini", "gpt", "llm", "openai",
];

const GENERATION_VERBS: &[&str] = &["authored", "created", "generated", "produced", "written"];

pub(crate) struct CommentRules {
    policy: CommentPolicy,
    scanned: HashSet<BytePos>,
    comments: BTreeMap<BytePos, Comment>,
}

struct Comment {
    span: Span,
    owner: HirId,
    previous_owners: Vec<HirId>,
    checks: CommentChecks,
}

struct CommentChecks {
    stacked: bool,
    unmatched: bool,
    attribution: bool,
    block_prose: bool,
}

impl<'tcx> LateLintPass<'tcx> for CommentRules {
    fn check_mod(&mut self, cx: &LateContext<'tcx>, module: &'tcx Mod<'tcx>, owner: HirId) {
        self.scan(cx, module.spans.inner_span, owner);
    }

    fn check_expr(&mut self, _: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let should_skip = expr.span.from_expansion() || contains_prose(expr);
        if should_skip {
            return;
        }
        for (_, comment) in self.comments.range_mut(expr.span.lo()..expr.span.hi()) {
            comment.checks.block_prose = false;
        }
    }

    fn check_attributes(&mut self, cx: &LateContext<'tcx>, attributes: &'tcx [Attribute]) {
        let should_skip = attributes.is_empty() || self.comments.is_empty();
        if should_skip {
            return;
        }
        let owner = cx.last_node_with_lint_attrs;
        let span = cx.tcx.hir_span_with_body(owner);
        if span.from_expansion() {
            return;
        }
        for (_, comment) in self.comments.range_mut(span.lo()..span.hi()) {
            comment.owner = owner;
        }
    }

    fn check_crate_post(&mut self, cx: &LateContext<'tcx>) {
        for comment in self.comments.values() {
            report_comment(cx, comment);
        }
    }
}

impl CommentRules {
    pub(crate) fn new(policy: CommentPolicy) -> Self {
        Self {
            policy,
            scanned: HashSet::new(),
            comments: BTreeMap::new(),
        }
    }

    fn scan(&mut self, cx: &LateContext<'_>, span: Span, owner: HirId) {
        if span.from_expansion() {
            return;
        }
        let file = cx.sess().source_map().lookup_source_file(span.lo());
        if !matches!(file.name, FileName::Real(_)) {
            return;
        }
        let base = span.with_lo(file.start_pos);
        if self.scanned.insert(file.start_pos) {
            self.scan_source(file.src.as_deref().map(String::as_str), base, owner);
        } else {
            self.revisit(cx, base, owner);
        }
    }

    fn scan_source(&mut self, source: Option<&str>, base: Span, owner: HirId) {
        let Some(source) = source else {
            return;
        };
        for (span, checks) in matched_comments(source, base, &self.policy) {
            let comment = Comment {
                span,
                owner,
                previous_owners: Vec::new(),
                checks,
            };
            self.comments.insert(span.lo(), comment);
        }
    }

    fn revisit(&mut self, cx: &LateContext<'_>, span: Span, owner: HirId) {
        if span.contains(cx.tcx.hir_span_with_body(owner)) {
            return;
        }
        for (_, comment) in self.comments.range_mut(span.lo()..span.hi()) {
            comment.previous_owners.push(comment.owner);
            comment.owner = owner;
        }
    }
}

fn matched_comments(
    source: &str,
    base: Span,
    policy: &CommentPolicy,
) -> Vec<(Span, CommentChecks)> {
    let mut offset = rustc_lexer::strip_shebang(source).unwrap_or(0);
    let mut previous = None;
    let mut comments = Vec::new();
    for token in rustc_lexer::tokenize(&source[offset..], FrontmatterAllowed::Yes) {
        let end = offset + token.len as usize;
        let stacked = is_stacked(source, offset..end, token.kind, &mut previous);
        if let Some(checks) = match_comment(source, offset..end, token.kind, policy, stacked) {
            let span = comment_span(base, offset);
            comments.push((span, checks));
        }
        offset = end;
    }
    comments
}

fn comment_span(base: Span, offset: usize) -> Span {
    base.with_lo(base.lo() + BytePos(offset as u32))
        .shrink_to_lo()
}

fn is_stacked(
    source: &str,
    range: Range<usize>,
    kind: TokenKind,
    previous: &mut Option<usize>,
) -> bool {
    if matches!(kind, TokenKind::Whitespace) {
        return false;
    }
    let last = previous.take();
    if !is_stackable_comment(source, range.clone(), kind) {
        return false;
    }
    *previous = Some(range.end);
    last.is_some_and(|end| {
        source[end..range.start]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count()
            == 1
    })
}

fn is_stackable_comment(source: &str, range: Range<usize>, kind: TokenKind) -> bool {
    let is_line = matches!(kind, TokenKind::LineComment { doc_style: None });
    let is_standalone_line = is_line && is_standalone(source, range.clone());
    if !is_standalone_line {
        return false;
    }
    let body = comment_body(&source[range]);
    let is_exempt = is_required_header(body) || body.lines().any(is_generation_marker);
    !is_exempt
}

fn match_comment(
    source: &str,
    range: Range<usize>,
    kind: TokenKind,
    policy: &CommentPolicy,
    stacked: bool,
) -> Option<CommentChecks> {
    if !is_ordinary_comment(kind) {
        return None;
    }
    let comment = &source[range.clone()];
    let body = comment_body(comment);
    let is_generated = body.lines().any(is_generation_marker);
    let unmatched = !is_generated && policy.is_unmatched(body);
    let attribution = has_attribution(comment);
    let block_prose = comment.starts_with("/*") && is_standalone(source, range);
    let has_match = stacked || unmatched || attribution || block_prose;
    has_match.then_some(CommentChecks {
        stacked,
        unmatched,
        attribution,
        block_prose,
    })
}

fn is_standalone(source: &str, range: Range<usize>) -> bool {
    let is_horizontal_space = |character: char| character != '\n' && character.is_whitespace();
    let before = source[..range.start].trim_end_matches(is_horizontal_space);
    let after = source[range.end..].trim_start_matches(is_horizontal_space);
    let starts_line = before.is_empty() || before.ends_with('\n');
    let ends_line = after.is_empty() || after.starts_with('\n');
    starts_line && ends_line
}

fn contains_prose(expr: &Expr<'_>) -> bool {
    matches!(
        expr.kind,
        ExprKind::Block(..)
            | ExprKind::If(..)
            | ExprKind::Loop(..)
            | ExprKind::Match(..)
            | ExprKind::Closure(..)
            | ExprKind::ConstBlock(..)
            | ExprKind::DropTemps(..)
    )
}

fn is_ordinary_comment(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::LineComment { doc_style: None }
            | TokenKind::BlockComment {
                doc_style: None,
                terminated: true
            }
    )
}

fn has_attribution(comment: &str) -> bool {
    let body = comment_body(comment);
    if body.lines().any(is_generation_marker) {
        return false;
    }
    let has_author = body.lines().any(author_signature);
    has_author || generation_signature(body)
}

fn comment_body(comment: &str) -> &str {
    if let Some(body) = comment.strip_prefix("//") {
        return body;
    }
    comment
        .strip_prefix("/*")
        .and_then(|body| body.strip_suffix("*/"))
        .unwrap_or(comment)
}

fn is_generation_marker(line: &str) -> bool {
    let line = line.trim().trim_start_matches('*').trim_start();
    let is_tag = line.split_whitespace().next() == Some("@generated");
    let is_header = line.starts_with("Code generated ") && line.ends_with("DO NOT EDIT.");
    is_tag || is_header
}

fn author_signature(line: &str) -> bool {
    let mut is_author = false;
    for word in words(line) {
        is_author |= word.eq_ignore_ascii_case("@author");
        let is_automated_author = is_author && is_identifier(word);
        if is_automated_author {
            return true;
        }
    }
    false
}

fn generation_signature(comment: &str) -> bool {
    let mut previous = [""; 3];
    for word in words(comment) {
        let is_passive = is_identifier(word) && generation_prefix(previous);
        let is_active = is_identifier(previous[2]) && is_generation_verb(word);
        let has_signature = is_passive || is_active;
        if has_signature {
            return true;
        }
        previous.rotate_left(1);
        previous[2] = word;
    }
    false
}

fn generation_prefix(previous: [&str; 3]) -> bool {
    let [verb, link, article] = previous;
    let is_article = article.eq_ignore_ascii_case("a") || article.eq_ignore_ascii_case("an");
    let with_article = is_generation_verb(verb) && link.eq_ignore_ascii_case("by") && is_article;
    let without_article = is_generation_verb(link) && article.eq_ignore_ascii_case("by");
    with_article || without_article
}

fn is_generation_verb(word: &str) -> bool {
    GENERATION_VERBS
        .iter()
        .any(|verb| verb.eq_ignore_ascii_case(word))
}

fn words(value: &str) -> impl Iterator<Item = &str> {
    value
        .split(|character: char| !character.is_alphanumeric() && character != '@')
        .filter(|word| !word.is_empty())
}

fn is_identifier(word: &str) -> bool {
    let word = word.strip_prefix('@').unwrap_or(word);
    IDENTIFIERS
        .iter()
        .any(|identifier| identifier.eq_ignore_ascii_case(word))
}

fn report_comment(cx: &LateContext<'_>, comment: &Comment) {
    let owners = comment
        .previous_owners
        .iter()
        .chain(std::iter::once(&comment.owner));
    for owner in owners {
        report_checks(cx, comment, *owner);
    }
}

fn report_checks(cx: &LateContext<'_>, comment: &Comment, owner: HirId) {
    if comment.checks.stacked {
        report_stacked(cx, comment.span, owner);
    }
    if comment.checks.unmatched {
        report_unmatched(cx, comment.span, owner);
    }
    if comment.checks.attribution {
        report_attribution(cx, comment.span, owner);
    }
    if comment.checks.block_prose {
        report_line_comment(cx, comment.span, owner);
    }
}

fn report_stacked(cx: &LateContext<'_>, span: Span, owner: HirId) {
    cx.tcx.emit_node_span_lint(
        NO_STACKED_COMMENTS,
        owner,
        span,
        DiagDecorator(|diag| {
            diag.primary_message("LEG061 no-stacked-comments: consecutive standalone comments");
            diag.help("remove redundant comments or combine the useful explanation");
        }),
    );
}

fn report_unmatched(cx: &LateContext<'_>, span: Span, owner: HirId) {
    cx.tcx.emit_node_span_lint(
        NO_UNMATCHED_COMMENTS,
        owner,
        span,
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG039 no-unmatched-comments: comment does not match the configured policy",
            );
            diag.help("use an allowed pattern, prefix, or suffix; configure the comment policy in dylint.toml");
        }),
    );
}

fn report_line_comment(cx: &LateContext<'_>, span: Span, owner: HirId) {
    cx.tcx.emit_node_span_lint(
        PREFER_LINE_COMMENTS,
        owner,
        span,
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG041 prefer-line-comments: use line comments for ordinary prose",
            );
            diag.help("replace the block comment with `//` comments");
        }),
    );
}

fn report_attribution(cx: &LateContext<'_>, span: Span, owner: HirId) {
    cx.tcx.emit_node_span_lint(
        NO_AUTOMATED_COMMENT_ATTRIBUTION,
        owner,
        span,
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG040 no-automated-comment-attribution: remove automated authorship attribution",
            );
            diag.help("keep the useful explanation and required generation markers");
        }),
    );
}
