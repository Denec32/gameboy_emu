use crate::game_boy::memory::object_attribute_memory::object_attributes::ObjectAttributes;

mod object_attributes;

struct ObjectAttributeMemory {
    objects: Vec<ObjectAttributes>,
}

impl ObjectAttributeMemory {
    fn new() -> ObjectAttributeMemory {
        let mut objects: Vec<ObjectAttributes> = Vec::new();
        for _ in 0..40 {
            objects.push(ObjectAttributes::new());
        }
        ObjectAttributeMemory { objects }
    }
}