//! The packs specapp knows: the manifests of public packs under specapp's `packs/`, embedded
//! when specapp is built. A pack is then named instead of its manifest's path
//! (`specapp pack check <name> --pack <dir>`), and `extends` resolves a name. The manifest of a
//! private pack is never here; it is given by its path at run time.

include!(concat!(env!("OUT_DIR"), "/known.rs"));

/// The YAML text of the manifest of a known pack.
pub fn manifest(name: &str) -> Option<&'static str> {
    KNOWN
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, text)| *text)
}

/// The names of the known packs, sorted.
pub fn names() -> Vec<&'static str> {
    KNOWN.iter().map(|(name, _)| *name).collect()
}
