import { Camera, Gamepad2, Heart, Star, Trophy, Users2 } from 'lucide-react';

const StreamInfo = () => {
  return (
          <div className="bg-black/40 rounded-xl backdrop-blur-sm border border-white/10 p-4">
            <div className="aspect-[4/3] bg-gray-800 rounded-lg overflow-hidden relative">
              <div className="absolute inset-0 flex items-center justify-center">
                <Camera className="w-16 h-16 text-white/20" />
                <span className="text-white/50 ml-4">Video Capture</span>
              </div>
            </div>
          </div>
  );
};

export default StreamInfo;