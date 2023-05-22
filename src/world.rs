// File: world.rs

struct Location {
    x: i32,
    y: i32,
    z: i32,
}
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}
struct Acceleration {
    x: i32,
    y: i32,
    z: i32,
}
struct Mass(i32);
struct Force(i32);

struct Name(&'static str);
struct Age(i32);
enum Era {
    StoneAge,
    BronzeAge,
    IronAge,
    IndustrialAge,
    InformationAge,
}

struct World {
    // We'll use `entities_count` to assign each Entity a unique ID.
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}
impl World {
    fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        /* do stuff */
        todo!("add_component_to_entity");
    }
}

trait ComponentVec {
    fn push_none(&mut self);
    /* we'll add more functions here in a moment */
}

impl<T> ComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None)
    }
}
