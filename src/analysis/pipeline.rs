use crate::ai_enhanced::{AIAnalysisResult, AIProvider, CodeLLamaProvider};
use crate::analysis::semantic::{SemanticAnalyzer, SemanticContext};
use crate::DevFlowError;
use crossbeam::channel::Receiver;
use dashmap::DashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use tokio::runtime::{Builder, Runtime};

pub type Result<T> = std::result::Result<T, DevFlowError>;

#[derive(Debug)]
pub struct AnalysisPipeline {
    cache: Arc<DashMap<PathBuf, AnalysisResult>>,
    stats: Arc<RwLock<PipelineStats>>,
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub file_path: PathBuf,
    pub semantic_context: SemanticContext,
    pub ai_insights: Option<AIAnalysisResult>,
}

#[derive(Debug, Default)]
pub struct PipelineStats {
    pub files_processed: AtomicUsize,
    pub total_files: AtomicUsize,
    pub processing_time: u128,
    pub errors: AtomicUsize,
}

impl PipelineStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment_files_processed(&self) {
        self.files_processed.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_errors(&self) {
        self.errors.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_files_processed(&self) -> usize {
        self.files_processed.load(Ordering::SeqCst)
    }

    pub fn get_total_files(&self) -> usize {
        self.total_files.load(Ordering::SeqCst)
    }

    pub fn get_errors(&self) -> usize {
        self.errors.load(Ordering::SeqCst)
    }
}

impl AnalysisPipeline {
    /// Creates a new analysis pipeline
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(PipelineStats::default())),
        }
    }

    /// Starts workers for processing files
    pub fn start_workers(&self, num_workers: usize, receiver: &Receiver<PathBuf>) {
        // Start the workers
        for id in 0..num_workers {
            let worker = Worker::new(
                id,
                Arc::clone(&self.cache),
                Arc::clone(&self.stats),
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
    #[must_use]
    pub fn get_stats(&self) -> PipelineStats {
        self.stats
            .read()
            .map(|stats| PipelineStats {
                files_processed: AtomicUsize::new(stats.get_files_processed()),
                total_files: AtomicUsize::new(stats.get_total_files()),
                processing_time: stats.processing_time,
                errors: AtomicUsize::new(stats.get_errors()),
            })
            .unwrap_or_default()
    }
}

impl Default for AnalysisPipeline {
    fn default() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(PipelineStats::default())),
        }
    }
}

#[derive(Debug, Clone)]
struct Worker {
    id: usize,
    cache: Arc<DashMap<PathBuf, AnalysisResult>>,
    stats: Arc<RwLock<PipelineStats>>,
    receiver: Receiver<PathBuf>,
    semantic_analyzer: Arc<SemanticAnalyzer>,
}

impl Worker {
    fn new(
        id: usize,
        cache: Arc<DashMap<PathBuf, AnalysisResult>>,
        stats: Arc<RwLock<PipelineStats>>,
        receiver: Receiver<PathBuf>,
    ) -> Self {
        Self {
            id,
            receiver,
            cache,
            stats,
            semantic_analyzer: Arc::new(SemanticAnalyzer::new()),
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
                                semantic_context: SemanticContext::default(),
                                ai_insights: None,
                            },
                        );
                    }
                }
            }
            log::debug!("Worker {} shutting down", self.id);
        });
    }

    fn process_file(
        &self,
        path: &PathBuf,
        runtime: &Runtime,
    ) -> std::result::Result<AnalysisResult, DevFlowError> {
        log::debug!("Worker {} starting to process file {:?}", self.id, path);

        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                log::error!("Failed to read file {:?}: {}", path, e);
                return Err(DevFlowError::Io(e));
            }
        };

        // Semantic analysis
        let semantic_context = self
            .semantic_analyzer
            .analyze_file(path, &content)
            .map_err(DevFlowError::Semantic)?;

        // AI-enhanced analysis
        let ai_insights = runtime.block_on(async {
            let provider = CodeLLamaProvider::new(
                "default_api_key",
                "https://api.together.xyz/v1",
                "codellama/CodeLlama-34b-Instruct-hf",
                10,
            );
            provider.analyze_code(&content).await
        })?;

        // Merge semantic and AI analysis
        let semantic_context = self
            .semantic_analyzer
            .merge_with_ai_analysis(&semantic_context, &ai_insights);

        // Create analysis result
        let result = AnalysisResult {
            file_path: path.clone(),
            semantic_context,
            ai_insights: Some(ai_insights),
        };

        log::debug!("Worker {} successfully processed file {:?}", self.id, path);
        Ok(result)
    }
}
