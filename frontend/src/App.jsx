import { useState } from 'react'
import './App.css'

function App() {
  const API_URL = import.meta.env.API_URL;

  const [board, setBoard] = useState(Array(9).fill(null));

  const makeMove = async (x, y) => {
    try {
      const response = await fetch(`${API_URL}/api/game/move`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ x, y }),
      });
    } 
    catch {
      
    }
  }

  const renderCell = (x, y) => {
    return (
      <button className="cell">
        {board[x, y]}
      </button>
    )
  }

  return (
    <>
      <div className="grid-container">
        {[...Array(3)].map((_, y) =>
          [...Array(3)].map((_, x) => renderCell(x, y))
        )}
      </div>
    </>
  )
}

export default App
