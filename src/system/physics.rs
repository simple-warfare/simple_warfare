use std::default;
use bevy::{ecs::entity, prelude::*};
use avian2d::{collision::contact_types::*, parry::query,prelude::*};
use tokio::time;
use crate::{
    custom::{
        unit::{
            section::{graphic::Graphic},
        },
    },
};
pub struct PhysicsSystemPlugin;



#[derive(Component, Debug,Default)]
pub struct LinearSpeed(pub f32);

#[derive(Component, Debug,Default)]
pub struct LinearOffset(pub Vec2);

#[derive(Component, Debug,Default)]
pub struct AngularSpeed(pub f32);

#[derive(Component, Debug,Default)]
pub struct AngularOffset(pub f32);




impl Plugin for PhysicsSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate,(fixed_physics_interpolate,handle_velocity,handle_collision).chain());
        app.add_systems(Update, handle_physics_interpolate);
    }
}

//处理碰撞
fn handle_collision(collisions: Collisions,mut query: Query<(&mut Transform,&Mass)>) {
    for collision in collisions.iter() {
        // println!("{:?}", collision);
        if let Some(entity_1) = collision.body1 && let Some(entity_2) = collision.body2{
            let entity_1=entity_1;
            let entity_2=entity_2;
            let entitys:[Entity;2]=[entity_1,entity_2];
            if let Ok([(mut transform_1, mass_1), (mut transform_2, mass_2)]) = 
                query.get_many_mut(entitys) 
            {
                //防止重叠
                if transform_1.translation == transform_2.translation {
                    transform_1.translation.y += 1.0;
                    transform_2.translation.y -= 1.0;
                }
                else{
                    let mut penetration=collision.manifolds[0].points[0].penetration*0.3;
                    if penetration<0.1{
                        penetration=0.1;
                    }
                    let force = collision.manifolds[0].normal * (mass_1.0 + mass_2.0) * penetration;
                    let offset1=force / mass_1.0;
                    let offset2=force / mass_2.0;
                    transform_1.translation.x -= offset1.x;
                    transform_1.translation.y -= offset1.y;

                    transform_2.translation.x += offset2.x;
                    transform_2.translation.y += offset2.y;
                }
            }
        }
    }
}




//还原物理插帧的修改
fn fixed_physics_interpolate(
    mut graphics_query: Query<(&Graphic, &mut Transform, &ChildOf)>,
    fixed_time: Res<Time<Fixed>>,
    time: Res<Time>,
){
    let fixed_step= fixed_time.timestep().as_secs_f32();
    for (graphic, mut transform, parent) in graphics_query.iter_mut() {
        transform.translation.x=0.;
        transform.translation.y=0.;
    }
}


//处理物理插帧
fn handle_physics_interpolate(
    mut graphics_query: Query<(&Graphic, &mut Transform, &ChildOf)>,
    graphics_parent_query: Query<&LinearOffset,Without<Graphic>>,
    fixed_time: Res<Time<Fixed>>,
    time: Res<Time>,
){
    let fixed_step= fixed_time.timestep().as_secs_f32();
    let step= time.delta_secs()/fixed_step;
    for (graphic, mut transform, parent) in graphics_query.iter_mut() {
        if let Ok(parent_linear_offset)=graphics_parent_query.get(parent.0){
            let offset=parent_linear_offset.0*step;
            transform.translation.x+=offset.x;
            transform.translation.y+=offset.y;
        }
    }
}


fn handle_velocity(
    mut query: Query<(&mut Transform, &mut LinearOffset, &mut AngularOffset,&Mass)>,
) {
    for (mut transform, mut linear_offset,mut angular_offset, mass) in query.iter_mut() {
        transform.translation.x+=linear_offset.0.x;
        transform.translation.y+=linear_offset.0.y;

        transform.rotate_z(angular_offset.0);
        angular_offset.0=0.;

        // linear_offset.0= Vec2::new(0.0, 0.0);
    }
}