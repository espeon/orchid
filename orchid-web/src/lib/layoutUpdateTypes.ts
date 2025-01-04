import { BottomLayoutItems } from "@/lib/bottomBoxTypes";

export type LayoutAction = "add" | "remove" | "update" | "reorder";

// Define the structure of each WebSocket message for incremental updates
export interface LayoutUpdateMessage {
  action: LayoutAction;
  item?: BottomLayoutItems; // Used for add/update actions
  id?: number; // Used for remove actions
  order?: number[]; // Used for reorder action (array of item IDs in new order)
}
