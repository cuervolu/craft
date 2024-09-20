use crate::frameworks::Framework;
use crate::frameworks::nuxt::Nuxt;

pub(crate) fn get_framework(name: &str) -> Box<dyn Framework> {
    match name {
        "Nuxt" => Box::new(Nuxt),
        // Add other frameworks here
        _ => panic!("Unsupported framework: {}", name),
    }
}