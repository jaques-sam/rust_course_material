#![allow(unused)]

use async_trait::async_trait;

fn main() {
}

struct S();

#[async_trait]
trait Streamer: 'static {
    async fn start(&mut self) -> Result<(), String>;
}

#[async_trait]
impl Streamer for S {
    async fn start(&mut self) -> Result<(), String> {
        Ok(())
    }
}

struct Streamers(Vec<Box<dyn Streamer>>);

impl Streamers {
    fn add(&mut self, streamer: impl Streamer) {
        self.0.push(Box::new(streamer));
    }

    async fn start(&mut self, id: &str) -> Result<(), String> {
        // logic which starts a particular streamer
        Err("cannot find id".to_string())
    }
}

#[cfg(test)]
mod test{
    use uuid::Uuid;
    use super::*;

    struct MockStreamer();

    #[async_trait]
    impl Streamer for MockStreamer {
        async fn start(&mut self) -> Result<(), String> {
            panic!("Should not be called");
        }
    }

    #[tokio::test]
    async fn starting_streamer_that_does_not_exist_should_fail() {
        let streamer = MockStreamer();

        let mut streamers = Streamers(vec![]);
        streamers.add(streamer);

        assert!(streamers.start("id").await.is_err());
    }
}
