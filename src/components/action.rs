use bevy::prelude::*;
use phf::Map;
use serde::Deserialize;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

use crate::constants::action::*;
use crate::constants::map::TILE;

/// Represents the direction of an action (e.g., Up, Down, Left, Right).
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Display, Deserialize, EnumString)]
pub enum ActionDirection {
    #[strum(ascii_case_insensitive)]
    Up,
    #[strum(ascii_case_insensitive)]
    Down,
    #[strum(ascii_case_insensitive)]
    Left,
    #[strum(ascii_case_insensitive)]
    Right,
}

/// Represents the kind of an action (e.g., Stand, Walk, Run).
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Display, Deserialize, EnumString)]
pub enum ActionKind {
    #[strum(ascii_case_insensitive)]
    Stand,
    #[strum(ascii_case_insensitive)]
    Walk,
    #[strum(ascii_case_insensitive)]
    Run,
    #[strum(ascii_case_insensitive)]
    Type,
    // Add future actions here
}

/// Represents an action with a kind and direction.
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Action {
    pub kind: ActionKind,
    pub direction: ActionDirection,
}

impl Action {
    pub const fn new(kind: ActionKind, direction: ActionDirection) -> Self {
        Self { kind, direction }
    }
    pub const fn get_transformation(&self) -> Vec3 {
        let i_vector = self.get_raw_transformation();
        let vector = Vec2::new(
            (i_vector.x * TILE as i32) as f32,
            (i_vector.y * TILE as i32) as f32,
        );
        Vec3::new(vector.x, vector.y, 0.)
    }

    pub const fn get_raw_transformation(&self) -> IVec2 {
        let norm = match self.kind {
            ActionKind::Walk => WALK_RATE as i32,
            ActionKind::Run => RUN_RATE as i32,
            _ => 0_i32,
        };

        match self.direction {
            ActionDirection::Up => IVec2::new(0, norm),
            ActionDirection::Down => IVec2::new(0, -norm),
            ActionDirection::Left => IVec2::new(-norm, 0),
            ActionDirection::Right => IVec2::new(norm, 0),
        }
    }

    /// Parses a command string and returns a vector of actions.
    ///
    /// # Arguments
    /// * `commands` - A command string containing one or more actions.
    ///
    /// # Returns
    /// A `Vec` of `Action` instances parsed from the command string.
    pub fn from_command_string(commands: &str) -> Option<Vec<Action>> {
        let uppercase = commands.to_uppercase();
        let mut actions = Vec::new();
        for line in uppercase.lines() {
            if let Some(mut line_actions) = Self::from_single_command_string(line) {
                actions.append(&mut line_actions);
            }
        }
        if actions.is_empty() {
            None
        } else {
            Some(actions)
        }
    }

    fn from_single_command_string(command: &str) -> Option<Vec<Action>> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() < 2 || parts.len() > 3 {
            return None;
        }

        let mut kind = ActionKind::from_str(parts[0]).ok()?;
        let direction = ActionDirection::from_str(parts[1]).ok()?;

        let times = if parts.len() == 3 {
            parts[2].parse::<usize>().ok()?
        } else {
            1
        };

        if times > 5 && kind == ActionKind::Walk {
            kind = ActionKind::Run
        }

        Some(vec![Action { kind, direction }; times])
    }
}

/// Check and handle action duration
#[derive(Component, Deref, DerefMut)]
pub struct ActionTimer(pub Timer);

/// Describe entities action duration
#[derive(Component)]
pub struct ActionDurationPHF(pub Map<&'static str, f32>);

impl ActionDurationPHF {
    pub fn lookup(&self, action: &Action) -> f32 {
        *self
            .0
            .get(&action.kind.to_string())
            .unwrap_or_else(|| panic!("Unable to lookup {:?} in ActionDurationPHF!", action))
    }

    pub fn generate_timer(&self, action: &Action) -> ActionTimer {
        ActionTimer(Timer::from_seconds(self.lookup(action), TimerMode::Once))
    }
}
