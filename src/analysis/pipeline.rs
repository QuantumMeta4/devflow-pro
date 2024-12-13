use crate::analysis::semantic::{SemanticAnalyzer, SemanticContext};
use crate::ai_enhanced::{AIAnalysisResult, AIProvider};
use crate::DevFlowError;
use dashmap::DashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;
use std::thread;
use crossbeam::channel::Receiver;
use tokio::runtime::Runtime;

pub type Result<T> = std::result::Result<T, DevFlowError>;

#[derive(Debug)]
pub struct AnalysisPipeline {
    semantic_analyzer: Arc<SemanticAnalyzer>,
    ai_provider: Arc<dyn AIProvider>,
    cache: Arc<DashMap<PathBuf, AnalysisResult>>,
    stats: Arc<RwLock<PipelineStats>>,
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub file_path: PathBuf,
    pub semantic_context: SemanticContext,
    pub ai_insights: Option<AIAnalysisResult>,
}

#[derive(Debug, Default)]
pub struct PipelineStats {
    pub files_processed: usize,
    pub total_files: usize,
    pub errors: usize,
}

impl AnalysisPipeline {
    pub fn new(
        semantic_analyzer: Arc<SemanticAnalyzer>,
        ai_provider: Arc<dyn AIProvider>,
    ) -> Self {
        Self {
            semantic_analyzer,
            ai_provider,
            cache: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(PipelineStats::default())),
        }
    }

    pub fn start_workers(&self, num_workers: usize, receiver: Receiver<PathBuf>) {
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
                    if let Ok(mut stats) = self.stats.write() {
                        stats.files_processed += 1;
                        stats.total_files += 1;
                    }
                }
                Err(e) => {
                    log::error!("Worker {} failed to process {:?}: {}", self.id, path, e);
                    if let Ok(mut stats) = self.stats.write() {
                        stats.errors += 1;
                    }
                }
            }
        }
    }

    fn process_file(&self, path: &PathBuf, runtime: &Runtime) -> Result<AnalysisResult> {
        let content = fs::read_to_string(path)?;
        
        // Perform semantic analysis
        let semantic_context = self.semantic_analyzer.analyze_file(path, &content)
            .map_err(|e| DevFlowError::Semantic(e))?;
        
        // Perform AI analysis
        let ai_insights = runtime.block_on(self.ai_provider.analyze_code(&content))
            .map_err(|e| DevFlowError::AI(e.to_string()))?;

        Ok(AnalysisResult {
            file_path: path.clone(),
            semantic_context,
            ai_insights: Some(ai_insights),
        })
    }
}
