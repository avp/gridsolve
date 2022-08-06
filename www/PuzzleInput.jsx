import React, { useState } from 'react';

export default function PuzzleInput() {
  const [numCategories, setNumCategories] = useState(2);
  const [numLabels, setNumLabels] = useState(2);

  const categoryInputs = [];
  console.log(numCategories);
  for (let i = 0; i < numCategories; ++i) {
    categoryInputs.push(<CategoryInput key={i} numLabels={numLabels} />);
  }

  return (
    <div>
      <form action="">
        <p>
          <label htmlFor="numCategories"># of Categories:</label>
          <select
            name="numCategories"
            onChange={(e) => setNumCategories(e.target.value)}
          >
            {[2, 3, 4, 5, 6, 7, 8].map((n) => (
              <option key={n} value={n}>
                {n}
              </option>
            ))}
          </select>
        </p>
        <p>
          <label htmlFor="numLabels"># of Labels:</label>
          <select
            name="numLabels"
            onChange={(e) => setNumLabels(e.target.value)}
          >
            {[2, 3, 4, 5, 6, 7, 8].map((n) => (
              <option key={n} value={n}>
                {n}
              </option>
            ))}
          </select>
        </p>

        {...categoryInputs}
      </form>
    </div>
  );
}

function CategoryInput({ numLabels }) {
  const labelInputs = [];
  for (let i = 0; i < numLabels; ++i) {
    labelInputs.push(<input key={i} type="text" />);
  }

  return <div>{labelInputs}</div>;
}
