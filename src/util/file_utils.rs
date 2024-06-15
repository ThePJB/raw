// todo make some nice ones for file to bytes,err etc. just for convenience one linings
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub fn dir_traverse(path: &Path, f: &mut impl FnMut(&Path)) -> Result<(), std::io::Error> {
    let entries = read_dir(path)?.into_iter();
    for entry in entries {
        let entry = entry?;
        let is_dir = entry.file_type()?.is_dir();
        if is_dir {
            dir_traverse(&entry.path(), f)?;
        } else {
            f(&entry.path());
        }
    }
    Ok(())
}

#[test]
fn test_dir_traverse() {
    dir_traverse(Path::new("data"), &mut |p| println!("{:?}", p));
    // this is lit
    // load assets
    // load preprocs
    // etc
}

pub fn my_read_dir(path: &Path) -> std::io::Result<Vec<MyDirEntry>> {
    read_dir(path).map(|x| {
        x.into_iter()
            .filter_map(|x| x.ok())
            .filter_map(|x| {
                let stem = match x
                    .path()
                    .file_stem()
                    .map(|x| x.to_str().map(|x| x.to_owned()))
                {
                    Some(Some(s)) => s,
                    _ => "".to_owned(),
                };
                let extension = match x
                    .path()
                    .extension()
                    .map(|x| x.to_str().map(|x| x.to_owned()))
                {
                    Some(Some(s)) => s,
                    _ => "".to_owned(),
                };

                Some(MyDirEntry {
                    path: x.path(),
                    is_dir: x.file_type().ok()?.is_dir(),
                    stem,
                    extension,
                })
            })
            .collect()
    })
}

#[derive(Debug, Clone)]
pub struct MyDirEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub stem: String,
    pub extension: String,
}
