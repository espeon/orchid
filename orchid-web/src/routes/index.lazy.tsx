import StreamLayout from "../components/StreamLayout";
import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/")({
  component: App,
});

function App() {
  return <StreamLayout />;
}
