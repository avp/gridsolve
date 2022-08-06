import * as React from 'react';
import * as ReactDOM from 'react-dom/client';
import init, { puzzle } from './pkg/gridsolve_wasm.js';

function App() {
  const solve = () => {
    console.log(
      puzzle(`
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
      `)
    );
  };

  return (
    <div>
      <button onClick={solve}>Make Puzzle</button>
    </div>
  );
}

const root = ReactDOM.createRoot(document.getElementById('root'));
fetch("dist/gridsolve_wasm_bg.wasm").then(
  (wasm) => init(wasm).then(() => root.render(<App />))
);
