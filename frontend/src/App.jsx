import { useEffect, useState } from 'react'
import './App.css'

function App() {
  const API_URL = import.meta.env.VITE_API_URL;

  const [board, setBoard] = useState(Array(9).fill(null));
  const [validMoves, setValidMoves] = useState(Array(9).fill(false));
  const [currentPlayer, setCurrentPlayer] = useState("X");
  const [matchStatus, setMatchStatus] = useState("InProgress");
  const [message, setMessage] = useState("Player X's turn");

  const handleGameState = (state) => {
    setBoard(state["board"]);
    setValidMoves(state["valid_moves"]);

    const status = state["status"];
    setMatchStatus(status["status"]);
    if (status["status"] === "Draw") {
      setMessage("It's a Draw!");
    } else if (status["status"] === "Won") {
      setMessage(`Player ${status["player"]} won!`)
    } else if (status["status"] === "InProgress") {
      setMessage(`Player ${status["player"]}'s turn`)
      setCurrentPlayer(status["player"])
    }
  }

  const updateGameState = async () => {
    try {
      const response = await fetch("/api/game", {
        method: 'GET',
      });

      const data = await response.json();
      handleGameState(data);
    } 
    catch (error) {
      console.error("Error getting state: ", error)
    }
  }

  const makeMove = async (x, y) => {
    try {
      const response = await fetch("/api/game/move", {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify([x, y]),
      });

      const data = await response.json();
      handleGameState(data);
    } 
    catch (error) {
      console.error("Error making move: ", error)
    }
  }

  const newGame = async () => {
    try {
      const response = await fetch("/api/game/new", {
        method: 'GET',
      });

      const data = await response.json();
      handleGameState(data);
    } 
    catch (error) {
      console.error("Error getting new game: ", error)
    }
  }

  const renderCell = (x, y) => {
    return (
      <button 
        key={`${x}-${y}`} 
        className="cell" 
        onMouseDown={() => makeMove(x, y)}
        disabled={!validMoves[x + 3 * y] || matchStatus !== "InProgress"}
        >
        {board[x + 3 * y]} 
      </button>
    )
  }

  useEffect(() => {
    updateGameState();
  }, []);

  return (
    <>
      <div className="grid-container">
        {[...Array(3)].map((_, y) =>
          [...Array(3)].map((_, x) => renderCell(x, y))
        )}
      </div>
      <p className='message'>{message}</p>
      <button onClick={() => newGame()} className='new-game'>New game</button>
    </>
  )
}

export default App
