use crate::ai_enhanced::{AIAnalysisResult, AIProvider};
use crate::analysis::semantic::{SemanticAnalyzer, SemanticContext};
use crate::DevFlowError;
use crossbeam::channel::Receiver;
use dashmap::DashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Instant;
use tokio::runtime::Runtime;

pub type Result<T> = std::result::Result<T, DevFlowError>;

#[derive(Debug)]
pub struct AnalysisPipeline {
    semantic_analyzer: Arc<SemanticAnalyzer>,
    ai_provider: Arc<dyn AIProvider>,
    cache: Arc<DashMap<PathBuf, AnalysisResult>>,
    stats: Arc<RwLock<PipelineStats>>,
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub file_path: PathBuf,
    pub semantic_context: SemanticContext,
    pub ai_insights: Option<AIAnalysisResult>,
}

#[derive(Debug, Default, Clone)]
pub struct PipelineStats {
    pub files_processed: usize,
    pub total_files: usize,
    pub errors: usize,
}

impl AnalysisPipeline {
    pub fn new(semantic_analyzer: Arc<SemanticAnalyzer>, ai_provider: Arc<dyn AIProvider>) -> Self {
        Self {
            semantic_analyzer,
            ai_provider,
            cache: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(PipelineStats::default())),
        }
    }

    pub fn start_workers(&self, num_workers: usize, receiver: Receiver<PathBuf>) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_files = num_workers;
        }
        for id in 0..num_workers {
            let worker = Worker::new(
                id,
                receiver.clone(),
                Arc::clone(&self.cache),
                Arc::clone(&self.stats),
                Arc::clone(&self.semantic_analyzer),
                Arc::clone(&self.ai_provider),
            );
            let _ = worker.start();
        }
    }

    pub fn is_file_processed(&self, path: &PathBuf) -> bool {
        self.cache.contains_key(path)
    }

    pub fn get_analysis_result(&self, path: &PathBuf) -> Option<AnalysisResult> {
        self.cache.get(path).map(|r| r.clone())
    }

    pub fn get_stats(&self) -> PipelineStats {
        self.stats.read().unwrap().clone()
    }
}

#[derive(Debug, Clone)]
struct Worker {
    id: usize,
    receiver: Receiver<PathBuf>,
    cache: Arc<DashMap<PathBuf, AnalysisResult>>,
    stats: Arc<RwLock<PipelineStats>>,
    semantic_analyzer: Arc<SemanticAnalyzer>,
    ai_provider: Arc<dyn AIProvider>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Receiver<PathBuf>,
        cache: Arc<DashMap<PathBuf, AnalysisResult>>,
        stats: Arc<RwLock<PipelineStats>>,
        semantic_analyzer: Arc<SemanticAnalyzer>,
        ai_provider: Arc<dyn AIProvider>,
    ) -> Self {
        Self {
            id,
            receiver,
            cache,
            stats,
            semantic_analyzer,
            ai_provider,
        }
    }

    fn start(self) -> std::thread::JoinHandle<()> {
        let id = self.id;
        thread::Builder::new()
            .name(format!("analysis-worker-{}", id))
            .spawn(move || self.run())
            .unwrap_or_else(|e| panic!("Failed to spawn worker {}: {}", id, e))
    }

    fn run(self) {
        // Create a new runtime for this worker
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create runtime");

        while let Ok(path) = self.receiver.recv() {
            let _start = Instant::now();

            match self.process_file(&path, &runtime) {
                Ok(result) => {
                    self.cache.insert(path.clone(), result);
                }
                Err(e) => {
                    log::error!("Worker {} failed to process {:?}: {}", self.id, path, e);
                    if let Ok(mut stats) = self.stats.write() {
                        stats.errors += 1;
                        // Still mark as processed even if there was an error
                        stats.files_processed += 1;
                    }
                    // Insert a failed result so we don't hang waiting
                    self.cache.insert(
                        path.clone(),
                        AnalysisResult {
                            file_path: path.clone(),
                            semantic_context: Default::default(),
                            ai_insights: None,
                        },
                    );
                }
            }
        }
    }

    fn process_file(&self, path: &PathBuf, runtime: &Runtime) -> Result<AnalysisResult> {
        log::debug!("Worker {} starting to process file {:?}", self.id, path);

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to read file {:?}: {}", path, e);
                return Err(DevFlowError::Io(e));
            }
        };

        // Perform semantic analysis
        log::debug!("Worker {} performing semantic analysis", self.id);
        let semantic_context = match self.semantic_analyzer.analyze_file(path, &content) {
            Ok(ctx) => ctx,
            Err(e) => {
                log::error!("Semantic analysis failed for {:?}: {}", path, e);
                return Err(DevFlowError::Semantic(e));
            }
        };

        // Perform AI analysis
        log::debug!("Worker {} performing AI analysis", self.id);
        let ai_insights = match runtime.block_on(self.ai_provider.analyze_code(&content)) {
            Ok(insights) => insights,
            Err(e) => {
                log::error!("AI analysis failed for {:?}: {}", path, e);
                return Err(DevFlowError::AI(e.to_string()));
            }
        };

        // Update stats before returning
        if let Ok(mut stats) = self.stats.write() {
            stats.files_processed += 1;
            log::debug!("Worker {} successfully processed file {:?}", self.id, path);
        }

        Ok(AnalysisResult {
            file_path: path.clone(),
            semantic_context,
            ai_insights: Some(ai_insights),
        })
    }
}
