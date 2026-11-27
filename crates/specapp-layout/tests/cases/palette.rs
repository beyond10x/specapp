//! The palette passes the contrast rules in both themes, matches the colour list of the
//! schemas, and every icon is a path.

use specapp_core::manifest::PALETTE;
use specapp_layout::palette::{self, COLOURS, DARK, LIGHT, contrast};
use specapp_layout::{icons, rows::pava};

#[test]
fn text_on_every_fill_and_every_stroke_on_the_page_pass_contrast_in_both_themes() {
    for c in COLOURS {
        for (theme, page, fill, stroke) in [
            ("light", LIGHT, c.light_fill, c.light_stroke),
            ("dark", DARK, c.dark_fill, c.dark_stroke),
        ] {
            let text = contrast(page.text, fill);
            assert!(text >= 4.5, "{theme} {}: text on fill {text:.2}", c.name);
            let line = contrast(stroke, page.background);
            assert!(line >= 3.0, "{theme} {}: stroke on page {line:.2}", c.name);
            let border = contrast(stroke, fill);
            assert!(
                border >= 3.0,
                "{theme} {}: stroke on fill {border:.2}",
                c.name
            );
        }
    }
    for (theme, page) in [("light", LIGHT), ("dark", DARK)] {
        for (what, fg, bg) in [
            ("text", page.text, page.background),
            ("muted", page.muted, page.background),
            ("muted on band", page.muted, page.band),
            ("link", page.link, page.background),
            ("mark text", page.finding_text, page.finding),
        ] {
            let ratio = contrast(fg, bg);
            assert!(ratio >= 4.5, "{theme} {what}: {ratio:.2}");
        }
        for (what, fg) in [("line", page.line), ("finding", page.finding)] {
            let ratio = contrast(fg, page.background);
            assert!(ratio >= 3.0, "{theme} {what}: {ratio:.2}");
        }
    }
}

#[test]
fn the_palette_has_the_names_of_the_manifest_in_their_order() {
    let names: Vec<&str> = COLOURS.iter().map(|c| c.name).collect();
    assert_eq!(names, PALETTE);
    assert_eq!(palette::NAMES, PALETTE);
    let css = palette::css();
    for name in PALETTE {
        assert_eq!(css.matches(&format!(".c-{name}{{")).count(), 2, "{name}");
    }
    assert!(css.contains("prefers-color-scheme: dark"));
}

#[test]
fn every_icon_is_a_path_and_names_are_unique() {
    let mut names: Vec<&str> = icons::ICONS.iter().map(|(n, _)| *n).collect();
    for (name, d) in icons::ICONS {
        assert!(d.starts_with('M'), "{name}");
        assert_eq!(icons::path(name), Some(*d));
    }
    let n = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), n);
    assert_eq!(icons::path("no-such-icon"), None);
}

#[test]
fn isotonic_regression_is_non_decreasing_and_keeps_an_ordered_target() {
    let ordered = [1.0, 2.0, 5.0];
    assert_eq!(pava(&ordered, &[1.0, 1.0, 1.0]), ordered);
    let pooled = pava(&[3.0, 1.0, 2.0], &[1.0, 1.0, 1.0]);
    assert!(pooled.windows(2).all(|w| w[0] <= w[1]), "{pooled:?}");
    assert_eq!(pooled, vec![2.0, 2.0, 2.0]);
}
