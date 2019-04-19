use lerp::*;
use rand::distributions::*;

use crate::core::geo::circle::*;
use crate::core::geo::intersect::segment_circle::*;
use crate::core::geo::polygon::*;
use crate::core::geo::segment2::*;

use crate::simulation::ai::pathfinding::find_path;
use crate::simulation::state::MoveMode;
use crate::simulation::barricade::*;

use super::state::*;

const SPRING_CONSTANT: f64 = 32.0;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum SoundType {
    GunshotHandgun,
    GunshotRifle,
    Reload,
    PersonInfected,
    ZombieDeath,
}

pub struct Sound {
    pub position: Vector2,
    pub sound_type: SoundType,
}

pub struct UpdateArgs {
    pub dt: Scalar
}

#[derive(Clone, Default)]
pub struct EntityCounts {
    pub civilians: usize,
    pub cops: usize,
    pub dead: usize,
    pub infected: usize,
    pub zombies: usize,
}
impl EntityCounts {
    pub fn total(&self) -> usize {
        self.civilians + self.cops + self.dead + self.zombies
    }
}

pub struct SimulationResults {
    pub entity_counts: EntityCounts,
    pub sounds: Vec<Sound>,
}

pub fn update(args: &UpdateArgs, state: &mut State) -> SimulationResults {

    let mut entity_counts = EntityCounts::default();
    let mut sounds = vec!();

    const DOUBLE_ENTITY_RADIUS_SQUARED: f64 = 4.0 * ENTITY_RADIUS * ENTITY_RADIUS;

    // Check for collisions
    for i in 0..state.entities.len() {
        let p1 = state.entities[i].position;
        let circle = Circle { center: p1, radius: ENTITY_RADIUS };

        if state.entities[i].is_dead() {
            continue
        }

        // Collisions with other entities
        for j in (i + 1)..state.entities.len() {

            // Do not collide with dead entities
            if state.entities[j].is_dead() {
                continue
            }

            let p2 = state.entities[j].position;

            let delta = p2 - p1;

            let delta_length_squared = delta.length_squared();

            if delta_length_squared < DOUBLE_ENTITY_RADIUS_SQUARED {
                handle_collision(args, &mut state.entities, i, j, &delta, delta_length_squared);
            }
        }

        // Collisions with buildings
        for j in 0..state.buildings.len() {
            // Check if position is inside the building
            let mut overlap = state.buildings[j].contains_point(p1);
            let inside = overlap;

            // Don't bother doing this if we already know there's an overlap
            if !inside {
                // Check if one of the building's sides intersects the entity
                for k in 0..state.buildings[j].num_sides() {
                    let segment = Segment2 {
                        p1: state.buildings[j].get(k),
                        p2: state.buildings[j].get((k + 1) % state.buildings[j].num_sides())
                    };

                    if segment_circle_has_intersection(&segment, &circle) {
                        overlap = true;
                        break;
                    }
                }
            }

            if overlap {
                handle_building_collision(args, &mut state.entities[i], &state.buildings[j], inside);
                break;
            }
        }

        // Collisions with barricades
        for j in 0..state.barricades.len() {
            let mut overlap = state.barricades[j].poly.contains_point(p1);
            let inside = overlap;

            if !inside {
                for k in 0..state.barricades[j].poly.num_sides() {
                    let segment = Segment2 {
                        p1: state.barricades[j].poly.get(k),
                        p2: state.barricades[j].poly.get((k + 1) % state.barricades[j].poly.num_sides())
                    };

                    if segment_circle_has_intersection(&segment, &circle) {
                        overlap = true;
                        break;
                    }
                }
            }

            if overlap {
                handle_barricade_collision(args, &mut state.entities[i], &mut state.barricades[j], inside);
                break;
            }
        }
    }

    // Apply individual behaviours
    for i in 0..state.entities.len() {

        // TODO: Ian - see if you can consolidate with the same trick used in update_cop
        let entity = unsafe { &mut *(&mut state.entities[i] as *mut Entity) };

        let infection_growth_factor = (args.dt * INFECTION_EXPONENTIAL_GROWTH_RATE).exp();

        match &mut entity.dead_or_alive {
            DeadOrAlive::Dead => {
                // Do nothing
                entity_counts.dead += 1;
            }
            DeadOrAlive::Alive { zombie_or_human, health } => {
                if *health <= ENTITY_HEALTH_MIN {
                    // Pay out bounty if zombie is killed
                    if entity.is_zombie() { state.money += 1 }
                    state.entities[i].dead_or_alive = DeadOrAlive::Dead;
                    sounds.push(Sound {
                        position: state.entities[i].position,
                        sound_type: SoundType::ZombieDeath
                    });
                }
                else {
                    match zombie_or_human {
                        ZombieOrHuman::Zombie{ state: zombie_state, left_hand_status, right_hand_status } => {
                            entity_counts.zombies += 1;
                            let old_state = zombie_state.clone();
                            let next_state =
                                update_zombie(&args, state, i, old_state, left_hand_status, right_hand_status);
                            *zombie_state = next_state;
                        }
                        ZombieOrHuman::Human { infection, human } => {
                            if *infection >= INFECTION_EXPONENTIAL_GROWTH_THRESHOLD {
                                entity_counts.infected += 1;
                                *infection *= infection_growth_factor;
                            }
                            if *infection >= INFECTION_MAX {
                                *zombie_or_human = ZombieOrHuman::Zombie {
                                    state: ZombieState::Roaming {
                                        jerk: Vector2::zero(),
                                        acceleration: Vector2::zero()
                                    },
                                    left_hand_status: HandStatus::Normal,
                                    right_hand_status: HandStatus::Normal
                                };
                                sounds.push(Sound {
                                    position: state.entities[i].position,
                                    sound_type: SoundType::PersonInfected
                                });
                            }
                            else {
                                match human {
                                    Human::Cop { .. } => {
                                        entity_counts.cops += 1;
                                        update_cop(&args, state, i, &mut sounds);
                                    }
                                    Human::Civilian { state: civilian_state, punch_time_cooldown, left_hand_status, right_hand_status } => {
                                        entity_counts.civilians += 1;
                                        let old_state = civilian_state.clone();
                                        let next_state = simulate_human(args, state, i, punch_time_cooldown, old_state, left_hand_status, right_hand_status);
                                        *civilian_state = next_state;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Apply acceleration
    for e in &mut state.entities {
        let displacement = args.dt * e.velocity;
        e.position += displacement;
        e.velocity -= ENTITY_DRAG * displacement;
    }

    // Remove motionless bullets
    state.projectiles.retain(
        |p| p.kind != ProjectileKind::Bullet ||
            p.velocity.length_squared() > BULLET_SPEED_MIN
    );

    // Remove fist and return to its owner
    let mut i = 0;
    while i < state.projectiles.len() {
        let p = state.projectiles[i];
        match p.kind {
            ProjectileKind::Fist { owner_index } => {
                if p.velocity.length_squared() <= FIST_SPEED_MIN {
                    let owner = &mut state.entities[owner_index];
                    match owner.dead_or_alive {
                        DeadOrAlive::Alive { health: _, ref mut zombie_or_human } => {
                            match zombie_or_human {
                                ZombieOrHuman::Zombie { state: _, left_hand_status: _, ref mut right_hand_status } => {
                                    *right_hand_status = HandStatus::Normal;
                                }
                                ZombieOrHuman::Human { infection: _, human } => {
                                    match human {
                                        Human::Civilian { state: _, punch_time_cooldown: _, left_hand_status: _, ref mut right_hand_status} => {
                                            *right_hand_status = HandStatus::Normal;
                                        }
                                        _ => ()
                                    }
                                }
                            }
                        }
                        _ => ()
                    }

                    state.projectiles.remove(i);
                } else {
                    i = i + 1;
                }
            }
            _ => {
                i = i + 1;
            }
        }
    }

    // Update projectiles
    for p in &mut state.projectiles {

        let displacement = args.dt * p.velocity;
        p.velocity -= PROJECTILE_DRAG * displacement;

        let mut segment = Segment2 { p1: p.position, p2: p.position + displacement };

        p.position = segment.p2;

        if p.kind == ProjectileKind::Casing {
            continue;
        }

        struct Collision {
            entity_id: usize,
            time: Scalar,
        }

        let mut first_collision = None;
        for i in 0..state.entities.len() {
            let entity = &state.entities[i];

            if let DeadOrAlive::Dead = entity.dead_or_alive {
                // Dead entities don't collide with bullets
                continue;
            }

            let circle = Circle { center: entity.position, radius: ENTITY_RADIUS };

            let this_collision_time = segment_circle_min_positive_intersect_time(&segment, &circle);

            match (&first_collision, &this_collision_time) {
                (None, Some(this_time)) => {
                    first_collision = Some(Collision {entity_id: i, time: *this_time})
                }
                (Some(Collision{time: first_time, ..}), Some(this_time))
                    if this_time < first_time => {
                    first_collision = Some(Collision{ entity_id: i, time: *this_time})
                }
                _ => ()
            }
        }

        match first_collision {
            Some(Collision{time, ..}) => {
                segment.p2 = segment.p1 + time * (segment.p2 - segment.p1);
            }
            None => ()
        }

        if !can_see(&state.buildings, segment.p1, segment.p2) {
            first_collision = None;
            p.velocity = Vector2::zero();
        }

        match p.kind {
            ProjectileKind::Bullet => {
                match &first_collision {
                    None => (),
                    Some(Collision{entity_id: i, ..}) => {

                        let distance_from_entity_center = segment
                            .distance_from_ray_to_point_squared(state.entities[*i].position)
                            .sqrt();

                        let distance_normalized
                        = (distance_from_entity_center - BULLET_MAX_DAMAGE_DISTANCE_FROM_ENTITY_CENTER)
                            / (BULLET_MIN_DAMAGE_DISTANCE_FROM_ENTITY_CENTER -
                            BULLET_MAX_DAMAGE_DISTANCE_FROM_ENTITY_CENTER);

                        let damage = BULLET_DAMAGE_MAX.lerp_bounded(BULLET_DAMAGE_MIN, distance_normalized);


                        match &mut state.entities[*i].dead_or_alive {
                            DeadOrAlive::Alive { health, .. } => { *health -= damage; }
                            _ => panic!("Only living entities should collide with bullets!")
                        }

                        p.velocity = Vector2::zero();
                    }
                }
            }
            ProjectileKind::Fist { owner_index } => {
                let owner = &mut state.entities[owner_index];
                match owner.dead_or_alive {
                    DeadOrAlive::Alive { health: _, ref mut zombie_or_human } => {
                        match zombie_or_human {
                            ZombieOrHuman::Zombie { .. } => {
                                match &first_collision {
                                    None => (),
                                    Some(Collision{entity_id: i, ..}) => {
                                        match &mut state.entities[*i].dead_or_alive {
                                            DeadOrAlive::Alive {zombie_or_human: ZombieOrHuman::Human { ref mut infection, .. }, ..} => {
                                                *infection += ZOMBIE_HUMAN_COLLISION_INFECTION_RATE
                                            }
                                            _ => ()
                                        }
                                        p.velocity = Vector2::zero();
                                    }
                                }
                            }
                            ZombieOrHuman::Human { .. } => {
                                match &first_collision {
                                    None => (),
                                    Some(Collision{entity_id: i, ..}) => {
                                        match &mut state.entities[*i].dead_or_alive {
                                            DeadOrAlive::Alive {zombie_or_human: ZombieOrHuman::Zombie { .. }, ref mut health } => {
                                                *health -= FIST_DAMAGE
                                            }
                                            _ => ()
                                        }
                                        p.velocity = Vector2::zero();
                                    }
                                }
                            }
                        }
                    }
                    _ => ()
                }
            }
            _ => panic!("Not a valid projectile")
        }
    }

    for i in 0..state.barricades.len() {
        if state.barricades[i].health <= 0.0 {
            state.barricades.remove(i);
        }
    }

    SimulationResults { entity_counts, sounds: sounds }
}

fn handle_collision(
    args: &UpdateArgs,
    entities: &mut Vec<Entity>,
    i: usize,
    j: usize,
    delta: &Vector2,
    delta_length_squared: f64) {

    // Spread the infection from zombies to others
    if entities[i].is_zombie() {
        match &mut entities[j].dead_or_alive {
            DeadOrAlive::Alive {zombie_or_human: ZombieOrHuman::Human { infection, .. }, ..} => {
                *infection += ZOMBIE_HUMAN_COLLISION_INFECTION_RATE
            }
            _ => ()
        }
    }
    if entities[j].is_zombie() {
        match &mut entities[i].dead_or_alive {
            DeadOrAlive::Alive {zombie_or_human: ZombieOrHuman::Human { infection, .. }, ..} => {
                *infection += ZOMBIE_HUMAN_COLLISION_INFECTION_RATE
            }
            _ => ()
        }
    }

    // Force entities apart that are overlapping
    let velocity_change = *delta * (args.dt / delta_length_squared);
    entities[i].velocity -= velocity_change;
    entities[j].velocity += velocity_change;
}

fn handle_building_collision(
    args: &UpdateArgs,
    entity: &mut Entity,
    building: &Polygon,
    inside: bool) {

    let mut distance_squared = INFINITY;
    let mut normal = Vector2::zero();
    let normals = building.normals();

    // Find the closest edge
    for i in 0..building.num_sides() {
        let seg_i = Segment2 {
            p1: building.get(i),
            p2: building.get((i + 1) % building.num_sides())
        };
        let dist_i = seg_i.distance_from_segment_to_point_squared(entity.position);

        if distance_squared > dist_i {
            distance_squared = dist_i;
            normal = normals[i];
        }
    }

    let distance = distance_squared.sqrt();

    if inside {
        // If the entity is inside move them to the nearest edge
        entity.position += (distance + ENTITY_RADIUS) * normal;
    } else {
        // If the entity is overlapping, force them away from the edge
        let overlap = ENTITY_RADIUS - distance;

        // Check if the entity is overlapping left/right borders, force them away if yes
        if building.get(0).x == -24.5 || building.get(0).x == 114.5 {
            entity.velocity -= args.dt * SPRING_CONSTANT * overlap * normal
        }
        else {
        entity.velocity += args.dt * SPRING_CONSTANT * overlap * normal}
    }
}

fn handle_barricade_collision(
    args: &UpdateArgs,
    entity: &mut Entity,
    barricade: &mut Barricade,
    inside: bool) {

    let mut distance_squared = INFINITY;
    let mut normal = Vector2::zero();
    let normals = barricade.poly.normals();

    for i in 0..barricade.poly.num_sides() {
        let seg_i = Segment2 {
            p1: barricade.poly.get(i),
            p2: barricade.poly.get((i + 1) % barricade.poly.num_sides())
        };
        let dist_i = seg_i.distance_from_segment_to_point_squared(entity.position);

        if distance_squared > dist_i {
            distance_squared = dist_i;
            normal = normals[i];
        }
    }

    let distance = distance_squared.sqrt();

    if inside {
        // If the entity is inside move them to the nearest edge
        entity.position += (distance + ENTITY_RADIUS) * normal;
    } else {
        // If the entity is overlapping, force them away from the edge
        let overlap = ENTITY_RADIUS - distance;
        let force: Vector2 = args.dt * SPRING_CONSTANT * overlap * normal;

        entity.velocity += force;

        // Only zombies damage barricades by running into them
        match &entity.dead_or_alive {
            DeadOrAlive::Alive { zombie_or_human, .. } => match zombie_or_human {
                ZombieOrHuman::Zombie { .. } => barricade.health -= force.length() * BARRICADE_HEALTH / 25.0,
                _ => ()
            },
            _ => ()
        }
    }
}

fn can_see(
    buildings: &Vec<Polygon>,
    from: Vector2,
    to: Vector2) -> bool {

    for building in buildings {
        let num_intersects = building.num_intersects(from, to);
        if num_intersects > 0 {
            return false;
        }
    };
    return true;
}

// Returns the index of the best target, if such a target exists
fn cop_find_best_target(sim_state: &mut State, cop_index: usize) -> Option<usize> {

    let entities = &sim_state.entities;

    let cop_position = entities[cop_index].position;

    let mut visible_entity_indices_by_distance_ascending = vec!();
    for i in 0..entities.len() {

        if i == cop_index {
            // Don't consider yourself
            continue;
        }

        if entities[i].is_dead() {
            // Don't consider corpses
            continue;
        }

        if !can_see(
            &sim_state.buildings,
            cop_position,
            entities[i].position) {
            // Don't consider entities you cannot see
            continue;
        }

        visible_entity_indices_by_distance_ascending.push(i);
    }

    visible_entity_indices_by_distance_ascending.sort_by(|a, b| {
        let distance_a = (entities[*a].position - cop_position).length();
        let distance_b = (entities[*b].position - cop_position).length();

        distance_b.partial_cmp(&distance_a).unwrap()
    });

    let mut best_target_score = -INFINITY;
    let mut best_target_index = None;

    // Consider each visible entity as a target
    for target_index in &visible_entity_indices_by_distance_ascending {
        if !entities[*target_index].is_zombie() {
            continue;
        }

        let vector_to_target_normal = (entities[*target_index].position - cop_position).normalize();

        let mut target_score = 0.0;

        // Consider each visible entity as a blocker
        for blocker_index in &visible_entity_indices_by_distance_ascending {

            let blocker_position = entities[*blocker_index].position;
            let vector_to_blocker = blocker_position - cop_position;
            let blocker_distance_squared = vector_to_blocker.length_squared();

            // Bit of an optimization to avoid a square root,
            // equivalent to coverage = cos(angle_of_blocker_from_target) / blocker_distance
            let blocker_coverage = vector_to_target_normal.dot(vector_to_blocker) / blocker_distance_squared;

            let blocker_score = match &entities[*blocker_index].dead_or_alive {

                // We want to target zombies
                DeadOrAlive::Alive { zombie_or_human: ZombieOrHuman::Zombie { .. }, .. } => 1.0,

                // We want to avoid hitting civilians
                DeadOrAlive::Alive {
                    zombie_or_human: ZombieOrHuman::Human {
                        human: Human::Civilian { .. },
                        .. },
                    ..
                } => -1.0,

                // We really want to avoid hitting cops
                DeadOrAlive::Alive {
                    zombie_or_human: ZombieOrHuman::Human {
                        human: Human::Cop { .. },
                        .. },
                    ..
                } => -2.0,

                _ => 0.0
            };

            target_score = (1.0 - blocker_coverage) * target_score + blocker_coverage * blocker_score;
        }
        if target_score > best_target_score {
            best_target_score = target_score;
            best_target_index = Some(*target_index);
        }
    }

    if best_target_score > 0.0 {
        best_target_index
    } else {
        None
    }
}

fn update_cop(
    args: &UpdateArgs,
    sim_state: &mut State,
    index: usize,
    sounds: &mut Vec<Sound>){

    let entities = &mut sim_state.entities;
    let buildings = &sim_state.buildings;
    let building_outlines = &sim_state.building_outlines;
    let barricades = &sim_state.barricades;

    enum StateChange {
        // Exit the state you're in
        Exit,
        // Continue in the current state unchanged
        Continue,
        // Update the state you're in
        Update(CopState),
        // Enter a new state
        Enter(CopState),
    }

    // Require unsafe to get second mutable reference
    let mut entity = unsafe { &mut *(&mut entities[index] as *mut Entity) };
    match &mut entity {
        Entity {
            position,
            dead_or_alive: DeadOrAlive::Alive {
                zombie_or_human: ZombieOrHuman::Human {
                    human: Human::Cop { cop_type, rounds_in_magazine, state_stack },
                    ..
                },
                ..
            },
            ..
        } => {
            let state_change = match state_stack.last() {
                Some(CopState::AttackingZombie { target_index, path: _ }) => {

                    if let DeadOrAlive::Dead = entities[*target_index].dead_or_alive {
                        // Target is dead, stop attacking
                        StateChange::Exit
                    }
                    else if *rounds_in_magazine <= 0 {
                        // Out of ammo, need to reload before we can attack
                        StateChange::Enter(
                            CopState::Reloading {
                                reload_time_remaining: cop_type.reload_time()
                            }
                        )
                    }
                    else if can_see(
                        &sim_state.buildings,
                        *position,
                        entities[*target_index].position) {
                        // Can see the target, take aim
                        let aim_time_distribution = Exp::new(cop_type.aim_time_mean());
                        StateChange::Enter(CopState::Aiming {
                            aim_time_remaining: aim_time_distribution.sample(&mut sim_state.rng),
                            target_index: *target_index,
                        })
                    }
                    else {
                        match find_path(entities[index].position, entities[*target_index].position,
                                        buildings, building_outlines, barricades) {
                            None => {
                                // No path to zombie possible, end chase
                                StateChange::Exit
                            },
                            Some(path) => {
                                match path.edges.first() {
                                    None =>
                                    // No path to zombie possible, end chase
                                        StateChange::Exit,
                                    Some(edge) => {
                                        let delta = edge.end.pos - entities[index].position;
                                        entities[index].accelerate_along_vector(delta, args.dt, COP_MOVEMENT_FORCE);
                                        StateChange::Update(CopState::AttackingZombie {
                                            target_index: *target_index,
                                            path: Some(path)
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
                Some(CopState::Aiming { aim_time_remaining, target_index }) => {

                    // Stop aiming if the target is already dead
                    if let DeadOrAlive::Dead = entities[*target_index].dead_or_alive {
                        StateChange::Exit
                    }

                    // Stop aiming if we can no longer see the target
                    else if !can_see(buildings,
                                entities[index].position,
                                entities[*target_index].position) {
                        StateChange::Exit
                    }
                    else {

                        let my_pos = entities[index].position;
                        let target_pos = entities[*target_index].position;
                        let delta = target_pos - my_pos;
                        entities[index].look_along_vector(delta, args.dt);

                        if *aim_time_remaining > args.dt {
                            // Taking aim, update the aim time
                            StateChange::Update(
                                CopState::Aiming { aim_time_remaining: *aim_time_remaining - args.dt, target_index: *target_index }
                            )
                        } else {
                            let angular_deviation =
                                Normal::new(0.0, cop_type.angular_accuracy_std_dev()).sample(&mut sim_state.rng);

                            // Finished aiming, take the shot
                            let delta_normal = delta.rotate_by(angular_deviation);

                            // Spawn outside of the entity - don't want to shoot the entity itself
                            let spawn_pos = entities[index].position +
                                BULLET_SPAWN_DISTANCE_MULTIPLIER * ENTITY_RADIUS * delta_normal;

                            // Fire at the target
                            sim_state.projectiles.push(
                                Projectile {
                                    position: spawn_pos,
                                    velocity: BULLET_SPEED * delta_normal,
                                    kind: ProjectileKind::Bullet
                                });

                            sim_state.projectiles.push(
                                Projectile {
                                    position: spawn_pos,
                                    // Casing ejects from the right of the weapon
                                    velocity: CASING_SPEED * delta_normal.right(),
                                    kind: ProjectileKind::Casing
                                });

                            *rounds_in_magazine -= 1;

                            sounds.push(Sound {
                                position: entities[index].position,
                                sound_type: match cop_type {
                                    CopType::Normal => SoundType::GunshotHandgun,
                                    CopType::Soldier => SoundType::GunshotRifle,
                                }
                            });
                            StateChange::Exit
                        }
                    }

                }
                Some(CopState::Moving { waypoint, mode, path: _ }) => {
                    match mode {
                        MoveMode::Moving => {
                            match find_path(entities[index].position, *waypoint,
                                            buildings, building_outlines, barricades) {
                                None => {
                                    StateChange::Exit
                                },
                                Some(path) => {
                                    match path.to_vec().get(1) {
                                        None => StateChange::Exit,
                                        Some(&node) => {
                                            let delta = node - entities[index].position;

                                            if *waypoint == node && delta.length_squared() < COP_MIN_DISTANCE_FROM_WAYPOINT_SQUARED {
                                                StateChange::Exit
                                            } else {
                                                entities[index].accelerate_along_vector(delta, args.dt, COP_MOVEMENT_FORCE);
                                                StateChange::Update(
                                                    CopState::Moving { waypoint: *waypoint, mode: MoveMode::Moving, path: Some(path) }
                                                )
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        MoveMode::Sprinting => {
                            // TODO:
                            StateChange::Exit
                        }
                    }
                }
                Some(CopState::Reloading { reload_time_remaining }) => {
                    let half_reload_time = 0.5 * cop_type.reload_time();
                    let new_reload_time_remaining = reload_time_remaining - args.dt;

                    // Play the reload sound when half-done reloading
                    if *reload_time_remaining > half_reload_time &&
                        half_reload_time > new_reload_time_remaining {
                            sounds.push(Sound{
                                position: entities[index].position,
                                sound_type: SoundType::Reload
                            });
                    }

                    if *reload_time_remaining > 0.0 {
                        StateChange::Update(CopState::Reloading {
                            reload_time_remaining: new_reload_time_remaining
                        })
                    } else {
                        // Finished reloading: replenish rounds and return to the previous state
                        *rounds_in_magazine = cop_type.magazine_capacity();
                        StateChange::Exit
                    }
                }
                None => {
                    // Reload if you don't have ammo
                    if *rounds_in_magazine <= 0 {
                        StateChange::Enter(CopState::Reloading {
                            reload_time_remaining: cop_type.reload_time() })
                    }
                    // Look for target if you do have ammo
                    else {
                        let target_index = cop_find_best_target(sim_state, index);

                        match target_index {
                            Some(i) => {
                                let aim_time_distribution = Exp::new(cop_type.aim_time_mean());
                                StateChange::Enter(CopState::Aiming {
                                    aim_time_remaining: aim_time_distribution.sample(&mut sim_state.rng),
                                    target_index: i,
                                })
                            },
                            None => {
                                // Remain in idle state
                                StateChange::Continue
                            }
                        }
                    }
                }
            };

            match state_change {
                StateChange::Exit => drop(state_stack.pop()),
                StateChange::Continue => (),
                StateChange::Update(new) => match state_stack.last_mut() {
                    None => state_stack.push(new),
                    Some(old) => *old = new,
                },
                StateChange::Enter(new) => state_stack.push(new)
            }
        }
        _ => panic!("Entity at index should be a cop!")
    }
}

fn update_zombie(
    args: &UpdateArgs,
    sim_state: &mut State,
    index: usize,
    state: ZombieState,
    _left_hand_status: &mut HandStatus,
    right_hand_status: &mut HandStatus) -> ZombieState {

    let entities = &mut sim_state.entities;
    let buildings = &sim_state.buildings;

    let my_pos = entities[index].position;

    match state {
        ZombieState::Chasing { target_index } => {
            let target_pos = entities[target_index].position;
            let delta = target_pos - my_pos;
            let can_see_target = can_see(buildings,my_pos,target_pos);

            // If target is within fighting range, fight
            if entities[target_index].is_human() && delta.x.abs() <= FIGHTING_RANGE && delta.y.abs() <= FIGHTING_RANGE && can_see_target {
                return ZombieState::Fighting { punch_time_remaining: PUNCH_TIME, target_index }
            }

            entities[index].accelerate_along_vector(delta, args.dt, ZOMBIE_MOVEMENT_FORCE);

            if entities[target_index].is_human() {
                if delta.length_squared() < ZOMBIE_SIGHT_RADIUS_SQUARE && can_see(buildings,my_pos,target_pos) {
                    // Continue chasing
                    ZombieState::Chasing { target_index: target_index }
                } else {
                    // Go to last known position
                    ZombieState::Moving { waypoint: target_pos }
                }
            }
            else {
                // Otherwise return to roaming
                ZombieState::Roaming {
                    jerk: Vector2::zero(),
                    acceleration: Vector2::zero()
                }
            }
        }
        ZombieState::Moving { waypoint } => {
            match closest_human(my_pos, entities, buildings) {
                // Continue moving
                None => {
                    let delta = waypoint - my_pos;
                    entities[index].accelerate_along_vector(delta, args.dt, ZOMBIE_MOVEMENT_FORCE);

                    if delta.length_squared() < COP_MIN_DISTANCE_FROM_WAYPOINT_SQUARED {
                        ZombieState::Roaming {
                            jerk: Vector2::zero(),
                            acceleration: Vector2::zero()
                        }
                    } else {
                        ZombieState::Moving { waypoint }
                    }
                },
                // Start chasing nearest human
                Some(i) => {
                    let delta = entities[i].position - my_pos;
                    entities[index].accelerate_along_vector(delta, args.dt, ZOMBIE_MOVEMENT_FORCE);
                    ZombieState::Chasing { target_index: i }
                }
            }
        }
        ZombieState::Roaming { jerk, acceleration } => {
            // Attempt to acquire a target
            match closest_human(my_pos, entities, buildings) {
                None => {
                    let normal = Normal::new(0.0, 5.0);
                    let delta_jerk = Vector2 {
                        x: normal.sample(&mut sim_state.rng),
                        y: normal.sample(&mut sim_state.rng)
                    } * args.dt;

                    let new_jerk = Vector2 {
                        x: (jerk.x + delta_jerk.x).max(-10.0).min(10.0),
                        y: (jerk.y + delta_jerk.y).max(-10.0).min(10.0)
                    };

                    let new_acceleration = Vector2 {
                        x: (acceleration.x + new_jerk.x * args.dt).max(-2.0).min(2.0),
                        y: (acceleration.y + new_jerk.y * args.dt).max(-2.0).min(2.0)
                    };

                    entities[index].look_along_vector(new_acceleration, args.dt);
                    entities[index].velocity += new_acceleration * args.dt / 5.0;

                    ZombieState::Roaming { jerk: new_jerk, acceleration: new_acceleration }
                },
                Some(i) => {
                    let delta = entities[i].position - my_pos;
                    entities[index].accelerate_along_vector(delta, args.dt, ZOMBIE_MOVEMENT_FORCE);
                    ZombieState::Chasing { target_index: i}
                }
            }
        }
        ZombieState::Fighting { punch_time_remaining, target_index } => {
            // Stop fighting if the target is already dead or zombie
            if entities[target_index].is_dead() || entities[target_index].is_zombie() {
                return ZombieState::Roaming {
                    jerk: Vector2::zero(),
                    acceleration: Vector2::zero()
                }
            }

            let target_pos = entities[target_index].position;
            let my_pos = entities[index].position;
            let delta = target_pos - my_pos;

            // Stop fighting if we can no longer see the target, go to last known position
            if !can_see(buildings,
                        entities[index].position,
                        entities[target_index].position) {
                return ZombieState::Moving { waypoint: target_pos }
            }

            // Stop fighting if we're not in fight range
            if delta.x.abs() > FIGHTING_RANGE && delta.y.abs() > FIGHTING_RANGE {
                return ZombieState::Chasing { target_index }
            }

            entities[index].look_along_vector(delta, args.dt);

            if punch_time_remaining > 0.0 {
                return ZombieState::Fighting { punch_time_remaining: punch_time_remaining - args.dt, target_index }
            } else {
                let angular_deviation =
                    Normal::new(0.0, ANGULAR_ACCURACY_STD_DEV).sample(&mut sim_state.rng);

                let delta_normal = delta.rotate_by(angular_deviation);

                // Spawn outside of the entity - don't want to punch the entity itself
                let spawn_pos = entities[index].position +
                    FIST_SPAWN_DISTANCE_MULTIPLIER * ENTITY_RADIUS * delta_normal;

                // Punch the target
                sim_state.projectiles.push(
                    Projectile {
                        position: spawn_pos,
                        velocity: FIST_SPEED * delta_normal,
                        kind: ProjectileKind::Fist { owner_index: index }
                    });

                *right_hand_status = HandStatus::None;

                ZombieState::Roaming {
                    jerk: Vector2::zero(),
                    acceleration: Vector2::zero()
                }
            }
        }
    }
}

// Get the index of the closest human in line of sight and sight radius
fn closest_human(my_pos: Vector2, entities: &Vec<Entity>, buildings: &Vec<Polygon>) -> Option<usize> {
    let mut min_distance_sqr = INFINITY;
    let mut closest_index: Option<usize> = None;

    for i in 0..entities.len() {
        if entities[i].is_human() {
            let delta_squared = (my_pos - entities[i].position).length_squared();
            if delta_squared < ZOMBIE_SIGHT_RADIUS_SQUARE &&
                can_see(buildings, my_pos, entities[i].position) &&
                delta_squared < min_distance_sqr {

                    min_distance_sqr = delta_squared;
                    closest_index = Some(i);
                }
        }
    }

    closest_index
}

fn simulate_human(args: &UpdateArgs,
                  sim_state: &mut State,
                  index: usize,
                  punch_time_cooldown: &mut Scalar,
                  state: HumanState,
                  left_hand_status: &mut HandStatus,
                  right_hand_status: &mut HandStatus) -> HumanState {

    let mut min_delta = Vector2::zero();
    let mut min_distance_sqr = INFINITY;
    let entities = &mut sim_state.entities;
    let buildings = &sim_state.buildings;
    let my_pos = entities[index].position;

    match state {
        HumanState::Running => {
            let mut zombie_index = 0;
            for i in 0..entities.len() {
                if entities[i].is_zombie() {
                    // Run from zombies
                    let delta = entities[i].position - my_pos;
                    let distance_sqr = delta.length_squared();
                    if distance_sqr < HUMAN_SIGHT_RADIUS_SQUARE &&
                        can_see(buildings, my_pos, entities[i].position) &&
                        distance_sqr < min_distance_sqr {

                        min_delta = delta;
                        min_distance_sqr = distance_sqr;
                        zombie_index = i;
                    }
                }
            }

            // If zombie is close
            if min_delta.x.abs() <= FIGHTING_RANGE && min_delta.y.abs() <= FIGHTING_RANGE && *punch_time_cooldown <= 0.0 {
                return HumanState::Fighting { target_index: zombie_index, punch_time_remaining: PUNCH_TIME }
            }

            *punch_time_cooldown -= args.dt;

            // else run
            if min_distance_sqr < INFINITY {
                // Accelerate away from the nearest zombie
                entities[index].accelerate_along_vector(-min_delta, args.dt, CIVILIAN_MOVEMENT_FORCE);
            }
            return HumanState::Running
        }
        HumanState::Fighting { target_index, punch_time_remaining } => {
            // Stop fighting if the target is already dead
            if entities[target_index].is_dead() {
                return HumanState::Running
            }

            let target_pos = entities[target_index].position;
            let my_pos = entities[index].position;
            let delta = target_pos - my_pos;

            // Stop fighting if we can no longer see the target
            if !can_see(buildings,
                        entities[index].position,
                        entities[target_index].position) {
                return HumanState::Running
            }

            // Stop fighting if we're not in fight range
            if delta.x.abs() > FIGHTING_RANGE && delta.y.abs() > FIGHTING_RANGE {
                return HumanState::Running
            }

            entities[index].look_along_vector(delta, args.dt);

            if punch_time_remaining > 0.0 {
                return HumanState::Fighting { punch_time_remaining: punch_time_remaining - args.dt, target_index }
            } else {
                let angular_deviation =
                    Normal::new(0.0, ANGULAR_ACCURACY_STD_DEV).sample(&mut sim_state.rng);

                let delta_normal = delta.rotate_by(angular_deviation);

                // Spawn outside of the entity - don't want to punch the entity itself
                let spawn_pos = entities[index].position +
                    FIST_SPAWN_DISTANCE_MULTIPLIER * ENTITY_RADIUS * delta_normal;

                // Punch the target
                sim_state.projectiles.push(
                    Projectile {
                        position: spawn_pos,
                        velocity: FIST_SPEED * delta_normal,
                        kind: ProjectileKind::Fist { owner_index: index }
                    });

                *right_hand_status = HandStatus::None;
                *punch_time_cooldown = PUNCH_TIME_COOLDOWN;

                HumanState::Running
            }
        }
    }
}
