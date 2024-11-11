import { BottomLayoutItems } from "@/lib/bottomBoxTypes";
import { LayoutUpdateMessage } from "@/lib/layoutUpdateTypes";
import { useEffect } from "react";
import { create } from "zustand";

interface LayoutStoreState {
  layoutItems: BottomLayoutItems[];
  setLayoutItems: (items: BottomLayoutItems[]) => void;
  updateLayoutItems: (message: LayoutUpdateMessage) => void;
  wsConnection: WebSocket | null;
  initWebSocket: () => void;
}

export const useLayoutStore = create<LayoutStoreState>()((set, get) => ({
  layoutItems: [],
  setLayoutItems: (items) => set({ layoutItems: items }),
  updateLayoutItems: (message: LayoutUpdateMessage) => {
    set((state) => {
      const { action, item, id, order } = message;

      switch (action) {
        case "add":
          if (item) return { layoutItems: [...state.layoutItems, item] };
          break;

        case "remove":
          if (id !== undefined)
            return {
              layoutItems: state.layoutItems.filter(
                (layoutItem) => layoutItem.id !== id,
              ),
            };
          break;

        case "update":
          if (item)
            return {
              layoutItems: state.layoutItems.map((layoutItem) =>
                layoutItem.id === item.id ? item : layoutItem,
              ),
            };
          break;

        case "reorder":
          if (order)
            return {
              layoutItems: order
                .map((id) => state.layoutItems.find((item) => item.id === id))
                .filter(Boolean) as BottomLayoutItems[],
            };
          break;

        default:
          console.warn("Unknown action:", action);
      }
      return {};
    });
  },
  wsConnection: null,
  initWebSocket: () => {
    if (!get().wsConnection) {
      // Check if a WebSocket connection already exists
      const ws = new WebSocket("ws://localhost:3000/ws");

      ws.addEventListener("open", () => {
        console.log("WebSocket connection established");
      });

      ws.addEventListener("message", (event) => {
        let data = null;
        let dataType = "string";
        try {
          if (event.data.startsWith("{")) {
            data = JSON.parse(event.data);
            dataType = "json";
            // TODO: better handle incoming data portions
            set({ layoutItems: data.layout_items });
          } else {
            data = event.data;
          }
          console.log("Received", dataType, "data:", data);
        } catch (error) {
          console.error("Error parsing WebSocket message:", error);
        }
      });

      ws.addEventListener("close", () => {
        console.warn("WebSocket connection closed. Attempting to reconnect...");
        set({ wsConnection: null }); // Reset connection in store
      });

      ws.addEventListener("error", (error) => {
        console.error("WebSocket error:", error);
      });

      set({ wsConnection: ws });
    }
  },
}));

// Custom hook to initialize the WebSocket connection
export const useInitializeWebSocket = () => {
  const initWebSocket = useLayoutStore((state) => state.initWebSocket);

  // useEffect to run the WebSocket initializer once on mount
  useEffect(() => {
    initWebSocket();
  }, [initWebSocket]);
};
