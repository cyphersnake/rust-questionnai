#![allow(dead_code)]

mod bytecode_interpret {
    use std::collections::HashMap;

    type Val = u32;

    #[derive(Debug)]
    enum ByteCode {
        LoadVal(Val),
        WriteVar(&'static str),
        ReadVar(&'static str),
        Add,
        Multiply,
        ReturnValue,
        CmpEq,
        JumpIfFalse(usize),
        Goto(usize),
    }

    struct Interpreter {
        stack: Vec<Val>,
        vars: HashMap<String, Val>,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Error {
        LoadFromEmptyStack,
        UnknownVarName,
    }

    impl Interpreter {
        fn new() -> Self {
            Interpreter { stack: Vec::new(), vars: HashMap::new() }
        }

        fn execute(&mut self, bytecode: &[ByteCode]) -> Result<Option<Val>, Error> {
            let mut pc = 0;

            while pc < bytecode.len() {
                pc = match bytecode[pc] {
                    ByteCode::LoadVal(val) => {
                        self.stack.push(val);
                        pc + 1
                    },
                    ByteCode::WriteVar(varname) => {
                        self.vars.insert(
                            varname.to_owned(),
                            self.stack.pop().ok_or(Error::LoadFromEmptyStack)?,
                        );
                        pc + 1
                    },
                    ByteCode::ReadVar(var) => {
                        self.stack.push(*self.vars.get(var).ok_or(Error::UnknownVarName)?);
                        pc + 1
                    },
                    ByteCode::Add => {
                        let a = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                        let b = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                        self.stack.push(a + b);
                        pc + 1
                    },
                    ByteCode::Multiply => {
                        let a = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                        let b = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                        self.stack.push(a * b);
                        pc + 1
                    },
                    ByteCode::ReturnValue =>
                        return self.stack.pop().map(Some).ok_or(Error::LoadFromEmptyStack),
                    ByteCode::CmpEq => {
                        let a = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                        let b = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                        self.stack.push(if a.eq(&b) { 1 } else { 0 });
                        pc + 1
                    },
                    ByteCode::JumpIfFalse(pos) => {
                        if self.stack.pop().ok_or(Error::LoadFromEmptyStack)? == 0 {
                            pos
                        } else {
                            pc + 1
                        }
                    },
                    ByteCode::Goto(pos) => pos,
                }
            }

            Ok(None)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{ByteCode, Error, Interpreter};

        #[test]
        fn from_task() {
            assert_eq!(
                Interpreter::new().execute(&[
                    ByteCode::LoadVal(1),
                    ByteCode::WriteVar("x"),
                    ByteCode::LoadVal(2),
                    ByteCode::WriteVar("y"),
                    ByteCode::ReadVar("x"),
                    ByteCode::LoadVal(1),
                    ByteCode::Add,
                    ByteCode::ReadVar("y"),
                    ByteCode::Multiply,
                    ByteCode::ReturnValue,
                ]),
                Ok(Some(4))
            );
        }
        #[test]
        fn simple_add() {
            assert_eq!(
                Interpreter::new().execute(&[
                    ByteCode::LoadVal(2),
                    ByteCode::LoadVal(3),
                    ByteCode::Add,
                    ByteCode::ReturnValue,
                ]),
                Ok(Some(5))
            );
        }

        #[test]
        fn missing_var() {
            assert_eq!(
                Interpreter::new().execute(&[ByteCode::ReadVar("x"), ByteCode::ReturnValue]),
                Err(Error::UnknownVarName)
            );
        }

        #[test]
        fn simple_loop() {
            assert_eq!(
                Interpreter::new().execute(&[
                    // i = 0
                    ByteCode::LoadVal(0),
                    ByteCode::WriteVar("i"),
                    // i = i + 1
                    ByteCode::ReadVar("i"),
                    ByteCode::LoadVal(1),
                    ByteCode::Add,
                    ByteCode::WriteVar("i"),
                    // while i != 10
                    ByteCode::ReadVar("i"),
                    ByteCode::LoadVal(10),
                    ByteCode::CmpEq,
                    ByteCode::JumpIfFalse(2),
                ]),
                Ok(None)
            );
        }
    }
}

pub mod line_counter {
    use memchr::memchr_iter;
    use std::{
        ffi::OsStr,
        fs::File,
        io::{self, BufRead, BufReader, Read},
        path::Path,
    };

    use log::error;
    use pariter::IteratorExt as _;
    use walkdir::WalkDir;

    fn naive_count_lines(file_path: impl AsRef<Path>) -> io::Result<usize> {
        Ok(BufReader::new(File::open(file_path)?).lines().count())
    }

    /// Get count of lines in file
    ///
    /// Reads the file in 8-kilobyte chunks and searches for all `\n` characters using `memchr`.
    fn count_lines(file_path: impl AsRef<Path>) -> io::Result<usize> {
        let mut reader = BufReader::new(File::open(file_path)?);

        let mut count = 0;
        let mut buffer = [0; 8192];

        loop {
            match reader.read(&mut buffer)? {
                0 => break, // EOF
                n => {
                    count += memchr_iter(b'\n', &buffer[..n]).count();
                },
            }
        }

        Ok(count)
    }

    pub fn print_line_counts_in_dir(dir: impl AsRef<Path>, extension: &str) -> io::Result<()> {
        let extension = extension.to_string();

        WalkDir::new(dir)
            .into_iter()
            .parallel_map(move |entry| {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(err) => {
                        error!("Error while open entry: {err:?}, ignore");
                        return Ok(None)
                    },
                };

                if !entry.file_type().is_file() {
                    return Ok(None)
                }

                let path = entry.path();

                if path.extension().and_then(OsStr::to_str).ne(&Some(&extension)) {
                    return Ok(None)
                }

                Ok(Some((path.display().to_string(), count_lines(path)?)))
            })
            .try_for_each(|result_with_option: io::Result<_>| {
                if let Some((path, count)) = result_with_option? {
                    println!("{path}: {count} lines");
                }
                io::Result::Ok(())
            })
    }
}

fn main() {
    line_counter::print_line_counts_in_dir(".", "rs").unwrap();
}
