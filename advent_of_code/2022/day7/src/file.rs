use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct File<'a> {
    name: &'a str,
    size: Option<u32>,
    directory: Option<BTreeSet<File<'a>>>,
}

impl<'a> File<'a> {
    pub fn new_file(filename: &'a str, size: u32) -> Self {
        File {
            name: filename,
            size: Some(size),
            directory: None,
        }
    }

    pub fn new_dir(dirname: &'a str) -> Self {
        File {
            name: dirname,
            size: None,
            directory: Some(BTreeSet::new()),
        }
    }

    pub fn is_file(&self) -> bool {
        self.directory == None
    }

    pub fn is_dir(&self) -> bool {
        self.directory != None
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_size(&self) -> u32 {
        if self.is_file() {
            self.size.unwrap()
        } else {
            let mut sum: u32 = 0;
            for file in self.open_dir() {
                sum += file.get_size();
            }
            sum
        }
    }

    pub fn insert(&mut self, file: File<'a>) {
        self.directory.as_mut().unwrap().insert(file);
    }

    pub fn has_subdir(&self) -> bool {
        if self.is_file() {
            false
        } else {
            for file in self.open_dir() {
                if file.is_dir() {
                    return true;
                }
            }
            false
        }
    }

    pub fn open_dir(&self) -> &BTreeSet<File<'a>> {
        self.directory.as_ref().unwrap()
    }
}

struct DisplayFilesystem<'a> {
    root: &'a File<'a>,
    level: usize,
}

impl fmt::Display for DisplayFilesystem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.root.is_file() {
            write!(f, "{} {}\n", self.root.size.unwrap(), self.root.name)?;
        } else {
            write!(f, "dir {}\n", self.root.name)?;
            let current_level = self.level + 4;
            for file in self.root.directory.as_ref().unwrap() {
                write!(f, "{:indent$}", "", indent = current_level)?;
                write!(
                    f,
                    "{}",
                    DisplayFilesystem {
                        root: file,
                        level: current_level
                    }
                )?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for File<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            DisplayFilesystem {
                root: self,
                level: 0
            }
        )
    }
}
