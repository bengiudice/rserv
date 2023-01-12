use std::thread;
pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}
pub struct PoolCreationError;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            // create threads here, store in vec.
        }
        ThreadPool { threads }
    }
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {
            return Err(PoolCreationError);
        }
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            // create threads here, store in vec.
        }
        Ok(ThreadPool { threads })
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
