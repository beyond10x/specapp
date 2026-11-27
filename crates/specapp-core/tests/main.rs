//! The one integration-test binary of specapp-core. Every test reads the fixture packs under
//! `fixtures/packs/` or a pack it writes under the target directory. No test names a word of a
//! pack: expectations come from `fixtures/expected/<pack>.json`.

mod cases;
