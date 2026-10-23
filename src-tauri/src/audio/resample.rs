/// Resampler de streaming con interpolación lineal y un filtro paso bajo
/// simple para downsampling. Suficiente para voz a 16/24 kHz.
pub struct StreamingResampler {
    from: u32,
    to: u32,
    pos: f64,
    prev: f32,
    lp_state: f32,
    lp_alpha: f32,
    use_lp: bool,
}

impl StreamingResampler {
    pub fn new(from: u32, to: u32) -> Self {
        let from = from.max(1);
        let to = to.max(1);
        let use_lp = from > to;
        let cutoff = (to as f32 * 0.45).min(from as f32 * 0.45);
        let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff.max(1.0));
        let dt = 1.0 / from as f32;
        let lp_alpha = (dt / (rc + dt)).clamp(0.0001, 1.0);
        Self {
            from,
            to,
            pos: 0.0,
            prev: 0.0,
            lp_state: 0.0,
            lp_alpha,
            use_lp,
        }
    }

    pub fn process(&mut self, input: &[f32]) -> Vec<f32> {
        if input.is_empty() {
            return Vec::new();
        }
        if self.from == self.to {
            return input.to_vec();
        }

        let mut filtered: Vec<f32> = Vec::with_capacity(input.len());
        if self.use_lp {
            for &sample in input {
                self.lp_state += self.lp_alpha * (sample - self.lp_state);
                filtered.push(self.lp_state);
            }
        } else {
            filtered.extend_from_slice(input);
        }

        let step = self.from as f64 / self.to as f64;
        let mut pos = self.pos;
        let mut out = Vec::with_capacity(((input.len() as f64 / step) + 2.0) as usize);
        let len = filtered.len();

        while pos < len as f64 {
            let index = pos.floor() as usize;
            let frac = (pos - index as f64) as f32;
            let a = if index == 0 {
                self.prev
            } else {
                filtered[index - 1]
            };
            let b = filtered[index];
            out.push(a + (b - a) * frac);
            pos += step;
        }

        self.pos = pos - len as f64;
        if let Some(&last) = filtered.last() {
            self.prev = last;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn downsamples_length() {
        let mut r = StreamingResampler::new(48000, 16000);
        let input = vec![0.1f32; 4800];
        let out = r.process(&input);
        assert!((out.len() as i32 - 1600).abs() <= 2, "len={}", out.len());
    }

    #[test]
    fn upsamples_length() {
        let mut r = StreamingResampler::new(24000, 48000);
        let input = vec![0.1f32; 2400];
        let out = r.process(&input);
        assert!((out.len() as i32 - 4800).abs() <= 2, "len={}", out.len());
    }

    #[test]
    fn streams_without_panicking() {
        let mut r = StreamingResampler::new(44100, 16000);
        for _ in 0..20 {
            let out = r.process(&vec![0.05f32; 441]);
            assert!(out.len() < 200);
        }
    }
}
