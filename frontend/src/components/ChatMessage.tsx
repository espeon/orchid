import { TwitchChatMessage } from "@/stores/chatStore";

const ChatMessage = ({ message }: { message: TwitchChatMessage }) => {
  return (
    <>
      <span
        className="text-purple-400 font-medium"
        style={{
          color: `rgb(${message.nicknameColor[0]}, ${message.nicknameColor[1]}, ${message.nicknameColor[2]})`,
        }}
      >
        {message.user.displayName}:
      </span>
      <span className="text-white/90 ml-2">{message.message}</span>
    </>
  );
};

export default ChatMessage;
