use self::{bullet::Bullet, coin::Coin, player::Player, tree::Tree, wood::Wood, zombie::Zombie};

pub mod bullet;
pub mod player;
pub mod zombie;
pub mod coin;
pub mod wood;
pub mod tree;

pub enum Entity {
    Bullet(Bullet),
    Player(Player),
    Zombie(Zombie),
    Coin(Coin),
    Wood(Wood),
    Tree(Tree)
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