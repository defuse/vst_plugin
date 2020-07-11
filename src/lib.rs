#[macro_use]
extern crate vst;
extern crate time;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

use std::sync::Arc;

struct VstEffect {
    params: Arc<VstEffectParameters>,
}

struct VstEffectParameters {
    amplitude: AtomicFloat,
}

impl Default for VstEffect {
    fn default() -> VstEffect {
        VstEffect{ 
            params: Arc::new(VstEffectParameters::default()),
        }
    }
}

impl Default for VstEffectParameters {
    fn default() -> VstEffectParameters {
        VstEffectParameters {
            amplitude: AtomicFloat::new(0.5),
        }
    }
}

impl Plugin for VstEffect {
    fn get_info(&self) -> Info {
        Info {
            name: "Example VST Effect".to_string(),
            vendor: "Taylor Hornby".to_string(),
            unique_id: 698588124,
            version: 1,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            category: Category::Effect,
            initial_delay: 0,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let amplitude = self.params.amplitude.get();

        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                *output_sample = *input_sample * amplitude;
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

impl PluginParameters for VstEffectParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.amplitude.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, val: f32) {
        #[allow(clippy::sincle_match)]
        match index {
            0 => self.amplitude.set(val),
            _ => (),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", self.amplitude.get()),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Amplitude",
            _ => "",
        }
        .to_string()
    }
}

plugin_main!(VstEffect);