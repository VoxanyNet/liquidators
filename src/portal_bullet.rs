use diff::Diff;
use gamelibrary::rapier_to_macroquad;
use macroquad::{color::BLUE, math::Vec2, shapes::draw_circle};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct PortalBullet {
    pub position: Vec2,
    pub direction: Vec2
}

impl PortalBullet {

    pub fn tick(&mut self) {
        self.position.x += self.direction.x * 4.;
        self.position.y += self.direction.y * 4.;

        //let bullet = self.attempt_portal_spawn(level);
        
    }

    // fn attempt_portal_spawn(self, portals: &mut Vec<Portal>, space: &mut Space, structures: &mut Vec<Structure>) -> Option<Self> {

    //     let mut portal_structure: Option<&Structure> = None;

    //     space.query_pipeline.intersections_with_point(
    //         &space.rigid_body_set, 
    //         &space.collider_set, 
    //         &vector![self.position.x, self.position.y].into(), 
    //         QueryFilter::default(), 
    //         |collider_handle| {
    //             // look for corresponding structure
    //             for structure in structures {
    //                 if structure.collider_handle == collider_handle {
    //                     portal_structure = Some(structure);

    //                     return false
    //                 }

    //             }

    //             return true
    //         }
    //     );

    //     match portal_structure {
    //         Some(portal_structure) => {

    //             let _structure_body = space.rigid_body_set.get(portal_structure.rigid_body_handle).unwrap();

    //             let portal = Portal {
    //                 attached_collider: portal_structure.collider_handle.clone(),
    //             };

    //             portals.push(portal);

    //             return None
    //         },
    //         None => {
    //             return Some(self)
    //         },
    //     }

        
    // }
    pub async fn draw(&self) {

        let macroquad_pos = rapier_to_macroquad(&self.position);

        draw_circle(macroquad_pos.x, macroquad_pos.y, 7., BLUE);
    }
}