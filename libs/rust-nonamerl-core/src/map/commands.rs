use crate::{IntVector2, Map, Tile, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapCommand {
    SetVisited(IntVector2, bool),
    SetVisible(IntVector2, bool),
}

#[derive(Debug, Clone)]
pub struct MapCommands {
    pub commands: Vec<MapCommand>,
}

impl MapCommands {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add(&mut self, command: MapCommand) {
        self.commands.push(command);
    }

    pub fn add_all(&mut self, commands: Vec<MapCommand>) {
        self.commands.extend(commands);
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn process_commands<T: Tile>(&mut self, map: &mut Map<T>) {
        for command in self.commands.iter() {
            match command {
                MapCommand::SetVisited(pos, visited) => {
                    map.set_visited(pos.x(), pos.y(), *visited);
                }
                MapCommand::SetVisible(pos, visible) => {
                    map.set_visible(pos.x(), pos.y(), *visible);
                }
            }
        }
        self.commands.clear();
    }
}

impl Default for MapCommands {
    fn default() -> Self {
        Self::new()
    }
}
