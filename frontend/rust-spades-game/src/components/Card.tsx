import React from "react";

interface CardProps {
  suit: string;
  rank: string;
  onClick?: () => void;
}

export const Card: React.FC<CardProps> = ({ suit, rank, onClick }) => {
  return (
    <div
      className="card"
      onClick={onClick}
      style={{
        width: "100px",
        height: "140px",
        border: "1px solid black",
        borderRadius: "8px",
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
        cursor: "pointer",
      }}
    >
      <div>{rank}</div>
      <div>{suit}</div>
    </div>
  );
};
