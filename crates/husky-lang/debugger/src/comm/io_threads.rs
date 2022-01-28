pub struct IoThreads {
    reader: std::thread::JoinHandle<std::io::Result<()>>,
    writer: std::thread::JoinHandle<std::io::Result<()>>,
}

impl IoThreads {
    // Creates an IoThreads
    pub(crate) fn new(
        reader: std::thread::JoinHandle<std::io::Result<()>>,
        writer: std::thread::JoinHandle<std::io::Result<()>>,
    ) -> IoThreads {
        IoThreads { reader, writer }
    }

    pub fn join(self) -> std::io::Result<()> {
        match self.reader.join() {
            Ok(r) => r?,
            Err(err) => {
                println!("reader panicked!");
                std::panic::panic_any(err)
            }
        }
        match self.writer.join() {
            Ok(r) => r,
            Err(err) => {
                println!("reader panicked!");
                std::panic::panic_any(err);
            }
        }
    }
}
