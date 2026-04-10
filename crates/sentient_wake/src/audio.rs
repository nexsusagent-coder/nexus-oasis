//! ─── Audio Utilities ───


/// Resample audio to target sample rate
pub fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate {
        return samples.to_vec();
    }
    
    let ratio = to_rate as f64 / from_rate as f64;
    let new_len = (samples.len() as f64 * ratio) as usize;
    
    // Simple linear interpolation
    let mut output = Vec::with_capacity(new_len);
    
    for i in 0..new_len {
        let src_idx = i as f64 / ratio;
        let idx = src_idx as usize;
        let frac = src_idx - idx as f64;
        
        let sample = if idx + 1 < samples.len() {
            samples[idx] * (1.0 - frac as f32) + samples[idx + 1] * frac as f32
        } else {
            samples[idx.min(samples.len() - 1)]
        };
        
        output.push(sample);
    }
    
    output
}

/// Convert stereo to mono
pub fn stereo_to_mono(stereo: &[f32]) -> Vec<f32> {
    stereo
        .chunks(2)
        .map(|chunk| (chunk[0] + chunk.get(1).copied().unwrap_or(0.0)) / 2.0)
        .collect()
}

/// Normalize audio
pub fn normalize(samples: &[f32]) -> Vec<f32> {
    let max = samples
        .iter()
        .map(|s| s.abs())
        .fold(0.0f32, |a, b| a.max(b));
    
    if max == 0.0 {
        return samples.to_vec();
    }
    
    samples.iter().map(|s| s / max).collect()
}

/// Apply pre-emphasis filter
pub fn pre_emphasis(samples: &[f32], coeff: f32) -> Vec<f32> {
    let mut output = Vec::with_capacity(samples.len());
    
    output.push(samples[0]);
    for i in 1..samples.len() {
        output.push(samples[i] - coeff * samples[i - 1]);
    }
    
    output
}

/// Calculate MFCC features (for advanced wake word detection)
pub fn mfcc(samples: &[f32], num_coeffs: usize) -> Vec<Vec<f32>> {
    // Simplified MFCC implementation
    // For production, use a proper DSP library
    
    let frame_size = 512;
    let hop_size = 256;
    let num_frames = (samples.len() - frame_size) / hop_size + 1;
    
    let mut features = Vec::with_capacity(num_frames);
    
    for i in 0..num_frames {
        let start = i * hop_size;
        let end = (start + frame_size).min(samples.len());
        let frame = &samples[start..end];
        
        // Simplified: just use energy in frequency bands
        let mut coeffs = vec![0.0; num_coeffs];
        let band_size = frame.len() / num_coeffs;
        
        for (j, coeff) in coeffs.iter_mut().enumerate() {
            let band_start = j * band_size;
            let band_end = (band_start + band_size).min(frame.len());
            let band = &frame[band_start..band_end];
            
            *coeff = band.iter().map(|x| x * x).sum::<f32>() / band.len() as f32;
        }
        
        features.push(coeffs);
    }
    
    features
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resample() {
        let samples = vec![0.0, 0.5, 1.0, 0.5, 0.0];
        let resampled = resample(&samples, 16000, 8000);
        assert!(resampled.len() < samples.len());
    }
    
    #[test]
    fn test_stereo_to_mono() {
        let stereo = vec![0.5, 0.5, 1.0, 1.0];
        let mono = stereo_to_mono(&stereo);
        assert_eq!(mono, vec![0.5, 1.0]);
    }
    
    #[test]
    fn test_normalize() {
        let samples = vec![0.0, 0.5, 1.0, 0.5, 0.0];
        let normalized = normalize(&samples);
        assert!((normalized[2] - 1.0).abs() < 0.001);
    }
}
