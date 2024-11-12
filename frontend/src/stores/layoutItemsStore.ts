import { BottomLayoutItems } from "@/lib/bottomBoxTypes";
import { LayoutUpdateMessage } from "@/lib/layoutUpdateTypes";
import { useEffect } from "react";
import { create } from "zustand";
import { TwitchChatMessage, useChatStore } from "./chatStore";

interface LayoutStoreState {
  layoutItems: BottomLayoutItems[];
  setLayoutItems: (items: BottomLayoutItems[]) => void;
  updateLayoutItems: (message: LayoutUpdateMessage) => void;
  wsConnection: WebSocket | null;
  initWebSocket: () => void;
  messages: TwitchChatMessage[];
  addMessage: (message: TwitchChatMessage) => void;
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
      const ws = new WebSocket("ws://localhost:3000/ws");

      ws.addEventListener("open", () => {
        console.log("WebSocket connection established");
      });

      ws.addEventListener("message", (event) => {
        try {
          if (event.data.startsWith("{")) {
            const data: TwitchChatMessage = JSON.parse(event.data);

            if (
              data.msgType === "PRIVMSG" &&
              data.user?.userName &&
              data.message
            ) {
              console.log(`${data.user.userName}: ${data.message}`);
              set((state) => {
                const updatedMessages = [...state.messages, data];
                return {
                  messages:
                    updatedMessages.length > 30
                      ? updatedMessages.slice(1)
                      : updatedMessages,
                };
              });
            }
          }
        } catch (error) {
          console.error("Error parsing WebSocket message:", error);
        }
      });

      ws.addEventListener("close", () => {
        console.warn("WebSocket connection closed. Attempting to reconnect...");
        set({ wsConnection: null });
      });

      ws.addEventListener("error", (error) => {
        console.error("WebSocket error:", error);
      });

      set({ wsConnection: ws });
    }
  },
  messages: [],
  addMessage: (message) =>
    set((state) => {
      const updatedMessages = [...state.messages, message];
      return {
        messages:
          updatedMessages.length > 30
            ? updatedMessages.slice(1)
            : updatedMessages,
      };
    }),
}));

// Custom hook to initialize the WebSocket connection
export const useInitializeWebSocket = () => {
  const initWebSocket = useLayoutStore((state) => state.initWebSocket);

  // useEffect to run the WebSocket initializer once on mount
  useEffect(() => {
    initWebSocket();
  }, [initWebSocket]);
};
