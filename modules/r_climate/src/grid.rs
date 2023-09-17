use godot::{prelude::*, engine::Area3D};

#[derive(GodotClass)]
#[class(base=Area3D)]
pub struct GridComponent {
    #[var]
    pub index: Vector3i,
    #[var]
    pub mean_altitude: real,
    #[var]
    pub temperature: real,
    #[var]
    pub wind: Vector3,
    #[var]
    pub pressure: real,
    #[var]
    pub humidity: real,
    #[var]
    pub td: real,
    #[base]
    base: Base<Area3D>
}

#[godot_api]
impl GridComponent {
    
}

#[godot_api]
impl NodeVirtual for GridComponent {
    fn init(base: Base<Area3D>) -> Self {
        GridComponent {
            index: Vector3i::new(0, 0, 0),
            mean_altitude: 0.0,
            temperature: 0.0,
            wind: Vector3::new(0.0, 0.0, 0.0),
            pressure: 0.0,
            humidity: 0.0,
            td: 0.0,
            base
        }
    }

    fn ready(&mut self) {
        
    }
}