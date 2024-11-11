import { LayoutUpdateMessage } from "@/lib/layoutUpdateTypes";
import { WebSocket } from "ws";
import { useLayoutStore } from "./layoutItemsStore";

interface WSService {
  socket: WebSocket;
  connect(): void;
  disconnect(): void;
}

class WebSocketService implements WSService {
  socket: WebSocket;

  constructor() {
    this.socket = new WebSocket("wss://localhost:3000");
  }

  connect(): void {
    this.socket.onopen = () => {
      console.log("Connected to websocket server");
    };

    this.socket.onmessage = (event) => {
      const message: LayoutUpdateMessage = JSON.parse(event.data.toString());

      console.log("Received message from websocket server:", message);
    };

    this.socket.onclose = () => {
      console.log("Disconnected from websocket server");
    };
  }

  disconnect(): void {
    if (this.socket.readyState === WebSocket.OPEN) {
      this.socket.close();
    }
  }
}

export default WebSocketService;
