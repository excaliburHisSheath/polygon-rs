use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::f32::{MAX, MIN};
use std::{mem, thread};
use std::sync::{Arc, Mutex, Condvar, RwLock};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread::JoinHandle;

use bootstrap::time::{Timer, TimeMark};
use hash::*;
use math::*;
use stopwatch::Stopwatch;

use ecs::Entity;
use super::bounding_volume::*;

const NUM_WORKERS: usize = 8;
const NUM_WORK_UNITS: usize = 8;

pub type CollisionGrid = HashMap<GridCell, Vec<*const BoundVolume>, FnvHashState>;

/// A collision processor that partitions the space into a regular grid.
///
/// # TODO
///
/// - Do something to configure the size of the grid.
pub struct GridCollisionSystem {
    _workers: Vec<JoinHandle<()>>,
    thread_data: Arc<ThreadData>,
    channel: Receiver<WorkUnit>,
    processed_work: Vec<WorkUnit>,
    pub collisions: HashSet<(Entity, Entity), FnvHashState>,
}

impl GridCollisionSystem {
    pub fn new() -> GridCollisionSystem {
        let thread_data = Arc::new(ThreadData {
            volumes: RwLock::new(Vec::new()),
            pending: (Mutex::new(Vec::new()), Condvar::new()),
        });

        let mut processed_work = Vec::new();
        if NUM_WORK_UNITS == 1 {
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(MIN, MIN, MIN),
                max: Point::new(0.0, 0.0, 0.0),
            }));
        } else if NUM_WORK_UNITS == 2 {
            processed_work.push(WorkUnit::new(AABB {
                min: Point::min(),
                max: Point::new(0.0, MAX, MAX),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(0.0, MIN, MIN),
                max: Point::max(),
            }));
        } else if NUM_WORK_UNITS == 4 {
            processed_work.push(WorkUnit::new(AABB {
                min: Point::min(),
                max: Point::new(0.0, 0.0, MAX),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(MIN, 0.0, MIN),
                max: Point::new(0.0, MAX, MAX),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(0.0, MIN, MIN),
                max: Point::new(MAX, 0.0, MAX),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(0.0, 0.0, MIN),
                max: Point::max(),
            }));
        } else if NUM_WORK_UNITS == 8 {
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(MIN, MIN, MIN),
                max: Point::new(0.0, 0.0, 0.0),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(MIN, MIN, 0.0),
                max: Point::new(0.0, 0.0, MAX),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(MIN, 0.0, MIN),
                max: Point::new(0.0, MAX, 0.0),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(MIN, 0.0, 0.0),
                max: Point::new(0.0, MAX, MAX),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(0.0, MIN, MIN),
                max: Point::new(MAX, 0.0, 0.0),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(0.0, MIN, 0.0),
                max: Point::new(MAX, 0.0, MAX),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(0.0, 0.0, MIN),
                max: Point::new(MAX, MAX, 0.0),
            }));
            processed_work.push(WorkUnit::new(AABB {
                min: Point::new(0.0, 0.0, 0.0),
                max: Point::new(MAX, MAX, MAX),
            }));
        } else {
            panic!("unsupported number of workers {}, only 1, 2, 4, or 8 supported", NUM_WORK_UNITS);
        }

        let (sender, receiver) = mpsc::sync_channel(NUM_WORKERS);
        let mut workers = Vec::new();
        for _ in 0..NUM_WORKERS {
            let thread_data = thread_data.clone();
            let sender = sender.clone();
            workers.push(thread::spawn(move || {
                let mut worker = Worker::new(thread_data, sender);
                worker.start();
            }));
        }

        GridCollisionSystem {
            _workers: workers,
            thread_data: thread_data.clone(),
            channel: receiver,
            collisions: HashSet::default(),
            processed_work: processed_work,
        }
    }

    pub fn update(&mut self, bvh_manager: &BoundingVolumeManager) {
        let _stopwatch = Stopwatch::new("Grid Collision System");

        self.collisions.clear();
        let timer = Timer::new();
        let start_time = timer.now();

        let thread_data = &*self.thread_data;

        // Convert all completed work units into pending work units, notifying a worker thread for each one.
        {
            let _stopwatch = Stopwatch::new("Preparing Work Units");

            assert!(
                self.processed_work.len() == NUM_WORK_UNITS,
                "Expected {} complete work units, found {}",
                NUM_WORK_UNITS,
                self.processed_work.len(),
            );

            let mut longest = 0.0;
            for bvh in bvh_manager.components() {
                let diff_x = bvh.aabb.max.x - bvh.aabb.min.x;
                let diff_y = bvh.aabb.max.y - bvh.aabb.min.y;
                let diff_z = bvh.aabb.max.z - bvh.aabb.min.z;

                if diff_x > longest {
                    longest = diff_x;
                }

                if diff_y > longest {
                    longest = diff_y;
                }

                if diff_z > longest {
                    longest = diff_z;
                }
            }

            for work_unit in self.processed_work.iter_mut() {
                work_unit.cell_size = longest;
            }

            // Prepare work unit by giving it a copy of the list of volumes.
            let mut volumes = thread_data.volumes.write().unwrap();
            volumes.clone_from(bvh_manager.components());

            let &(ref pending, _) = &thread_data.pending;
            let mut pending = pending.lock().unwrap();

            // Swap all available work units into the pending queue.
            mem::swap(&mut *pending, &mut self.processed_work);
        }

        // Synchronize with worker threads to get them going or whatever.
        {
            let _stopwatch = Stopwatch::new("Synchronizing To Start Workers");
            let &(_, ref condvar) = &thread_data.pending;
            condvar.notify_all();
        }

        // Wait until all work units have been completed and returned.
        let _stopwatch = Stopwatch::new("Running Workers and Merging Results");
        while self.processed_work.len() < NUM_WORK_UNITS {
            // Retrieve each work unit as it becomes available.
            let mut work_unit = self.channel.recv().unwrap();
            work_unit.returned_time = timer.now();

            // Merge results of work unit into total.
            for (collision, _) in work_unit.collisions.drain() {
                self.collisions.insert(collision);
            }
            self.processed_work.push(work_unit);
        }

        println!("\n-- TOP OF GRID UPDATE --");
        println!("Total Time: {}ms", timer.elapsed_ms(start_time));
        for work_unit in &self.processed_work {
            println!(
                "work unit returned: recieved @ {}ms, broadphase @ {}ms, narrowphase @ {}ms, returned @ {}ms",
                timer.duration_ms(work_unit.received_time - start_time),
                timer.duration_ms(work_unit.broadphase_time - start_time),
                timer.duration_ms(work_unit.narrowphase_time - start_time),
                timer.duration_ms(work_unit.returned_time - start_time),
            );
        }
    }
}

impl Clone for GridCollisionSystem {
    /// `GridCollisionSystem` doesn't have any real state between frames, it's only used to reuse
    /// the grid's allocated memory between frames. Therefore to clone it we just invoke
    /// `GridCollisionSystem::new()`.
    fn clone(&self) -> Self {
        GridCollisionSystem::new()
    }
}

#[derive(Debug)]
#[allow(raw_pointer_derive)]
struct WorkUnit {
    collisions: HashMap<(Entity, Entity), (), FnvHashState>, // This should be a HashSet, but HashSet doesn't have a way to get at entries directly.
    bounds: AABB,

    grid: HashMap<GridCell, Vec<*const BoundVolume>, FnvHashState>,
    cell_size: f32,

    received_time: TimeMark,
    broadphase_time: TimeMark,
    narrowphase_time: TimeMark,
    returned_time: TimeMark,
}

impl WorkUnit {
    fn new(bounds: AABB) -> WorkUnit {
        let timer = Timer::new();
        WorkUnit {
            bounds: bounds,
            collisions: HashMap::default(),

            grid: HashMap::default(),
            cell_size: 1.0,

            received_time: timer.now(),
            broadphase_time: timer.now(),
            narrowphase_time: timer.now(),
            returned_time: timer.now(),
        }
    }

    /// Converts a point in world space to its grid cell.
    fn world_to_grid(&self, point: Point) -> GridCell {
        GridCell {
            x: (point.x / self.cell_size).floor() as GridCoord,
            y: (point.y / self.cell_size).floor() as GridCoord,
            z: (point.z / self.cell_size).floor() as GridCoord,
        }
    }
}

unsafe impl ::std::marker::Send for WorkUnit {}

struct ThreadData {
    volumes: RwLock<Vec<BoundVolume>>,
    pending: (Mutex<Vec<WorkUnit>>, Condvar),
}

struct Worker {
    thread_data: Arc<ThreadData>,
    channel: SyncSender<WorkUnit>,

    candidate_collisions: Vec<(*const BoundVolume, *const BoundVolume)>,
    cell_cache: Vec<Vec<*const BoundVolume>>,
}

impl Worker {
    fn new(thread_data: Arc<ThreadData>, channel: SyncSender<WorkUnit>) -> Worker {
        Worker {
            thread_data: thread_data,
            channel: channel,
            candidate_collisions: Vec::new(),
            cell_cache: Vec::new(),
        }
    }

    fn start(&mut self) {
        let timer = Timer::new();
        loop {
            // Wait until there's pending work, and take the first available one.
            let mut work = {
                let &(ref pending, ref condvar) = &self.thread_data.pending;
                let mut pending = pending.lock().unwrap();
                while pending.len() == 0 {
                    pending = condvar.wait(pending).unwrap();
                }

                pending.pop().unwrap()
            };
            work.received_time = timer.now();

            self.do_broadphase(&mut work);
            work.broadphase_time = timer.now();

            self.do_narrowphase(&mut work);
            work.narrowphase_time = timer.now();

            // Send completed work back to main thread.
            self.channel.send(work).unwrap();
        }
    }

    fn do_broadphase(&mut self, work: &mut WorkUnit) {
        // let _stopwatch = Stopwatch::new("Broadphase Testing (Grid Based)");
        let volumes = self.thread_data.volumes.read().unwrap();
        for bvh in &*volumes {
            // Retrieve the AABB at the root of the BVH.
            let aabb = bvh.aabb;

            // Only test volumes that are within the bounds of this work unit's testing area.
            if !aabb.test_aabb(&work.bounds) {
                continue;
            }

            let min = work.world_to_grid(aabb.min);
            let max = work.world_to_grid(aabb.max);
            debug_assert!(
                max.x - min.x <= 1
             && max.y - min.y <= 1
             && max.z - min.z <= 1,
                "AABB spans too many grid cells (min: {:?}, max: {:?}), grid cells are too small, bvh: {:?}",
                min,
                max,
                bvh);

            // Iterate over all grid cells that the AABB touches. Test the BVH against any entities
            // that have already been placed in that cell, then add the BVH to the cell, creating
            // new cells as necessary.
            {
                let cell_cache = &mut self.cell_cache;
                let candidate_collisions = &mut self.candidate_collisions;
                let _cell_size = work.cell_size;
                let mut test_cell = |grid_cell: GridCell| {
                    // // Visualize test cell.
                    // ::debug_draw::box_min_max(
                    //     Point::new(
                    //         grid_cell.x as f32 * _cell_size,
                    //         grid_cell.y as f32 * _cell_size,
                    //         grid_cell.z as f32 * _cell_size,
                    //     ),
                    //     Point::new(
                    //         grid_cell.x as f32 * _cell_size + _cell_size,
                    //         grid_cell.y as f32 * _cell_size + _cell_size,
                    //         grid_cell.z as f32 * _cell_size + _cell_size,
                    //     )
                    // );

                    let mut cell = work.grid.entry(grid_cell).or_insert_with(|| {
                        cell_cache.pop().unwrap_or(Vec::new())
                    });

                    // Check against other volumes.
                    for other_bvh in cell.iter().cloned() {
                        candidate_collisions.push((bvh, other_bvh));
                    }

                    // Add to existing cell.
                    cell.push(bvh);
                };

                test_cell(min);

                let overlap_x = min.x < max.x;
                let overlap_y = min.y < max.y;
                let overlap_z = min.z < max.z;

                // Test cases where volume overlaps along x.
                if overlap_x {
                    test_cell(GridCell::new(max.x, min.y, min.z));

                    if overlap_y {
                        test_cell(GridCell::new(min.x, max.y, min.z));
                        test_cell(GridCell::new(max.x, max.y, min.z));

                        if overlap_z {
                            test_cell(GridCell::new(min.x, min.y, max.z));
                            test_cell(GridCell::new(min.x, max.y, max.z));
                            test_cell(GridCell::new(max.x, min.y, max.z));
                            test_cell(GridCell::new(max.x, max.y, max.z));
                        }
                    } else if overlap_z {
                        test_cell(GridCell::new(min.x, min.y, max.z));
                        test_cell(GridCell::new(max.x, min.y, max.z));
                    }
                } else if overlap_y {
                    test_cell(GridCell::new(min.x, max.y, min.z));

                    if overlap_z {
                        test_cell(GridCell::new(min.x, min.y, max.z));
                        test_cell(GridCell::new(min.x, max.y, max.z));
                    }
                } else if overlap_z {
                    test_cell(GridCell::new(min.x, min.y, max.z));
                }
            }
        }

        // Clear out grid contents from previous frame, start each frame with an empty grid and
        // rebuild it rather than trying to update the grid as objects move.
        for (_, mut cell) in work.grid.drain() {
            cell.clear();
            self.cell_cache.push(cell);
        }
    }

    fn do_narrowphase(&mut self, work: &mut WorkUnit) {
        // let _stopwatch = Stopwatch::new("Narrowphase Testing");
        for (bvh, other_bvh) in self.candidate_collisions.drain(0..) {
            let bvh = unsafe { &*bvh };
            let other_bvh = unsafe { &*other_bvh };
            let collision_pair = (bvh.entity, other_bvh.entity);

            // Check if the collision has already been detected before running the
            // collision test since it's potentially very expensive. We get the entry
            // directly, that way we only have to do one hash lookup.
            match work.collisions.entry(collision_pair) {
                Entry::Vacant(vacant_entry) => {
                    // Collision hasn't already been detected, so do the test.
                    if bvh.test(other_bvh) {
                        // Woo, we have a collison.
                        vacant_entry.insert(());
                    }
                },
                _ => {},
            }
        }
    }
}

/// A wrapper type around a triple of coordinates that uniquely identify a grid cell.
///
/// # Details
///
/// Grid cells are axis-aligned cubes of a regular sice. The coordinates of a grid cell are its min
/// value. This was chosen because of how it simplifies the calculation to find the cell for a
/// given point (`(point / cell_size).floor()`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCell {
    pub x: GridCoord,
    pub y: GridCoord,
    pub z: GridCoord,
}

// TODO: Using i16 for the grid coordinate makes the hash lookups substantially faster, but it means
//       we'll have to take extra care when mapping world coordinates to grid coordinates. Points
//       outside the representable range should be wrapped around. This will technically lead to
//       more grid collisions, but extras will be culled quickly by the AABB test so it shouldn't
//       be more of a performance hit than what we gained from converting to using i16s.
pub type GridCoord = i16;

impl GridCell {
    pub fn new(x: GridCoord, y: GridCoord, z: GridCoord) -> GridCell {
        GridCell {
            x: x,
            y: y,
            z: z,
        }
    }
}
