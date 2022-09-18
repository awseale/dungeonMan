  //represents a 5ft x 5ft square utilized by most modern TTRPGs
pub struct Tile {
	hasPlayer: bool, //represents whether a player character occupies current tile
	hasMonster: bool, //represents whether a monster (or any hostile NPC) occupies current tile
	hasNPC: bool //represents whether a nonhostile NPC occupies current tile
	isOccupied: bool //represents whether a tile is currently occupied
	canOccupy: bool //represents whether it is possible to occupy a given tile
}

impl Tile {
	pub fn hasPlayer(&self) -> bool {
		return self.hasPlayer;
	}

	pub fn hasMonster(&self) -> bool {
		return self.hasMonster;
	}

	pub fn hasNPC(&self) -> bool {
		return self.hasNPC;
	}

	pub fn isOccupied(&self) -> bool {
		return self.isOccupied;
	}

	pub fn canOccupy(&self) -> bool {
		return self.canOccupy;
	}
}
