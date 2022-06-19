# Traits
    Scene -> Contains vector of Box<dyn Object>

                            + Serialize/Sync
    Bounded -> Hittable    ->      Object
                   |                  |
                   v                  v
                Bvh Trees,          Spheres, Meshes, etc..
                Hittable Lists

    Ideas:
        scene builds a bvh tree which contains "copy" of all the data in the scene
        requires downcasting Box<dyn Object> to Box<dyn Hittable>
