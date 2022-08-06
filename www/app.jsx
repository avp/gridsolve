import * as React from 'react';
import * as ReactDOM from 'react-dom/client';
import initWASM, { solve_puzzle as solveWASM } from './pkg/gridsolve_wasm.js';

function App() {
  const solve = () => {
    let json = solveWASM(`
[Categories]
First Name
Angela
Donald
Leo

Country
Germany
Ireland
United States

Year of Birth
1946
1954
1979

[Clues]
1,yes,United States,1946
2,after,Leo,Year of Birth,Germany
3,or,Donald,1946,Ireland
      `);
    let solution = JSON.parse(json);
    console.log(solution);
  };

  return (
    <div>
      <button onClick={solve}>Make Puzzle</button>
    </div>
  );
}

const root = ReactDOM.createRoot(document.getElementById('root'));
fetch('dist/gridsolve_wasm_bg.wasm').then((wasm) =>
  initWASM(wasm).then(() => root.render(<App />))
);