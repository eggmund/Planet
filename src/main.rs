extern crate kiss3d;
extern crate nalgebra as na;

mod planet;
use planet::Planet;

use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{UnitQuaternion, Vector3, Translation};
use std::cell::RefCell;
use std::rc::Rc;


fn main() {
   let mut window = Window::new("Kiss3d: custom_mesh");

   let pl = Planet::generate(2.0, 2);
   let mut c = window.add_mesh(Rc::new(RefCell::new(pl.mesh)), Vector3::new(1.0, 1.0, 1.0));  // Add planet's mesh to window
   c.set_color(0.9, 0.9, 0.9);
   c.enable_backface_culling(true);
   c.set_local_translation(Translation::from(Vector3::new(0.0, 0.0, 10.0)));

   let pl2 = Planet::generate(2.0, 0);
   let mut c2 = window.add_mesh(Rc::new(RefCell::new(pl2.mesh)), Vector3::new(1.0, 1.0, 1.0));  // Add planet's mesh to window
   c2.set_color(0.9, 0.9, 0.9);
   c2.enable_backface_culling(true);
   c2.set_local_translation(Translation::from(Vector3::new(5.0, 0.0, 10.0)));


   window.set_light(Light::StickToCamera);

   let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

   while window.render() {
      c.prepend_to_local_rotation(&rot);
   }
}
