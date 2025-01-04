import { BottomLayoutItems } from "@/lib/bottomBoxTypes";
import { LayoutUpdateMessage } from "@/lib/layoutUpdateTypes";
import { useEffect } from "react";
import { create } from "zustand";
import { TwitchChatMessage, useChatStore } from "./chatStore";
import { WebSocketService } from "./webSocketSvc";

interface LayoutStoreState {
  layoutItems: BottomLayoutItems[];
  setLayoutItems: (items: BottomLayoutItems[]) => void;
  updateLayoutItems: (message: LayoutUpdateMessage) => void;
  wsService: WebSocketService | null;
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
  wsService: null,
  initWebSocket: () => {
    if (!get().wsService) {
      const messageHandler = (message: TwitchChatMessage) => {
        set((state) => {
          const updatedMessages = [...state.messages, message];
          return {
            messages:
              updatedMessages.length > 30
                ? updatedMessages.slice(1)
                : updatedMessages,
          };
        });
      };

      const wsService = new WebSocketService(messageHandler);
      wsService.connect();
      set({ wsService });
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
  const { initWebSocket, wsService } = useLayoutStore();

  // useEffect to run the WebSocket initializer once on mount
  useEffect(() => {
    initWebSocket();
    return () => {
      wsService?.disconnect();
    };
  }, [initWebSocket, wsService]);
};
