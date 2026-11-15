//! The `selected` state of an element with the `option` role follows its native `selected`
//! attribute. An explicit `aria-selected` attribute takes precedence over the native attribute.

use accesskit::{Node as AccessKitNode, Role};
use blitz_dom::DocumentConfig;
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;
use test_that::prelude::*;

#[test]
fn option_with_native_selected_attribute_is_selected() -> TestResult<()> {
    let document = make_document(r#"<div role="option" selected>One</div>"#);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): matches_pattern!(Role::ListBoxOption),
                is_selected(): some(eq(true)),
            })
        ))
    )
}

#[test]
fn option_without_native_selected_attribute_is_unselected() -> TestResult<()> {
    let document = make_document(r#"<div role="option">One</div>"#);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): matches_pattern!(Role::ListBoxOption),
                is_selected(): some(eq(false)),
            })
        ))
    )
}

#[test]
fn aria_selected_false_overrides_native_selected_attribute() -> TestResult<()> {
    let document = make_document(r#"<div role="option" selected aria-selected="false">One</div>"#);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): matches_pattern!(Role::ListBoxOption),
                is_selected(): some(eq(false)),
            })
        ))
    )
}

#[test]
fn aria_selected_true_applies_without_native_selected_attribute() -> TestResult<()> {
    let document = make_document(r#"<div role="option" aria-selected="true">One</div>"#);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): matches_pattern!(Role::ListBoxOption),
                is_selected(): some(eq(true)),
            })
        ))
    )
}

#[test]
fn native_selected_attribute_is_only_mapped_on_options() -> TestResult<()> {
    let document = make_document(r#"<div selected>One</div>"#);

    let tree_update = document.build_accessibility_tree();

    verify_that!(
        tree_update.nodes,
        contains((
            anything(),
            matches_pattern!(AccessKitNode {
                role(): eq(Role::GenericContainer),
                is_selected(): none(),
            })
        ))
    )
}

fn make_document(body: &str) -> HtmlDocument {
    let mut document = HtmlDocument::from_html(
        &format!("<html><body>{body}</body></html>"),
        default_document_config(),
    );
    document.resolve(0.0);
    document
}

fn default_document_config() -> DocumentConfig {
    DocumentConfig {
        viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
        html_parser_provider: Some(Arc::new(HtmlProvider) as _),
        ..Default::default()
    }
}
