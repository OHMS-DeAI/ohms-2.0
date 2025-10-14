use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::Mutex;

#[derive(Clone)]
pub struct ProgressTracker {
    multi: Arc<MultiProgress>,
    bars: Arc<Mutex<Vec<ProgressBar>>>,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self {
            multi: Arc::new(MultiProgress::new()),
            bars: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_download_bar(&self, file_name: &str, total_bytes: u64) -> ProgressBar {
        let pb = self.multi.add(ProgressBar::new(total_bytes));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                .expect("valid template")
                .progress_chars("#>-"),
        );
        pb.set_message(format!("Downloading {}", file_name));
        self.bars.lock().push(pb.clone());
        pb
    }

    pub fn add_quantization_bar(&self, layer_name: &str) -> ProgressBar {
        let pb = self.multi.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .expect("valid template"),
        );
        pb.set_message(format!("Quantizing {}", layer_name));
        pb.enable_steady_tick(Duration::from_millis(100));
        self.bars.lock().push(pb.clone());
        pb
    }

    pub fn add_file_processing_bar(&self, file_name: &str, total_tensors: u64) -> ProgressBar {
        let pb = self.multi.add(ProgressBar::new(total_tensors));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} tensors")
                .expect("valid template")
                .progress_chars("#>-"),
        );
        pb.set_message(format!("Processing {}", file_name));
        self.bars.lock().push(pb.clone());
        pb
    }

    pub fn finish_all(&self) {
        let bars = self.bars.lock();
        for bar in bars.iter() {
            bar.finish();
        }
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct BandwidthMonitor {
    total_bytes: Arc<Mutex<u64>>,
    start_time: Instant,
}

impl BandwidthMonitor {
    pub fn new() -> Self {
        Self {
            total_bytes: Arc::new(Mutex::new(0)),
            start_time: Instant::now(),
        }
    }

    pub fn add_bytes(&self, bytes: u64) {
        let mut total = self.total_bytes.lock();
        *total += bytes;
    }

    pub fn average_bandwidth_mbps(&self) -> f64 {
        let total = *self.total_bytes.lock();
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            (total as f64 / 1_048_576.0) / elapsed
        } else {
            0.0
        }
    }

    pub fn total_bytes(&self) -> u64 {
        *self.total_bytes.lock()
    }
}

impl Default for BandwidthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

