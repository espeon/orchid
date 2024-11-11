// https://raw.githubusercontent.com/PokeAPI/sprites/refs/heads/master/sprites/badges/1.png

import { useState } from "react";

interface Badge {
    name: string,
    image: string,
    complete: boolean,
}

const badges: Badge[] = [
    { name: "Boulder Badge", image: "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/badges/1.png", complete: false },
    { name: "Cascade Badge", image: "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/badges/2.png", complete: false },
    { name: "Thunder Badge", image: "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/badges/3.png", complete: false },
    { name: "Rainbow Badge", image: "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/badges/4.png", complete: false },
    { name: "Soul Badge", image: "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/badges/5.png", complete: false },
    { name: "Marsh Badge", image: "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/badges/6.png", complete: false },
    { name: "Volcano Badge", image: "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/badges/7.png", complete: false },
    { name: "Earth Badge", image: "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/badges/8.png", complete: false },
];


export default function BadgeCounter() {
    const [badgeList, setBadgeList] = useState(badges);

    const toggleComplete = (index: number) => {
        const updatedBadges = [...badgeList];

        const checkTrue = !updatedBadges[index].complete;
        if (checkTrue){
            for (let i = 0; i < index+1; i++) {
                console.log("Setting", i)
                updatedBadges[i].complete = true
            }
        } else {
            for (let i = index; i < updatedBadges.length; i++) {
                console.log("Unsetting", i)
                updatedBadges[i].complete = false
            }
        }
        setBadgeList(updatedBadges);
    };

    return (
        <div className="grid grid-rows-2 grid-cols-4 gap-4">
            {badgeList.map((badge, index) => (
                <div
                    key={badge.name + badge.image}
                    onClick={() => toggleComplete(index)}
                    className={`align-center justify-center cursor-pointer filter transition-[all] duration-1000 
                        ${badge.complete 
                            ? "drop-shadow-[0_0_0.7em_green] brightness-1" 
                            : "brightness-0 hover:brightness-50 hover:grayscale"}
                        `}
                >
                    <img className="w-10 h-10" src={badge.image} />
                </div>
            ))}
        </div>
    )
}