import { Gamepad2 } from "lucide-react";
import VideoCapture from "./VideoCapture";
import ChatBox from "./ChatBox";
import RecentEvents from "./RecentEvents";
import PartySelector from "./PartySelector";
import BadgeCounter from "./BadgeCounter";
import BottomBox from "./BottomBox";

const StreamLayout = () => {
  return (
    <div className="min-h-screen max-w-screen min-w-full bg-gradient-to-br from-indigo-900 via-purple-900 to-pink-900 py-auto flex">
      <div className="grid grid-cols-10 gap-3 content-center justify-center aspect-video flex-1 mx-3">
        {/* Main Game Feed */}
        <div className="col-span-8 space-y-3 content-center">
          <div className="bg-black/40 rounded-xl backdrop-blur-sm border border-white/10 p-4">
            <div className="aspect-video bg-gray-800 rounded-lg overflow-hidden relative">
              <div className="absolute inset-0 flex items-center justify-center">
                <Gamepad2 className="w-16 h-16 text-white/20" />
                <span className="text-white/50 ml-4">Game Capture</span>
              </div>
            </div>
          </div>
          {/* <BottomBox>
            <PartySelector />
            <div className="flex-1" />
            <BadgeCounter />
          </BottomBox> */}
        </div>

        {/* Right Side Elements */}
        <div className="col-span-2 space-y-3 flex flex-col max-w-full h-full">
          <ChatBox />
          {/* <RecentEvents /> */}
          <VideoCapture />
        </div>
      </div>
    </div>
  );
};

export default StreamLayout;
