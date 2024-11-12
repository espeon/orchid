// src/components/ChatBox.tsx
import { MessageCircle } from "lucide-react";
import { useEffect, useRef } from "react";
import ChatMessage from "./ChatMessage";
import { useLayoutStore } from "@/stores/layoutItemsStore";

const ChatBox = () => {
  const { messages } = useLayoutStore();
  const scrollRef = useRef<HTMLDivElement>(null);

  // useEffect(() => {
  //   const interval = setInterval(() => {
  //     let newMessage =
  //       randomMessages[Math.floor(Math.random() * randomMessages.length)];

  //     // if the message is the same as the last one, try again
  //     while (
  //       messages.length > 0 &&
  //       messages[messages.length - 1].message === newMessage.message
  //     ) {
  //       newMessage =
  //         randomMessages[Math.floor(Math.random() * randomMessages.length)];
  //     }

  //     addMessage(newMessage);
  //   }, 300);

  //   return () => clearInterval(interval);
  // }, [messages, addMessage]);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [messages]);

  return (
    <div className="bg-black/40 rounded-xl backdrop-blur-sm border border-white/10 p-4 h-full max-h-full flex flex-col">
      <div className="fixed">
        <div className="flex items-center space-x-2 mb-4">
          <MessageCircle className="w-5 h-5 text-purple-300" />
          <h3 className="text-white font-semibold">Live Chat</h3>
        </div>
      </div>
      <div
        className="overflow-y-auto h-min min-h-full max-h-full fixed"
        style={{
          maskImage: `linear-gradient(to bottom, transparent 8%, black 40%)`,
          maskComposite: "intersect",
        }}
        ref={scrollRef}
      >
        <div className="overflow-y-visible hide-scrollbar space-y-2 pb-8 px-1 mr-1">
          <div className="h-10" />
          {messages.map((m, i) => (
            <div
              className="text-sm text-pretty break-words "
              key={m.user + m.message + i}
            >
              <ChatMessage message={m} />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default ChatBox;
