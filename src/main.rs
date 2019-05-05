extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::window::Window;
use na::{Point3, UnitQuaternion, Vector3, Translation};
use std::cell::RefCell;
use std::rc::Rc;

const TAO: f32 = 1.618033988749895;

fn generate_icosohedron(radius: f32) -> (Vec<Point3<f32>>, Vec<Point3<u16>>) {
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

   (ico, faces)
}

fn main() {
   let mut window = Window::new("Kiss3d: custom_mesh");

   let a = Point3::new(-1.0, -1.0, 0.0);
   let b = Point3::new(1.0, -1.0, 0.0);
   let c = Point3::new(0.0, 1.0, 0.0);

   let (vertices, faces) = generate_icosohedron(1.0);   //vec![a, b, c];

   let mesh = Rc::new(RefCell::new(Mesh::new(
      vertices, faces, None, None, false,
   )));
   let mut c = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));

   c.set_color(1.0, 0.0, 0.0);
   c.enable_backface_culling(false);
   c.set_local_translation(Translation::from(Vector3::new(0.0, 0.0, 10.0)));

   window.set_light(Light::StickToCamera);

   let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

   while window.render() {
      c.prepend_to_local_rotation(&rot);
   }
}
