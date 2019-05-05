use na::{Point3};
use kiss3d::resource::Mesh;

const TAO: f32 = 1.618033988749895;
const PLNT_SURFACE_RECURSION: u16 = 1;

pub struct Planet {
   pub mesh: Mesh,
}

impl Planet {
   pub fn generate(rad: f32) -> Planet {
      let (vertices, faces) = generate_icosohedron(rad, PLNT_SURFACE_RECURSION);

      Planet {
         mesh: Mesh::new(vertices, faces, None, None, false),
      }
   }

   pub fn update() {
      
   }
}

fn generate_icosohedron(radius: f32, recursion_level: u16) -> (Vec<Point3<f32>>, Vec<Point3<u16>>) {
   let mut ico: Vec<Point3<f32>> =
      vec![Point3::new(-1.0, TAO, 0.0),   // 0
           Point3::new(1.0, TAO, 0.0),    // 1
           Point3::new(-1.0, -TAO, 0.0),  // 2
           Point3::new(1.0, -TAO, 0.0),   // 3

           Point3::new(0.0, -1.0, TAO),   // 4
           Point3::new(0.0, 1.0, TAO),    // 5
           Point3::new(0.0, -1.0, -TAO),  // 6
           Point3::new(0.0, 1.0, -TAO),   // 7

           Point3::new(TAO, 0.0, -1.0),   // 8
           Point3::new(TAO, 0.0, 1.0),    // 9
           Point3::new(-TAO, 0.0, -1.0),  //10
           Point3::new(-TAO, 0.0, 1.0)];  //11

   let faces: Vec<Point3<u16>> =
      vec![Point3::new(0, 11, 5),
           Point3::new(0, 5, 1),
           Point3::new(0, 1, 7),
           Point3::new(0, 7, 10),
           Point3::new(0, 10, 11),

           Point3::new(1, 5, 9),
           Point3::new(5, 11, 4),
           Point3::new(11, 10, 2),
           Point3::new(10, 7, 6),
           Point3::new(7, 1, 8),

           Point3::new(3, 9, 4),
           Point3::new(3, 4, 2),
           Point3::new(3, 2, 6),
           Point3::new(3, 6, 8),
           Point3::new(3, 8, 9),

           Point3::new(4, 9, 5),
           Point3::new(2, 4, 11),
           Point3::new(6, 2, 10),
           Point3::new(8, 6, 7),
           Point3::new(9, 8, 1)];

   for i in 0..ico.len() {
     ico[i] *= radius
   }

   /*
   for i in 0..recursion_level {
      let faces2: Vec<Point3<u16>> = Vec<u16>::new();
      for tri in faces.iter() {
         
      }
   
   }
   */

   (ico, faces)
}
