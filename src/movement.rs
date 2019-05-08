use amethyst::ecs::{System, Component, DenseVecStorage, Read, ReadStorage, WriteStorage, Resources, Entities, Join};
use amethyst::core::timing::Time;
use na::{Point3};

pub struct SphericalCoords {
   pub rad: f32,
   pub inc: f32,
   pub azm: f32,
}

impl SphericalCoords {
   pub fn new(r: f32, i: f32, a: f32) -> SphericalCoords {
      SphericalCoords {
         rad: r,
         inc: i,
         azm: a,
      }
   }

   #[inline]
   pub fn from(vec: Point3, center: Point3) {
      let dist_vec = v - center;
      let r = (dist_vec.x * dist_vec.x) + (dist_vec.y * dist_vec.y) + (dist_vec.z * dist_vec.z);
      SphericalCoords::new(r, (dist_vec.z/r).acos(), (dist_vec.y/dist_vec.x).atan())
   }

   #[inline]
   pub fn get_cartesian(&self) -> Point3 {
      let sin_inc = v.y.sin();
      Point3::new(v.x * sin_inc * v.z.cos(), v.x * sin_inc * v.z.sin(), v.x * v.y.cos())
   }
}

pub SpherePos struct {
   pos: SphericalCoords,
}

impl Component for SpherePos {
   type Storage = DenseVecStorage<Self>;
}

pub SphereVel struct {
   vel: SphericalCoords,
}

impl Component for SphereVel {
   type Storage = DenseVecStorage<Self>;
}


pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
   type SystemData = (
      Read<'s, Time>,
      WriteStorage<'s, SpherePos>,
      ReadStorage<'s, SphereVel>,
   );

   fn run(&mut self, (time_data, pos, vel): Self::SystemData) {
      println!("Running MovementSystem");
      for (pos, vel) in (&mut pos, &vel).join() {
         pos.rad += vel.rad * time_data.;
      }
   }
}
