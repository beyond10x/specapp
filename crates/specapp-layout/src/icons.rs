//! specapp's icons: 14 by 14 stroke paths by name. A family of a pack names one of them; any
//! other name is drawn as its first letter in a circle, with the name on hover.

/// The icon names specapp draws, and their paths.
pub const ICONS: &[(&str, &str)] = &[
    (
        "anchor",
        "M7 4.2a1.4 1.4 0 1 0 0-2.8 1.4 1.4 0 0 0 0 2.8zM7 4.2v8.3M4.5 6.5h5M2 8.5c0 2.3 2.2 4 5 4s5-1.7 5-4",
    ),
    ("boxes", "M1.5 7.5h5v5h-5zM7.5 7.5h5v5h-5zM4.5 1.5h5v5h-5z"),
    ("chat", "M2 2.5h10v7H6.5L3.5 12V9.5H2z"),
    (
        "chip",
        "M4 4h6v6H4zM5.5 1.5V4M8.5 1.5V4M5.5 10v2.5M8.5 10v2.5M1.5 5.5H4M1.5 8.5H4M10 5.5h2.5M10 8.5h2.5",
    ),
    ("flag", "M3 12.5V1.5M3 2h7.5l-1.8 2.5L10.5 7H3"),
    ("folder", "M1.5 3.5h4l1.5 1.5h5.5v6.5h-11z"),
    ("gate", "M2 2.5v9M5 7h7M9 4l3 3-3 3"),
    (
        "gavel",
        "M5.2 2.2l3.6 3.6M3.4 4l3.6 3.6M4.3 3.1l3.6 3.6M6.1 5.8l5.6 5.6M1.5 12.5h6",
    ),
    (
        "leaf",
        "M2.5 11.5C2.5 5.5 6 2.5 11.5 2.5 11.5 8 8.5 11.5 2.5 11.5zM2.5 11.5L8 6",
    ),
    (
        "list",
        "M2 3.8l1.4 1.4L6 2.6M2 9l1.4 1.4L6 7.8M8 4.2h4.5M8 9.4h4.5",
    ),
    ("lock", "M3.5 6.5h7v5.5h-7zM5 6.5V4.8a2 2 0 0 1 4 0v1.7"),
    (
        "person",
        "M7 6.6a2.4 2.4 0 1 0 0-4.8 2.4 2.4 0 0 0 0 4.8zM2.4 12.6c0-2.6 2-4.2 4.6-4.2s4.6 1.6 4.6 4.2",
    ),
    ("play", "M4 2.5v9l7.5-4.5z"),
    (
        "robot",
        "M3.5 4.5h7a1.5 1.5 0 0 1 1.5 1.5v4.5a1.5 1.5 0 0 1-1.5 1.5h-7A1.5 1.5 0 0 1 2 10.5V6a1.5 1.5 0 0 1 1.5-1.5zM7 4.5V2M5.2 8.2h.01M8.8 8.2h.01",
    ),
    (
        "shield",
        "M7 1.5l5 2v3.6c0 2.9-2.1 4.7-5 5.4-2.9-.7-5-2.5-5-5.4V3.5z",
    ),
    ("stack", "M2 2.5h10v2.5H2zM2 6.25h10v2.5H2zM2 10h10v2H2z"),
    (
        "wrench",
        "M9.6 1.8a3 3 0 0 0-3.4 4.1L2 10.1 3.9 12l4.2-4.2a3 3 0 0 0 4.1-3.4L10.4 6.2 8.6 5.4 7.8 3.6z",
    ),
];

/// The path of an icon, none for a name specapp does not draw.
pub fn path(name: &str) -> Option<&'static str> {
    ICONS.iter().find(|(n, _)| *n == name).map(|(_, p)| *p)
}
