use crate::material::BRDF;
use crate::utils::random_element;
use crate::vec3::Vec3;
use crate::Sphere;

pub struct LightSampler {
    lights: Vec<Sphere>,
}

impl LightSampler {
    pub fn new() -> LightSampler {
        LightSampler { lights: Vec::new() }
    }

    pub fn sample(&self) -> (Vec3, std::rc::Rc<dyn BRDF>, f64) {
        let (sampled_light, lights_pdf) = random_element(&self.lights);
        let (sampled_point, light_pdf) = sampled_light.sample_point();

        return (
            sampled_point,
            sampled_light.material.clone(),
            lights_pdf * light_pdf,
        );
    }

    pub fn add(&mut self, s: Sphere) {
        self.lights.push(s);
    }
}
