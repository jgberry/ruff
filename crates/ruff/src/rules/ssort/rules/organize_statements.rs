use crate::checkers::ast::Checker;
use ruff_diagnostics::{AutofixKind, Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_semantic::{Binding, Scope};

/// ## What it does
/// Groups and sorts a statements based on the order in which they are referenced.
///
/// ## Why is this bad?
/// Consistency is good. Use a common convention for statement ordering to make your code more
/// readable and idiomatic.
///
/// ## Example
/// ```python
/// def h(): g()
/// def f(): pass
/// def g(): f()
/// ```
///
/// Use instead:
/// ```python
/// def f(): pass
/// def g(): f()
/// def h(): g()
/// ```
#[violation]
pub struct UnsortedStatements;

impl Violation for UnsortedStatements {
    const AUTOFIX: AutofixKind = AutofixKind::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Statements are un-sorted")
    }

    fn autofix_title(&self) -> Option<String> {
        Some("Organize statements".to_string())
    }
}

pub(crate) fn organize_statements(
    checker: &Checker,
    scope: &Scope,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    let bindings: Vec<(&str, &Binding<'_>)> = scope
        .bindings()
        .map(|(name, id)| (name, &checker.semantic().bindings[id]))
        .filter(|(_, binding)| !binding.kind.is_builtin())
        .collect();
    println!("Bindings: {bindings:?}");
}
