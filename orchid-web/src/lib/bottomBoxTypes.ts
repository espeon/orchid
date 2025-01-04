// Enum for Pokémon Types
export enum PkType {
  Bug = "Bug",
  Dark = "Dark",
  Dragon = "Dragon",
  Electric = "Electric",
  Fairy = "Fairy",
  Fighting = "Fighting",
  Fire = "Fire",
  Flying = "Flying",
  Ghost = "Ghost",
  Grass = "Grass",
  Ground = "Ground",
  Ice = "Ice",
  Normal = "Normal",
  Poison = "Poison",
  Psychic = "Psychic",
  Rock = "Rock",
  Steel = "Steel",
  Water = "Water",
}

// Enum for BottomLayoutItems with specific type structure for each variant
export type BottomLayoutItems =
  | { type: "PkTeamLayout"; id: number }
  | { type: "PkBadgeLayout"; id: number };

// Interface for a Pokémon
export interface Pokemon {
  rowid?: number;
  name: string;
  image: string;
  description: string;
  types: PkType[];
  abilities: string[];
  height: number;
  weight: number;
  gender: string;
  shiny: boolean;
}

// Interface for a Team of Pokémon (PkTeam)
export interface PkTeam {
  rowid?: number;
  pokemon_ids: number[];
  team_id: number;
}

// Interface for a Badge
export interface PkBadge {
  rowid?: number;
  name: string;
  image: string;
  obtained: boolean;
}

// Interface for Badge Collection
export interface PkBadgeCollection {
  rowid?: number;
  badge_ids: number[];
}

// Interface for Bottom Layout
export interface BottomLayout {
  rowid?: number;
  layout_items: BottomLayoutItems[];
}
