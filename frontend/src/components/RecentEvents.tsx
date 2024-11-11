import { Heart, Star, Trophy } from 'lucide-react';

interface StreamEvent {
  type: 'follow' | 'donation' | 'subscription';
  username: string;
  amount?: number;
  message?: string;
  timestamp: Date;
}

const recentEvents: StreamEvent[] = [
  { type: 'follow', username: 'PokeMaster99', timestamp: new Date() },
  { type: 'donation', username: 'Pikachu_Lover', amount: 5, timestamp: new Date() },
  { type: 'subscription', username: 'DragoniteKing', timestamp: new Date() },
];

const RecentEvents = () => {
  return (
    <div className="bg-black/40 rounded-xl backdrop-blur-sm border border-white/10 p-4">
      <h3 className="text-white font-semibold mb-4">Recent Events</h3>
      <div className="space-y-3">
        {recentEvents.map((event, index) => (
          <div key={index} className="flex items-center space-x-3 text-sm">
            {event.type === 'follow' && <Heart className="w-4 h-4 text-red-400" />}
            {event.type === 'donation' && <Star className="w-4 h-4 text-yellow-400" />}
            {event.type === 'subscription' && <Trophy className="w-4 h-4 text-purple-400" />}
            <span className="text-white/90">{event.username}</span>
            {event.amount && <span className="text-green-400">${event.amount}</span>}
          </div>
        ))}
      </div>
    </div>
  );
};

export default RecentEvents;