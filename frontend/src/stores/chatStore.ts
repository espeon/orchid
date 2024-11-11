// src/store/chatStore.ts
import { create } from "zustand";
import randomMessages from "../lib/exampleMsgs";

interface ChatMessage {
  message: string;
  user: string;
}

interface ChatStore {
  messages: ChatMessage[];
  addMessage: (message: ChatMessage) => void;
  initializeMessages: () => void;
}

const initialMessages = [
  randomMessages[Math.floor(Math.random() * randomMessages.length)],
  randomMessages[Math.floor(Math.random() * randomMessages.length)],
  randomMessages[Math.floor(Math.random() * randomMessages.length)],
];

export const useChatStore = create<ChatStore>((set) => ({
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
  initializeMessages: () => set({ messages: initialMessages }),
}));
