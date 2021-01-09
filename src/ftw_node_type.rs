use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

macro_rules! generate_ftw_node_types {
    ($($i:ident), *) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FtwNodeType {
            $($i,)*
        }

        impl FromStr for FtwNodeType {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($i) => Ok(FtwNodeType::$i),)*
                    _ => Ok(FtwNodeType::Node),
                }
            }
        }

        impl Display for FtwNodeType {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let node_type: &str = match self {
                    $(FtwNodeType::$i => stringify!($i),)*
                };
                write!(f, "{}", node_type)
            }
        }
    };
}

generate_ftw_node_types![
    AnimatedSprite,
    AnimationPlayer,
    Area2D,
    AudioStreamPlayer2D,
    Camera2D,
    CanvasLayer,
    CollisionPolygon2D,
    CollisionShape2D,
    Control,
    KinematicBody2D,
    Navigation,
    Node,
    Node2D,
    ParallaxBackground,
    Particles,
    Path2D,
    PathFollow2D,
    RigidBody2D,
    Spatial,
    Sprite,
    StaticBody2D,
    TileMap,
    Timer,
    Tween,
    YSort
];
