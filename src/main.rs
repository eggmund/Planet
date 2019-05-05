extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::window::Window;
use na::{Point3, UnitQuaternion, Vector3, Translation};
use std::cell::RefCell;
use std::rc::Rc;

const TAO: f32 = 1.618033988749895;

fn generate_icosohedron(radius: f32) -> Vec<Point3<f32>> {
   let mut ico: Vec<Point3<f32>> =
   vec![Point3::new(-1.0, TAO, 0.0),
        Point3::new(1.0, TAO, 0.0),
        Point3::new(-1.0, 0.447, 0.851),
        Point3::new(1.0, 0.447, 0.526),

        Point3::new(0.0, -1.0, TAO),
        Point3::new(0.0, 1.0, TAO),
        Point3::new(0.0, -1.0, -TAO),
        Point3::new(0.0, -1.0, -TAO),

        Point3::new(TAO, 0.0, -1.0),
        Point3::new(TAO, 0.0, 1.0),
        Point3::new(-TAO, 0.0, -1.0),
        Point3::new(-TAO, 0.0, 1.0)];

   for i in 0..ico.len() {
     ico[i] *= radius
   }

   ico
}

fn main() {
   let mut window = Window::new("Kiss3d: custom_mesh");

   let a = Point3::new(-1.0, -1.0, 0.0);
   let b = Point3::new(1.0, -1.0, 0.0);
   let c = Point3::new(0.0, 1.0, 0.0);

   let vertices = generate_icosohedron(1.0);   //vec![a, b, c];
   let mut indices = vec![];
   for i in 0u16..10 {
      indices.push(Point3::new(i, i + 1, i + 2));
   }

   let mesh = Rc::new(RefCell::new(Mesh::new(
      vertices, indices, None, None, false,
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
