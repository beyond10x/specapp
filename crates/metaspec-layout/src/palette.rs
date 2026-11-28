//! metaspec's palette: twelve colour names, each with a fill and a stroke per theme, and the
//! page colours. A pack writes only the names. Text is drawn in the page's text colour on the
//! fill; the stroke draws borders and icons on the page background.
//!
//! The contrast rules (WCAG 2): text on a fill at least 4.5:1, a stroke against the page
//! background at least 3:1, in both themes. The tests hold them.

/// The colour names, in the order a block in no family takes them by layer.
pub const NAMES: &[&str] = &[
    "blue", "violet", "green", "teal", "amber", "orange", "red", "pink", "cyan", "lime", "brown",
    "slate",
];

/// One colour: fill and stroke in the light theme, fill and stroke in the dark theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour {
    pub name: &'static str,
    pub light_fill: &'static str,
    pub light_stroke: &'static str,
    pub dark_fill: &'static str,
    pub dark_stroke: &'static str,
}

pub const COLOURS: &[Colour] = &[
    Colour {
        name: "blue",
        light_fill: "#dbeafe",
        light_stroke: "#1d4ed8",
        dark_fill: "#172554",
        dark_stroke: "#60a5fa",
    },
    Colour {
        name: "violet",
        light_fill: "#ede9fe",
        light_stroke: "#6d28d9",
        dark_fill: "#2e1065",
        dark_stroke: "#a78bfa",
    },
    Colour {
        name: "green",
        light_fill: "#dcfce7",
        light_stroke: "#15803d",
        dark_fill: "#052e16",
        dark_stroke: "#4ade80",
    },
    Colour {
        name: "teal",
        light_fill: "#ccfbf1",
        light_stroke: "#0f766e",
        dark_fill: "#042f2e",
        dark_stroke: "#2dd4bf",
    },
    Colour {
        name: "amber",
        light_fill: "#fef3c7",
        light_stroke: "#b45309",
        dark_fill: "#451a03",
        dark_stroke: "#fbbf24",
    },
    Colour {
        name: "orange",
        light_fill: "#ffedd5",
        light_stroke: "#c2410c",
        dark_fill: "#431407",
        dark_stroke: "#fb923c",
    },
    Colour {
        name: "red",
        light_fill: "#fee2e2",
        light_stroke: "#b91c1c",
        dark_fill: "#450a0a",
        dark_stroke: "#f87171",
    },
    Colour {
        name: "pink",
        light_fill: "#fce7f3",
        light_stroke: "#be185d",
        dark_fill: "#500724",
        dark_stroke: "#f472b6",
    },
    Colour {
        name: "cyan",
        light_fill: "#cffafe",
        light_stroke: "#0e7490",
        dark_fill: "#083344",
        dark_stroke: "#22d3ee",
    },
    Colour {
        name: "lime",
        light_fill: "#ecfccb",
        light_stroke: "#4d7c0f",
        dark_fill: "#1a2e05",
        dark_stroke: "#a3e635",
    },
    Colour {
        name: "brown",
        light_fill: "#f3e8dc",
        light_stroke: "#7c4a1e",
        dark_fill: "#2a1a0e",
        dark_stroke: "#d4a373",
    },
    Colour {
        name: "slate",
        light_fill: "#f1f5f9",
        light_stroke: "#334155",
        dark_fill: "#1e293b",
        dark_stroke: "#94a3b8",
    },
];

/// The page colours of one theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    pub background: &'static str,
    pub text: &'static str,
    pub muted: &'static str,
    pub line: &'static str,
    pub band: &'static str,
    pub finding: &'static str,
    pub finding_text: &'static str,
    pub link: &'static str,
}

pub const LIGHT: Page = Page {
    background: "#ffffff",
    text: "#111827",
    muted: "#4b5563",
    line: "#6b7280",
    band: "#f8fafc",
    finding: "#b91c1c",
    finding_text: "#ffffff",
    link: "#1d4ed8",
};

pub const DARK: Page = Page {
    background: "#0f1115",
    text: "#e5e7eb",
    muted: "#9ca3af",
    line: "#9ca3af",
    band: "#161a20",
    finding: "#f87171",
    finding_text: "#0f1115",
    link: "#93c5fd",
};

/// The colour of a name; slate for a name the palette does not have.
pub fn colour(name: &str) -> Colour {
    COLOURS
        .iter()
        .copied()
        .find(|c| c.name == name)
        .unwrap_or(COLOURS[COLOURS.len() - 1])
}

/// The relative luminance of a colour `#rrggbb` (WCAG 2).
pub fn luminance(hex: &str) -> f64 {
    let channel = |i: usize| {
        let v = u8::from_str_radix(&hex[1 + 2 * i..3 + 2 * i], 16).unwrap_or(0) as f64 / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(0) + 0.7152 * channel(1) + 0.0722 * channel(2)
}

/// The contrast ratio of two colours `#rrggbb` (WCAG 2), from 1 to 21.
pub fn contrast(a: &str, b: &str) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

/// The CSS of the palette: page variables per theme, and one class `c-<name>` per colour
/// that sets `--fill` and `--stroke`.
pub fn css() -> String {
    let page = |p: &Page| {
        format!(
            "--bg:{};--fg:{};--muted:{};--line:{};--band:{};--finding:{};--finding-fg:{};--link:{};",
            p.background, p.text, p.muted, p.line, p.band, p.finding, p.finding_text, p.link
        )
    };
    let mut out = format!(":root{{color-scheme:light dark;{}}}\n", page(&LIGHT));
    for c in COLOURS {
        out.push_str(&format!(
            ".c-{}{{--fill:{};--stroke:{}}}\n",
            c.name, c.light_fill, c.light_stroke
        ));
    }
    out.push_str(&format!(
        "@media (prefers-color-scheme: dark){{\n:root{{{}}}\n",
        page(&DARK)
    ));
    for c in COLOURS {
        out.push_str(&format!(
            ".c-{}{{--fill:{};--stroke:{}}}\n",
            c.name, c.dark_fill, c.dark_stroke
        ));
    }
    out.push_str("}\n");
    out
}
