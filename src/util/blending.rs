pub mod colors;
pub mod vec3;

// Thank you ChatGPT for the following mess I did not want to deal with

/// Samples `vector` at a continuous position given by `factor` in [0,1],
/// optionally smoothing with a Gaussian window around that position.
///
/// - `factor`: where to read along `vector` (0 = start, 1 = end).
/// - `window`: smoothing width. If `window <= 0`, does plain linear interpolation.
///             If `0 < window <= 1`, it's interpreted as a *fraction of the slice length*.
///             If `window > 1`, it's interpreted as an absolute width in *samples*.
///
/// Returns 0.0 for an empty input.
pub fn sample_windowed(vector: &[f32], factor: f32, window: f32) -> f32 {
    let n = vector.len();
    if n == 0 {
        return 0.0;
    }
    if n == 1 {
        return vector[0];
    }

    // Clamp factor and map to continuous index in [0, n-1]
    let f = factor.clamp(0.0, 1.0);
    let pos = f * ((n as f32) - 1.0);

    // Fast path: tiny/zero window => linear interpolation (best transient preservation)
    // We'll treat <= ~0.5 sample as "tiny".
    let win_samples = if window <= 1.0 {
        (window.max(0.0)) * (n as f32) // fraction of length
    } else {
        window // absolute samples
    };
    if win_samples <= 0.5 {
        // Simple linear interpolation at `pos`
        let i0 = pos.floor() as usize;
        let i1 = i0.saturating_add(1).min(n - 1);
        let t = pos - (i0 as f32);
        return vector[i0] * (1.0 - t) + vector[i1] * t;
    }

    // Gaussian windowing centered at `pos`
    // We'll cover roughly +/- win_samples/2 around center.
    let radius = (win_samples * 0.5).ceil() as isize;
    // Choose sigma so that ~3*sigma spans our radius.
    let sigma = ((radius as f32) / 3.0).max(1e-6);
    let inv_two_sigma2 = 1.0 / (2.0 * sigma * sigma);

    let center = pos;
    let start = 0isize.max(center.floor() as isize - radius);
    let end = (n as isize - 1).min(center.ceil() as isize + radius);

    let mut wsum = 0.0f32;
    let mut vsum = 0.0f32;

    for i in start..=end {
        let dx = (i as f32) - center; // fractional distance in samples
        let w = (-dx * dx * inv_two_sigma2).exp();
        let v = unsafe { *vector.get_unchecked(i as usize) }; // bounds ensured by start/end
        wsum += w;
        vsum += w * v;
    }

    if wsum > 0.0 { vsum / wsum } else { 0.0 }
}
