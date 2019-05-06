use amethyst::ecs::{Component, VecStorage};


const TAO: f32 = 1.618033988749895;
const PLNT_SURFACE_RECURSION: u16 = 1;
const TERR_HEIGHT_LIM: f32 = 10.0;  // Multiplied by radius

pub struct Planet {
   radius: f32,
   pub mesh: Mesh,
   pub tiles: HashMap<usize, Tile>,  // Key is the index of the face in the mesh,
}

impl Planet {
   pub fn generate(window: &mut Window, mesh_manager: &mut MeshManager, rad: f32, iter: u16) -> Planet {
      let pl = PlanetGenerator::generate_planet(rad, iter);

      pl
   }

   //pub fn update() {
      
   //}
}

pub struct Tile {
   pub biome: Biome,
}

impl Component for Tile {
   type Storage = VecStorage<Self>;
}

pub enum Biome {
   Plains,
}


struct PlanetGenerator {
   vertices: Vec<Point3<f32>>,
   faces: Vec<Point3<u16>>,
   mid_point_index_cache: HashMap<u16, u16>,
   radius: f32,
}

impl PlanetGenerator {
   fn new(rad: f32) -> PlanetGenerator {
      PlanetGenerator {
         vertices: vec![],
         faces: vec![],
         mid_point_index_cache: HashMap::new(),
         radius: rad,
      }
   }

   fn generate_planet(rad: f32, iter: u16) -> Planet {
      let mut generator = PlanetGenerator::new(rad);
      let (vertices, faces) = generator.generate_icosphere(iter);
      //let terrain_map = generator.generate_terrain(10.0);  // Gets hash map for getting tile information, and changes height of points for cliffs etc.

      Planet {
         radius: rad,
         mesh: Mesh::new(vertices, faces, None, None, false),
         tiles: HashMap::new(),//terrain_map,
      }
   }

   fn generate_terrain(&mut self, max_height: f32) -> HashMap<usize, Tile> {
      let mut tile_map: HashMap<usize, Tile> = HashMap::new();
      for (i, face) in self.faces.iter().enumerate() {
         tile_map.insert(i, Tile { biome: Biome::Plains });
      }

      tile_map
   }

   fn generate_icosphere(&mut self, recursion_level: u16) -> (Vec<Point3<f32>>, Vec<Point3<u16>>) { // Returns vertices and faces
      self.add_vertex(Point3::new(-1.0, TAO, 0.0));   // 0
      self.add_vertex(Point3::new(1.0, TAO, 0.0));    // 1
      self.add_vertex(Point3::new(-1.0, -TAO, 0.0));  // 2
      self.add_vertex(Point3::new(1.0, -TAO, 0.0));   // 3

      self.add_vertex(Point3::new(0.0, -1.0, TAO));   // 4
      self.add_vertex(Point3::new(0.0, 1.0, TAO));   // 5
      self.add_vertex(Point3::new(0.0, -1.0, -TAO));  // 6
      self.add_vertex(Point3::new(0.0, 1.0, -TAO));   // 7

      self.add_vertex(Point3::new(TAO, 0.0, -1.0));   // 8
      self.add_vertex(Point3::new(TAO, 0.0, 1.0));    // 9
      self.add_vertex(Point3::new(-TAO, 0.0, -1.0));  //10
      self.add_vertex(Point3::new(-TAO, 0.0, 1.0));  //11

      self.faces =   // Then, each face will be given a biome
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

      for _ in 0..recursion_level {  // Further split icosohedron into an icosphere
         let mut faces2: Vec<Point3<u16>> = vec![];
         let temp_faces = self.faces.clone();
         for tri in temp_faces.iter() {
            let a = self.get_mid_point(tri.x, tri.y);
            let b = self.get_mid_point(tri.y, tri.z);
            let c = self.get_mid_point(tri.z, tri.x);

            faces2.push(Point3::new(tri.x, a, c));
            faces2.push(Point3::new(tri.y, b, a));
            faces2.push(Point3::new(tri.z, c, b));
            faces2.push(Point3::new(a, b, c));

         }
         
         self.faces = faces2;
      }

      for i in 0..self.vertices.len() {
        self.vertices[i] *= self.radius
      }

      (self.vertices.clone(), self.faces.clone())
   }

   fn get_mid_point(&mut self, p1: u16, p2: u16) -> u16 { // Returns index of new point
      let (small_ind, big_ind) = if p1 < p2 { (p1, p2) } else { (p2, p1) };   // Since an edge is shared between two faces, the points may already be calculated.
      let key = (small_ind << 8) + big_ind;
      
      if self.mid_point_index_cache.contains_key(&key) {
         println!("Found in cache");
         return *self.mid_point_index_cache.get(&key).unwrap();
      }

      // Not in cache, so calculate it

      let point1 = self.vertices[p1 as usize];
      let point2 = self.vertices[p2 as usize];
      let middle = Point3::new(
         (point1.x + point2.x)/2.0,
         (point1.y + point2.y)/2.0,
         (point1.z + point2.z)/2.0);

      let i = self.add_vertex(middle);

      // Add to the cache
      self.mid_point_index_cache.insert(key, i);

      i
   }

   fn add_vertex(&mut self, p: Point3<f32>) -> u16 {
      let len = ((p.x * p.x) + (p.y * p.y) + (p.z * p.z)).sqrt();
      self.vertices.push(p/len);
      (self.vertices.len() - 1) as u16  // Returns index.
   }
}

#[inline]
fn cartesian_to_spherical(v: Point3<f32>, center: Point3<f32>) -> Vector3<f32> {  // Converts catesian to radius, inclination and azimuth (r, i, a) = (x, y ,z) in vector. All coords in f32 format
   let dist_vec = v - center;
   let r = (dist_vec.x * dist_vec.x) + (dist_vec.y * dist_vec.y) + (dist_vec.z * dist_vec.z);
   Vector3::new(r, (dist_vec.z/r).acos(), (dist_vec.y/dist_vec.x).atan())
}

#[inline]
fn spherical_to_cartesian(v: Vector3<f32>) -> Point3<f32> {
   let sin_inc = v.y.sin();
   Point3::new(v.x * sin_inc * v.z.cos(), v.x * sin_inc * v.z.sin(), v.x * v.y.cos())
}
