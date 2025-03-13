import { useEffect, useState } from "react";
import { useRouter } from "next/router";
import { Hand } from "../../components/Hand";
import { Table } from "../../components/Table";
import { ScoreBoard } from "../../components/ScoreBoard";

export default function Game() {
  const router = useRouter();
  const { id } = router.query;
  const [gameState, setGameState] = useState(null);

  useEffect(() => {
    // Connect to WebSocket and handle game state updates
    if (id) {
      const ws = new WebSocket(`ws://localhost:8000/game/${id}`);
      ws.onmessage = (event) => {
        setGameState(JSON.parse(event.data));
      };
    }
  }, [id]);

  const handleCardClick = (index: number) => {
    // Send card play action to backend
  };

  if (!gameState) return <div>Loading...</div>;

  return (
    <div>
      <ScoreBoard scores={gameState.scores} />
      <Table currentTrick={gameState.currentTrick} />
      <Hand cards={gameState.playerHand} onCardClick={handleCardClick} />
    </div>
  );
}
