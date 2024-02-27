use self::{bullet::Bullet, player::Player, zombie::Zombie};

pub mod bullet;
pub mod player;
pub mod zombie;

pub enum Entity {
    Bullet(Bullet),
    Player(Player),
    Zombie(Zombie)
}