import React from "react";
import { Card } from "./card";

interface HandProps {
  cards: Array<{ suit: string; rank: string }>;
  onCardClick: (index: number) => void;
}

export const Hand: React.FC<HandProps> = ({ cards, onCardClick }) => {
  return (
    <div style={{ display: "flex", gap: "10px" }}>
      {cards.map((card, index) => (
        <Card
          key={index}
          suit={card.suit}
          rank={card.rank}
          onClick={() => onCardClick(index)}
        />
      ))}
    </div>
  );
};
