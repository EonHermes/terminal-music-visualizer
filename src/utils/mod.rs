//! Utility functions and helpers
//! 
//! Common utilities used across the application.

pub mod math;
pub mod time;

/// Clamp a value between min and max
#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Map a value from one range to another
#[inline]
pub fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

/// Linear interpolation between two values
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * clamp(t, 0.0, 1.0)
}

/// Smoothstep interpolation
#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-1.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_map_range() {
        // Map 0.5 from [0, 1] to [0, 100]
        assert!((map_range(0.5, 0.0, 1.0, 0.0, 100.0) - 50.0).abs() < 1e-6);
        
        // Map 0.25 from [0, 1] to [-10, 10]
        assert!((map_range(0.25, 0.0, 1.0, -10.0, 10.0) - (-5.0)).abs() < 1e-6);
    }

    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 10.0, 0.0) - 0.0).abs() < 1e-6);
        assert!((lerp(0.0, 10.0, 1.0) - 10.0).abs() < 1e-6);
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_smoothstep() {
        // Edge cases
        assert!((smoothstep(0.0, 1.0, -0.5) - 0.0).abs() < 1e-6);
        assert!((smoothstep(0.0, 1.0, 1.5) - 1.0).abs() < 1e-6);
        
        // Middle point
        assert!((smoothstep(0.0, 1.0, 0.5) - 0.5).abs() < 1e-2); // Smoothstep at 0.5 is 0.5
    }
}
