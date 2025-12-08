use std::error::Error;

use vergen_git2::{Emitter, Git2Builder};

fn main() -> Result<(), Box<dyn Error>> {
    let git2 = Git2Builder::all_git()?;
    Emitter::default().add_instructions(&git2)?.emit()?;

    #[cfg(not(debug_assertions))]
    lazyjinja::embed_templates()?;

    Ok(())
}

/// Everything in the following mod has been "heavily inspired" by the upstream
/// implementation of minijinja_embed, which is licensed under the Apache
/// License 2.0, see https://github.com/mitsuhiko/minijinja
#[cfg(not(debug_assertions))]
mod lazyjinja {
    use std::{
        env,
        fs::{self, DirEntry, File},
        io::{self, Write},
        path::Path,
    };

    fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path
                    .file_name()
                    .and_then(|x| x.to_str())
                    .is_some_and(|x| x.starts_with('.'))
                {
                    continue;
                }
                if path.is_dir() {
                    visit_dirs(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }

    pub fn embed_templates() -> io::Result<()> {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let out_path = Path::new(&out_dir).join("lazyjinja_templates.rs");
        let src_path = Path::new("templates").canonicalize().unwrap();
        let src_path_ref = src_path.as_ref();

        let mut outfile = File::create(out_path)?;
        write!(outfile, "::std::collections::HashMap::from([")?;
        let _ = visit_dirs(src_path_ref, &mut |f| {
            let path = f.path();
            let name = path
                .strip_prefix(&src_path)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            let contents = fs::read_to_string(&path).unwrap();

            write!(outfile, "({name:?}, {contents:?}),").unwrap();
        });
        write!(outfile, "])")?;

        println!("cargo:rerun-if-changed=templates");
        Ok(())
    }
}
