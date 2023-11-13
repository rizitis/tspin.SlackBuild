use crate::io::writer::AsyncLineWriter;
use async_trait::async_trait;
use tokio::io;

pub struct StdoutWriter {}

impl StdoutWriter {
    pub fn init() -> Box<dyn AsyncLineWriter + Send> {
        Box::new(StdoutWriter {})
    }
}

#[async_trait]
impl AsyncLineWriter for StdoutWriter {
    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        println!("{}", line);

        Ok(())
    }
}
