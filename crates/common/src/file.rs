use super::*;
use std::fmt::Debug;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    pub i: u32,
    pub j: u32,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}
impl Range {
    pub fn combine(a: &Range, b: &Range) -> Range {
        Range {
            start: a.start,
            end: b.end,
        }
    }
    pub fn none() -> Range {
        Range {
            start: Position { i: 0, j: 0 },
            end: Position { i: 0, j: 0 },
        }
    }
}
impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}:{}, {}:{})",
            self.start.i, self.start.j, self.end.i, self.end.j,
        ))
    }
}

#[derive(PartialEq, Debug)]
pub enum Indent {
    None,
    Indent(usize),
}
impl Indent {
    fn from_line(line: &str) -> Indent {
        let mut white_spaces: usize = 0;
        let mut iter = line.chars();
        loop {
            match iter.next() {
                Some(c) => {
                    if c == ' ' {
                        white_spaces += 1;
                    } else {
                        break Indent::Indent(white_spaces);
                    }
                }
                None => break Indent::None,
            }
        }
    }
}

#[derive(Debug)]
pub struct IndentedLine {
    pub indent: Indent,
    pub line: Vec<char>,
}

pub struct HuskyFile {
    pub filepath: PathBuf,
    pub lines: Vec<String>,
    pub indents: Vec<Indent>,
}
impl HuskyFile {
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    pub fn new(filepath: PathBuf) -> Result<HuskyFile, ParserError> {
        let mut source = HuskyFile {
            filepath,
            lines: vec![],
            indents: vec![],
        };
        source.load()?;
        Ok(source)
    }
    fn load(&mut self) -> Result<(), ParserError> {
        match Self::read_lines(&self.filepath) {
            Ok(lines) => {
                self.indents.clear();
                self.lines.clear();
                for line in lines {
                    if let Ok(s) = line {
                        let indent = Indent::from_line(&s);
                        self.indents.push(indent);
                        self.lines.push(s);
                    }
                }
                Ok(())
            }
            Err(e) => io_error!(e, "IOError".to_string()),
        }
    }
}

pub struct FileTree {
    pub key: String,
    pub source: HuskyFile,
    pub children: Vec<FileTree>,
}
impl FileTree {
    pub fn new_package(folderpath: PathBuf) -> Result<FileTree, ParserError> {
        let package_name = folderpath
            .file_name()
            .expect("package name")
            .to_str()
            .expect("valid package name")
            .to_string();
        assert!(folderpath.is_dir());
        let mut mainpath = folderpath;
        mainpath.push("main.hsk");
        assert!(mainpath.is_file());
        let mut sources = FileTree {
            key: package_name,
            source: HuskyFile::new(mainpath)?,
            children: vec![],
        };
        sources.load();
        Ok(sources)
    }
    fn new_module(path: PathBuf) -> Option<FileTree> {
        let mut module_path = path;
        if module_path.is_dir() {
            let module_name = module_path
                .file_name()
                .expect("package name")
                .to_str()
                .expect("valid package name")
                .to_string();
            module_path.push("mod.hsk");
            if module_path.is_file() {
                let mut sources = FileTree {
                    key: module_name,
                    source: HuskyFile::new(module_path).expect("success"),
                    children: vec![],
                };
                sources.load();
                Some(sources)
            } else {
                None
            }
        } else {
            if let Some(extension) = module_path.extension() {
                if extension == "hsk" && module_path.is_file() {
                    let module_name = module_path
                        .file_stem()
                        .expect("package name")
                        .to_str()
                        .expect("valid package name")
                        .to_string();
                    let mut sources = FileTree {
                        key: module_name,
                        source: HuskyFile::new(module_path).expect("success"),
                        children: vec![],
                    };
                    sources.load();
                    Some(sources)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
    pub fn load(&mut self) {
        self.source.load().expect("success");
        self.children.clear();
        if self.source.filepath.ends_with("mod.hsk") || self.source.filepath.ends_with("main.hsk") {
            let folderpath = self.source.filepath.parent().expect("none root");
            for entry in std::fs::read_dir(folderpath).expect("success") {
                let path = entry.expect("success").path();
                if !path.ends_with("mod.hsk") && !path.ends_with("main.hsk") {
                    if let Some(sources) = FileTree::new_module(path) {
                        self.children.push(sources);
                    }
                }
            }
        }
    }
    // pub fn get_child(&self, key: &String) -> &FileTree {
    //     td!()
    // }
}

fn visit_dirs(dir: &Path, cb: &mut dyn RtMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            cb(&entry);
            visit_dirs(&path, cb)?;
        }
    }
    Ok(())
}

const TESTS_ROOT: &str = "/home/xiyuzhai/Documents/husky/tests";

pub fn find_husky_tests_source_filepaths(folder: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut filepaths: Vec<PathBuf> = vec![];
    let mut add_to_source_filepaths = |entry: &DirEntry| {
        if let Some(extension) = entry.path().extension() {
            if extension == "hsk" {
                filepaths.push(entry.path())
            }
        }
    };
    visit_dirs(
        &Path::new(&(TESTS_ROOT.to_string() + "/" + folder)),
        &mut add_to_source_filepaths,
    )?;
    Ok(filepaths)
}

pub fn find_husky_tests_package_folders(folder: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut filepaths: Vec<PathBuf> = vec![];
    let mut add_to_source_filepaths = |entry: &DirEntry| {
        if entry.path().is_dir() {
            let mut main_path = entry.path().clone();
            main_path.push("main.hsk");
            if main_path.is_file() {
                filepaths.push(entry.path())
            }
        }
    };
    visit_dirs(
        &Path::new(&(TESTS_ROOT.to_string() + "/" + folder)),
        &mut add_to_source_filepaths,
    )?;
    Ok(filepaths)
}
