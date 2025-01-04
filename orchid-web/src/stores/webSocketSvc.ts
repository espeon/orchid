import { TwitchChatMessage } from "./chatStore";
import { LayoutUpdateMessage } from "@/lib/layoutUpdateTypes";

export class WebSocketService {
  private socket: WebSocket | null = null;
  private messageHandler: (message: TwitchChatMessage) => void;

  constructor(messageHandler: (message: TwitchChatMessage) => void) {
    this.messageHandler = messageHandler;
  }

  connect(): void {
    if (this.socket) return;

    const url = new URL(window.location.href);
    url.protocol = "wss:";
    url.pathname = "/ws";

    console.log("Connecting to websocket server:", url.toString());
    this.socket = new WebSocket(url.toString());

    this.socket.addEventListener("open", () => {
      console.log("WebSocket connection established");
    });

    this.socket.addEventListener("message", (event) => {
      try {
        if (event.data.startsWith("{")) {
          const data: TwitchChatMessage = JSON.parse(event.data);
          if (
            data.msgType === "PRIVMSG" &&
            data.user?.userName &&
            data.message
          ) {
            console.log(`${data.user.userName}: ${data.message}`);
            this.messageHandler(data);
          }
        }
      } catch (error) {
        console.error("Error parsing WebSocket message:", error);
      }
    });

    this.socket.addEventListener("close", () => {
      console.warn("WebSocket connection closed. Attempting to reconnect...");
      this.socket = null;
      setTimeout(() => this.connect(), 5000); // Reconnect after 5 seconds
    });

    this.socket.addEventListener("error", (error) => {
      console.error("WebSocket error:", error);
    });
  }

  startPing(): void {
    if (!this.socket) return;

    const interval = setInterval(() => {
      if (this.socket.readyState === WebSocket.OPEN) {
        this.socket.send("ping");
      }
    }, 30000); // Send a ping message every 30 seconds

    this.socket.addEventListener("message", (event) => {
      if (event.data === "pong") {
        clearInterval(interval);
      }
    });
  }

  disconnect(): void {
    if (this.socket) {
      this.socket.close();
      this.socket = null;
    }
  }

  isConnected(): boolean {
    return this.socket !== null && this.socket.readyState === WebSocket.OPEN;
  }
}
