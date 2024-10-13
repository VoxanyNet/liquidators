use diff::Diff;
use gamelibrary::{rapier_to_macroquad, space::Space};
use macroquad::{color::BLUE, math::Vec2, shapes::draw_circle};
use nalgebra::vector;
use rapier2d::prelude::{ColliderHandle, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{level::Level, portal::Portal, structure::{self, Structure}};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct PortalBullet {
    pub position: Vec2,
    pub direction: Vec2
}

impl PortalBullet {

    pub fn tick(mut self, level: &mut Level) -> Option<PortalBullet> {
        self.position.x += self.direction.x * 4.;
        self.position.y += self.direction.y * 4.;

        let bullet = self.attempt_portal_spawn(level);

        return bullet;
        
    }

    fn attempt_portal_spawn(self, level: &mut Level) -> Option<Self> {

        let mut portal_structure: Option<&Structure> = None;

        level.space.query_pipeline.intersections_with_point(
            &level.space.rigid_body_set, 
            &level.space.collider_set, 
            &vector![self.position.x, self.position.y].into(), 
            QueryFilter::default(), 
            |collider_handle| {
                // look for corresponding structure
                for structure in &level.structures {
                    if structure.collider_handle == collider_handle {
                        portal_structure = Some(structure);

                        return false
                    }

                }

                return true
            }
        );

        match portal_structure {
            Some(portal_structure) => {

                let structure_body = level.space.rigid_body_set.get(portal_structure.rigid_body_handle).unwrap();

                let portal = Portal {
                    attached_collider: portal_structure.collider_handle.clone(),
                };

                level.portals.push(portal);

                return None
            },
            None => {
                return Some(self)
            },
        }

        
    }
    pub async fn draw(&self) {

        let macroquad_pos = rapier_to_macroquad(&self.position);

        draw_circle(macroquad_pos.x, macroquad_pos.y, 7., BLUE);
    }
}