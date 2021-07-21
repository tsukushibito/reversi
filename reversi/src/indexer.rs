pub struct Indexer {
    pub indices: [usize; 38],
}

impl Indexer {
    pub fn new() -> Indexer {
        let mut indices = [0; 38];

        Indexer { indices: indices }
    }
}
