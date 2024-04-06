use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::game::{HasOwner, Tickable};

use self::{bullet::Bullet, coin::Coin, player::Player, tree::Tree, wood::Wood, zombie::Zombie};

pub mod bullet;
pub mod player;
pub mod zombie;
pub mod coin;
pub mod wood;
pub mod tree;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub enum Entity {
    Bullet(Bullet),
    Player(Player),
    Zombie(Zombie),
    Coin(Coin),
    Wood(Wood),
    Tree(Tree)
}

impl HasOwner for Entity {
    fn get_owner(&self) -> String {

        match self {
            Entity::Bullet(bullet) => bullet.get_owner(),
            Entity::Player(player) => player.get_owner(),
            Entity::Zombie(zombie) => zombie.get_owner(),
            Entity::Coin(coin) => coin.get_owner(),
            Entity::Wood(wood) => wood.get_owner(),
            Entity::Tree(tree) => tree.get_owner(),
        }
    }

    fn set_owner(&mut self, uuid: String) {
        match self {
            Entity::Bullet(bullet) => bullet.owner = uuid,
            Entity::Player(player) => player.owner = uuid,
            Entity::Zombie(zombie) => zombie.owner = uuid,
            Entity::Coin(coin) => coin.owner = uuid,
            Entity::Wood(wood) => wood.owner = uuid,
            Entity::Tree(tree) => tree.owner = uuid,
        }
    }
}

impl Tickable for Entity {
    fn tick(&mut self, game: &mut crate::game::TickContext) {
        match self {
            Entity::Bullet(bullet) => bullet.tick(game),
            Entity::Player(player) => player.tick(game),
            Entity::Zombie(zombie) => zombie.tick(game),
            Entity::Coin(coin) => coin.tick(game),
            Entity::Wood(wood) => wood.tick(game),
            Entity::Tree(tree) => tree.tick(game),
        }
    }
}

impl From<Player> for Entity {
    fn from(value: Player) -> Self {
        Self::Player(value)
    }
}
impl From<Tree> for Entity {
    fn from(value: Tree) -> Self {
        Self::Tree(value)
    }
}
impl From<Bullet> for Entity {
    fn from(value: Bullet) -> Self {
        Self::Bullet(value)
    }
}

impl From<Zombie> for Entity {
    fn from(value: Zombie) -> Self {
        Self::Zombie(value)
    }
}

impl From<Coin> for Entity {
    fn from(value: Coin) -> Self {
        Self::Coin(value)
    }
}

impl From<Wood> for Entity {
    fn from(value: Wood) -> Self {
        Self::Wood(value)
    }
}