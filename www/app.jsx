import React, { useState, useEffect } from 'react';
import ReactDOM from 'react-dom/client';
import initWASM, { solve_puzzle as solveWASM } from './pkg/gridsolve_wasm.js';
import PuzzleInput from './PuzzleInput';
import Solution from './Solution';

function readHash() {
  if (window.location.hash) {
    try {
      const puzzle = JSON.parse(
        decodeURIComponent(window.location.hash.substring(1))
      );
      const solution = JSON.parse(solveWASM(puzzle.puzzleString));
      if (!solution.error) {
        return [puzzle, solution];
      }
    } catch (e) {
      console.error(e);
    }
  }
  return [null, null];
}

function App() {
  const [puzzle, setPuzzle] = useState(existingPuzzle);
  const [solution, setSolution] = useState(existingSolution);

  useEffect(() => {
    if (!puzzle) {
      window.location.hash = '';
    } else {
      window.location.hash = encodeURIComponent(JSON.stringify(puzzle));
    }
  }, [puzzle]);

  function handleInput(puzzle, solution) {
    setPuzzle(puzzle);
    setSolution(solution);
  }

  if (solution) {
    return (
      <div>
        <Solution
          puzzle={puzzle}
          solution={solution}
          onClear={() => handleInput(null, null)}
        ></Solution>
      </div>
    );
  }
  return <PuzzleInput onSolution={handleInput}></PuzzleInput>;
}

let existingPuzzle = null;
let existingSolution = null;
const root = ReactDOM.createRoot(document.getElementById('root'));

fetch('dist/gridsolve_wasm_bg.wasm').then((wasm) =>
  initWASM(wasm).then(() => {
    [existingPuzzle, existingSolution] = readHash();
    root.render(<App />);
  })
);
