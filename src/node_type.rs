use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeType {
    Node,
    Node2D,
    AnimatedSprite,
    AnimationPlayer,
    Area2D,
    AudioStreamPlayer2D,
    Camera2D,
    CanvasLayer,
    CollisionPolygon2D,
    CollisionShape2D,
    KinematicBody2D,
    Navigation,
    ParallaxBackground,
    Particles,
    Path2D,
    PathFollow2D,
    RigidBody2D,
    Sprite,
    StaticBody2D,
    TileMap,
    Timer,
    Tween,
    YSort,
}

impl FromStr for NodeType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "node" => Ok(NodeType::Node),
            "node2d" => Ok(NodeType::Node2D),
            "animatedsprite" => Ok(NodeType::AnimatedSprite),
            "animationplayer" => Ok(NodeType::AnimationPlayer),
            "area2d" => Ok(NodeType::Area2D),
            "audiostreamplayer2d" => Ok(NodeType::AudioStreamPlayer2D),
            "camera2d" => Ok(NodeType::Camera2D),
            "canvaslayer" => Ok(NodeType::CanvasLayer),
            "collisionpolygon2d" => Ok(NodeType::CollisionPolygon2D),
            "collisionshape2d" => Ok(NodeType::CollisionShape2D),
            "kinematicbody2d" => Ok(NodeType::KinematicBody2D),
            "navigation" => Ok(NodeType::Navigation),
            "parallaxbackground" => Ok(NodeType::ParallaxBackground),
            "particles" => Ok(NodeType::Particles),
            "path2d" => Ok(NodeType::Path2D),
            "pathfollow2d" => Ok(NodeType::PathFollow2D),
            "rigidbody2d" => Ok(NodeType::RigidBody2D),
            "sprite" => Ok(NodeType::Sprite),
            "staticbody2d" => Ok(NodeType::StaticBody2D),
            "tilemap" => Ok(NodeType::TileMap),
            "timer" => Ok(NodeType::Timer),
            "tween" => Ok(NodeType::Tween),
            "ysort" => Ok(NodeType::YSort),
            _ => Ok(NodeType::Node),
        }
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let node_type: &str = match self {
            NodeType::Node => "Node",
            NodeType::Node2D => "Node2D",
            NodeType::AnimatedSprite => "AnimatedSprite",
            NodeType::AnimationPlayer => "AnimationPlayer",
            NodeType::Area2D => "Area2D",
            NodeType::AudioStreamPlayer2D => "AudioStreamPlayer2D",
            NodeType::Camera2D => "Camera2D",
            NodeType::CanvasLayer => "CanvasLayer",
            NodeType::CollisionPolygon2D => "CollisionPolygon2D",
            NodeType::CollisionShape2D => "CollisionShape2D",
            NodeType::KinematicBody2D => "KinematicBody2D",
            NodeType::Navigation => "Navigation",
            NodeType::ParallaxBackground => "ParallaxBackground",
            NodeType::Particles => "Particles",
            NodeType::Path2D => "Path2D",
            NodeType::PathFollow2D => "PathFollow2D",
            NodeType::RigidBody2D => "RigidBody2D",
            NodeType::Sprite => "Sprite",
            NodeType::StaticBody2D => "StaticBody2D",
            NodeType::TileMap => "TileMap",
            NodeType::Timer => "Timer",
            NodeType::Tween => "Tween",
            NodeType::YSort => "YSort",
        };
        write!(f, "{}", node_type)
    }
}
