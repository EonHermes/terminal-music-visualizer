//! Time-related utilities

use std::time::{Duration, Instant};

/// Simple frame rate tracker
pub struct FrameRateTracker {
    start: Instant,
    frames: u64,
    last_report: Instant,
    report_interval: Duration,
}

impl FrameRateTracker {
    /// Create a new FPS tracker with default 1 second report interval
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            frames: 0,
            last_report: Instant::now(),
            report_interval: Duration::from_secs(1),
        }
    }

    /// Create with custom report interval
    pub fn with_interval(interval: Duration) -> Self {
        Self {
            start: Instant::now(),
            frames: 0,
            last_report: Instant::now(),
            report_interval: interval,
        }
    }

    /// Record a frame
    pub fn record(&mut self) -> Option<f64> {
        self.frames += 1;
        
        let elapsed = self.last_report.elapsed();
        
        if elapsed >= self.report_interval {
            let fps = self.frames as f64 / elapsed.as_secs_f64();
            self.frames = 0;
            self.last_report = Instant::now();
            Some(fps)
        } else {
            None
        }
    }

    /// Get total runtime
    pub fn runtime(&self) -> Duration {
        self.start.elapsed()
    }

    /// Get current frame count
    pub fn frame_count(&self) -> u64 {
        self.frames
    }
}

impl Default for FrameRateTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Format duration as MM:SS
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let minutes = secs / 60;
    let seconds = secs % 60;
    
    format!("{:02}:{:02}", minutes, seconds)
}

/// Format duration as HH:MM:SS
pub fn format_duration_long(duration: Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format_duration(duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(0)), "00:00");
        assert_eq!(format_duration(Duration::from_secs(5)), "00:05");
        assert_eq!(format_duration(Duration::from_secs(60)), "01:00");
        assert_eq!(format_duration(Duration::from_secs(95)), "01:35");
        assert_eq!(format_duration(Duration::from_secs(3661)), "61:01");
    }

    #[test]
    fn test_format_duration_long() {
        assert_eq!(format_duration_long(Duration::from_secs(0)), "00:00");
        assert_eq!(format_duration_long(Duration::from_secs(59)), "00:59");
        assert_eq!(format_duration_long(Duration::from_secs(60)), "01:00");
        assert_eq!(format_duration_long(Duration::from_secs(3661)), "01:01:01");
    }

    #[test]
    fn test_fps_tracker_creation() {
        let tracker = FrameRateTracker::new();
        
        assert_eq!(tracker.frame_count(), 0);
        assert!(tracker.runtime().as_secs() < 2); // Just created
    }

    #[test]
    fn test_fps_tracker_with_interval() {
        let tracker = FrameRateTracker::with_interval(Duration::from_millis(100));
        
        assert_eq!(tracker.frame_count(), 0);
    }

    #[test]
    fn test_fps_tracker_record() {
        let mut tracker = FrameRateTracker::new();
        
        // Record some frames - should not report yet (less than 1 second)
        for _ in 0..10 {
            assert!(tracker.record().is_none());
        }
        
        assert_eq!(tracker.frame_count(), 10);
    }
}
