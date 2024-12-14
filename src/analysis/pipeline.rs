use crate::ai_enhanced::{AIAnalysisResult, AIProvider, CodeLLamaProvider};
use crate::analysis::semantic::{Analyzer, Context};
use crate::DevFlowError;
use crossbeam::channel::Receiver;
use dashmap::DashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use tokio::runtime::{Builder, Runtime};

pub type Result<T> = std::result::Result<T, DevFlowError>;

/// Main pipeline for analyzing code files
#[derive(Debug)]
pub struct Pipeline {
    cache: Arc<DashMap<PathBuf, AnalysisResult>>,
    stats: Arc<RwLock<Stats>>,
    ai_provider: Arc<Box<dyn AIProvider>>,
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub file_path: PathBuf,
    pub semantic_context: Context,
    pub ai_insights: Option<AIAnalysisResult>,
}

/// Statistics tracking for the analysis process
#[derive(Debug, Default)]
pub struct Stats {
    pub files_processed: AtomicUsize,
    pub total_files: AtomicUsize,
    pub processing_time: u128,
    pub errors: AtomicUsize,
}

impl Clone for Stats {
    fn clone(&self) -> Self {
        Self {
            files_processed: AtomicUsize::new(self.files_processed.load(Ordering::SeqCst)),
            total_files: AtomicUsize::new(self.total_files.load(Ordering::SeqCst)),
            processing_time: self.processing_time,
            errors: AtomicUsize::new(self.errors.load(Ordering::SeqCst)),
        }
    }
}

impl Stats {
    /// Creates a new stats instance
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment_files_processed(&self) {
        self.files_processed.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_errors(&self) {
        self.errors.fetch_add(1, Ordering::SeqCst);
    }

    #[must_use]
    pub fn get_files_processed(&self) -> usize {
        self.files_processed.load(Ordering::SeqCst)
    }

    #[must_use]
    pub fn get_errors(&self) -> usize {
        self.errors.load(Ordering::SeqCst)
    }
}

impl Pipeline {
    /// Creates a new analysis pipeline with default configuration.
    ///
    /// # Panics
    ///
    /// This function will panic if the `TOGETHER_API_KEY` environment variable is not set.
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(Stats::default())),
            ai_provider: Arc::new(Box::new(CodeLLamaProvider::new(
                &std::env::var("TOGETHER_API_KEY")
                    .expect("TOGETHER_API_KEY environment variable must be set"),
                "https://api.together.xyz/v1",
                "codellama/CodeLlama-34b-Instruct-hf",
                10,
            ))),
        }
    }

    /// Creates a new pipeline with a custom AI provider
    #[must_use]
    pub fn new_with_provider(provider: Box<dyn AIProvider>) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(Stats::default())),
            ai_provider: Arc::new(provider),
        }
    }

    /// Starts workers for processing files
    pub fn start_workers(&self, num_workers: usize, receiver: &Receiver<PathBuf>) {
        for id in 0..num_workers {
            let worker = Worker::new(
                id,
                Arc::clone(&self.cache),
                Arc::clone(&self.stats),
                Arc::clone(&self.ai_provider),
                receiver.clone(),
            );
            worker.start();
        }
    }

    /// Checks if a file has been processed
    #[must_use]
    pub fn is_file_processed(&self, path: &PathBuf) -> bool {
        self.cache.contains_key(path)
    }

    /// Retrieves analysis result for a processed file
    #[must_use]
    pub fn get_analysis_result(&self, path: &PathBuf) -> Option<AnalysisResult> {
        self.cache.get(path).map(|r| r.clone())
    }

    /// Retrieves pipeline statistics
    ///
    /// # Panics
    /// Panics if the stats lock is poisoned
    #[must_use]
    pub fn get_stats(&self) -> Stats {
        self.stats.read().expect("Failed to read stats").clone()
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    cache: Arc<DashMap<PathBuf, AnalysisResult>>,
    stats: Arc<RwLock<Stats>>,
    ai_provider: Arc<Box<dyn AIProvider>>,
    receiver: Receiver<PathBuf>,
    semantic_analyzer: Arc<Mutex<Analyzer>>,
}

impl Worker {
    fn new(
        id: usize,
        cache: Arc<DashMap<PathBuf, AnalysisResult>>,
        stats: Arc<RwLock<Stats>>,
        ai_provider: Arc<Box<dyn AIProvider>>,
        receiver: Receiver<PathBuf>,
    ) -> Self {
        Self {
            id,
            receiver,
            cache,
            stats,
            ai_provider,
            semantic_analyzer: Arc::new(Mutex::new(Analyzer::default())),
        }
    }

    fn start(self) {
        std::thread::spawn(move || {
            let runtime = Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create runtime");

            while let Ok(path) = self.receiver.recv() {
                if self.cache.contains_key(&path) {
                    log::debug!(
                        "Worker {} skipping already processed file {:?}",
                        self.id,
                        path
                    );
                    continue;
                }

                match self.process_file(&path, &runtime) {
                    Ok(result) => {
                        log::debug!("Worker {} successfully processed {:?}", self.id, path);
                        self.cache.insert(path.clone(), result);
                        if let Ok(stats) = self.stats.write() {
                            stats.increment_files_processed();
                        }
                    }
                    Err(e) => {
                        log::error!("Worker {} failed to process {:?}: {}", self.id, path, e);
                        if let Ok(stats) = self.stats.write() {
                            stats.increment_errors();
                            stats.increment_files_processed();
                        }
                        self.cache.insert(
                            path.clone(),
                            AnalysisResult {
                                file_path: path.clone(),
                                semantic_context: Context::default(),
                                ai_insights: None,
                            },
                        );
                    }
                }
            }
            log::debug!("Worker {} shutting down", self.id);
        });
    }

    fn process_file(&self, path: &PathBuf, runtime: &Runtime) -> Result<AnalysisResult> {
        let content = fs::read_to_string(path)?;

        let semantic_context = self.semantic_analyzer.lock().unwrap().analyze_file(path)?;

        let ai_insights =
            runtime.block_on(async { self.ai_provider.analyze_code(&content).await })?;

        let semantic_context = self
            .semantic_analyzer
            .lock()
            .unwrap()
            .merge_with_ai_analysis(semantic_context, &ai_insights);

        Ok(AnalysisResult {
            file_path: path.clone(),
            semantic_context,
            ai_insights: Some(ai_insights),
        })
    }
}
