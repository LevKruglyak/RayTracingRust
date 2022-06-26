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

# Big Rearchtitecturing

    Scene
    - Every resource in the program is stored in the Scene
    - Single instance data
        - Render settings
        - Scene background handle
    - Resources are referred to by a handle:
        - Mesh data
            - Different numbers of vertex attributes / groups
        - Material
            - Possible inputs:
                - Texture
        - Texture
            - Just the raw texture data, some basic properties, nothing else
        - Objects
            - Can contain mesh data or analytic data (e.g. sphere mesh or perfect sphere)
            - Contain materials
            - Object properties such as smooth shade

    Object Traits
    - Hittable -> something that can be hit by a ray


