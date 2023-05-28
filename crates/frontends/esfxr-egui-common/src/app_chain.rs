use esfxr_dsp::{cpal, DspChain, DspParameters};

#[derive(Default)]
pub struct AppChain {
    pub parameters: DspParameters,
    pub chain: Option<DspChain>,
    pub stream: Option<cpal::Stream>,
    pub parameters_string: String,
}

impl AppChain {
    pub fn new_with_empty_stream() -> Self {
        let chain = DspChain::new().expect("could not build dsp chain");
        let stream = chain
            .build_empty_stream()
            .expect("could not build audio stream");

        let parameters = DspParameters::default();
        let parameters_string = parameters.to_b64();

        Self {
            parameters,
            chain: Some(chain),
            stream: Some(stream),
            parameters_string,
        }
    }

    pub fn update_parameters_string(&mut self) {
        self.parameters_string = self.parameters.to_b64();
    }

    pub fn load_from_parameters_string(&mut self) {
        self.parameters = DspParameters::from_b64(&self.parameters_string).unwrap();
    }

    pub fn play_stream(&mut self) {
        if self.chain.is_none() {
            self.chain = Some(DspChain::new().expect("could not build dsp chain"));
        }

        let stream = self
            .chain
            .as_ref()
            .expect("chain should be initialized")
            .build_stream(self.parameters.clone())
            .expect("could not build audio stream");
        self.stream = Some(stream);
    }
}
