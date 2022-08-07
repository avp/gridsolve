import React, { useState } from 'react';
import ClueInput from './ClueInput';

export default function PuzzleInput() {
  const [numCategories, setNumCategories] = useState(2);
  const [numLabels, setNumLabels] = useState(2);
  const [numClues, setNumClues] = useState(0);
  const [categories, setCategories] = useState([]);
  const [labels, setLabels] = useState([]);
  const [clues, setClues] = useState([]);

  function setCategory(i, name) {
    let copy = categories.slice();
    copy[i] = name;
    setCategories(copy);
  }

  function setClue(i, clue) {
    let copy = clues.slice();
    copy[i] = clue;
    setClues(copy);
  }

  function setLabel(categoryIdx, labelIdx, name) {
    let i = categoryIdx * numLabels + labelIdx;
    let copy = labels.slice();
    copy[i] = name;
    setLabels(copy);
  }

  function validate(ev) {
    ev.preventDefault();
    console.log(categories, labels, clues);
    console.log(numClues);
  }

  const categoryInputs = [];
  for (let i = 0; i < numCategories; ++i) {
    categoryInputs.push(
      <CategoryInput
        key={i}
        numLabels={numLabels}
        setCategory={(c) => setCategory(i, c)}
        setLabel={(j, labelName) => setLabel(i, j, labelName)}
      />
    );
  }

  const clueInputs = [];
  for (let i = 0; i < numClues; ++i) {
    clueInputs.push(
      <ClueInput
        key={i}
        categories={categories}
        labels={labels}
        onChange={(clue) => setClue(i, clue)}
      />
    );
  }

  return (
    <div>
      <form action="" onSubmit={validate}>
        <p>
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
          <label htmlFor="numCategories">Categories</label>
        </p>
        <p>
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
          <label htmlFor="numLabels">Items</label>
        </p>

        <div className="categoryInputs">{...categoryInputs}</div>

        <button
          type="button"
          value="Solve"
          onClick={() => setNumClues(numClues + 1)}
        >
          Add Clue
        </button>

        <div className="clueInputs">{...clueInputs}</div>

        <input type="submit" value="Solve" />
      </form>
    </div>
  );
}

function CategoryInput({ numLabels, setLabel, setCategory }) {
  const labelInputs = [];
  for (let i = 0; i < numLabels; ++i) {
    labelInputs.push(
      <p key={i}>
        <input
          className="labelInput"
          type="text"
          placeholder={`Item ${i}`}
          onChange={(e) => setLabel(i, e.target.value)}
          required
        />
      </p>
    );
  }

  return (
    <div className="categoryInput">
      <input
        key={-1}
        className="categoryNameInput"
        type="text"
        placeholder="Category Name"
        onChange={(e) => setCategory(e.target.value)}
        required
      />
      {labelInputs}
    </div>
  );
}
