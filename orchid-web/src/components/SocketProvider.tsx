import { useInitializeWebSocket } from "@/stores/layoutItemsStore";

export default function SocketProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  useInitializeWebSocket();
  return <>{children}</>;
}
