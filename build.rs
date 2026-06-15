use std::{
    env, fs,
    path::{Path, PathBuf},
};
use xdgen::{App, Context, FluentString};

fn main() {
    let id = "io.github.big-ol-pants.CosmicExtQuakeTerm";
    let domain = "cosmic_ext_quake_term";
    println!("cargo:rerun-if-changed=res/{id}.desktop");
    println!("cargo:rerun-if-changed=res/{id}.metainfo.xml");
    let ctx_dir = prepare_i18n_context(domain);
    let ctx = Context::new(ctx_dir, domain)
        .expect("failed to load i18n resources for cosmic_ext_quake_term");
    let app = App::new(FluentString("cosmic-ext-quake-term"))
        .comment(FluentString("comment"))
        .keywords(FluentString("keywords"));
    let output = PathBuf::from("target/xdgen");
    fs::create_dir_all(&output).unwrap();
    fs::write(
        output.join(format!("{}.desktop", id)),
        app.expand_desktop(format!("res/{}.desktop", id), &ctx)
            .unwrap(),
    )
    .unwrap();
    fs::write(
        output.join(format!("{}.metainfo.xml", id)),
        app.expand_metainfo(format!("res/{}.metainfo.xml", id), &ctx)
            .unwrap(),
    )
    .unwrap();
}

fn prepare_i18n_context(domain: &str) -> PathBuf {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"))
        .join("xdgen-i18n");
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out).unwrap();

    let filename = format!("{domain}.ftl");
    for entry in fs::read_dir("i18n").unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_dir() {
            continue;
        }

        let source = entry.path().join(&filename);
        if !source.exists() {
            continue;
        }

        let lang_dir = out.join(entry.file_name());
        fs::create_dir_all(&lang_dir).unwrap();
        fs::copy(&source, lang_dir.join(&filename)).unwrap();
        println!("cargo:rerun-if-changed={}", source.display());
    }

    if !has_language_dir(&out) {
        panic!("no i18n resources found for {domain}");
    }

    out
}

fn has_language_dir(path: &Path) -> bool {
    fs::read_dir(path)
        .unwrap()
        .any(|entry| entry.unwrap().file_type().unwrap().is_dir())
}
