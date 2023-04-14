use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AudioBuffer {
    data: Arc<RwLock<Vec<(f64, f64)>>>,
}

impl AudioBuffer {
    pub fn new(sample_rate: u32) -> Self {
        let mut data = Vec::new();
        data.resize(sample_rate as usize / 2, (0.0, 0.0));

        Self {
            data: Arc::new(RwLock::new(data)),
        }
    }

    pub fn reset(&self) {
        for x in self.data.write().unwrap().iter_mut() {
            *x = (0.0, 0.0);
        }
    }

    pub fn write_sample(&self, idx: usize, sample: (f64, f64)) {
        self.data.write().unwrap()[idx] = sample;
    }

    pub fn peak(&self) -> (f64, f64) {
        self.data
            .read()
            .unwrap()
            .iter()
            .copied()
            .fold((0.0f64, 0.0f64), |(al, ar), (l, r)| {
                (al.max(l.abs()), ar.max(r.abs()))
            })
    }
}
