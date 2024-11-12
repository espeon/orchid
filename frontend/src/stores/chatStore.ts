// src/store/chatStore.ts
import { create } from "zustand";
import randomMessages from "../lib/exampleMsgs";

export interface TwitchChatMessage {
  msgType: string;
  // The channel name the message was sent in
  channel: string;
  // The channel ID the message was sent in
  channelId: string;
  // The user that sent the message
  user: TwitchChatUser;
  // User's current badges (name, URL)
  userBadges: [string, string][];
  nicknameColor: [number, number, number];
  // The message, with Discord-esque emote formatting
  message: string;
  messageId: string;
  serverTimestamp: string;
}

export interface TwitchChatUser {
  userId: string;
  // The user's "Login" name
  userName: string;
  // The user's display name
  displayName: string;
}

interface ChatStore {
  messages: TwitchChatMessage[];
  addMessage: (message: TwitchChatMessage) => void;
  initializeMessages: () => void;
}

const initialMessages: TwitchChatMessage[] = [];

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
