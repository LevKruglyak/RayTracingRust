use ray_tracing_rust::render::RayTracingDemo;

pub fn setup(width: u32, height: u32) -> RayTracingDemo {
    RayTracingDemo::load("scenes/simple.json")
}
