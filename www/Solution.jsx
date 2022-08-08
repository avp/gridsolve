import React, { useState, useRef, useEffect } from 'react';

const YES = <span className="yesEntry">&#x2713;</span>;
const NO = <span className="noEntry">&#x2a2f;</span>;

export default function Solution({ puzzle, solution, onClear }) {
  [numSteps, setNumSteps] = useState(solution.steps.length);

  return (
    <div className="Solution">
      <div className="LeftContainer">
        <button onClick={onClear}>Clear Solution</button>
        <SolutionTable puzzle={puzzle} solution={solution} />
        <input
          type="range"
          className="stepsSlider"
          min="1"
          max={solution.steps.length}
          value={numSteps}
          onChange={(e) => setNumSteps(parseInt(e.target.value, 10))}
        />
        <SolutionGrid puzzle={puzzle} solution={solution} numSteps={numSteps} />
      </div>
      <div className="StepContainer">
        <StepList puzzle={puzzle} solution={solution} numSteps={numSteps} />
      </div>
    </div>
  );
}

function SolutionTable({ puzzle, solution }) {
  const topRow = [];
  for (const category of puzzle.categories) {
    topRow.push(
      <td key={category}>
        <b>{category}</b>
      </td>
    );
  }

  const rows = [];
  for (let i = 0; i < solution.solution.length; ++i) {
    const solutionRow = solution.solution[i];
    const row = [];
    for (const category of puzzle.categories) {
      row.push(<td key={category}>{solutionRow[category]}</td>);
    }
    rows.push(<tr key={i}>{...row}</tr>);
  }

  return (
    <div>
      <table className="SolutionTable">
        <thead>
          <tr>{topRow}</tr>
        </thead>
        <tbody>{...rows}</tbody>
      </table>
    </div>
  );
}

function SolutionGrid({ puzzle, solution, numSteps }) {
  const rows = [];

  const lookup = {};
  for (let i = 0; i < numSteps; ++i) {
    const step = solution.steps[i];
    lookup[step.label1] ??= {};
    lookup[step.label1][step.label2] = step.yes;
    console.log(lookup);
  }

  function doLookup(label1, label2) {
    const entry = lookup[label1]?.[label2];
    if (typeof entry === 'undefined') {
      return <span>&nbsp;</span>;
    }
    return entry ? YES : NO;
  }

  let row = [];
  row.push(<td key={-1} className="spacer"></td>);
  for (let c = puzzle.categories.length - 1; c > 0; --c) {
    for (let i = 0; i < puzzle.numLabels; ++i) {
      const label = puzzle.labels[puzzle.numLabels * c + i];
      row.push(
        <td key={label} className="labelName top">
          <span className="rotatedText">
            <span className="rotatedTextInner">{label}</span>
          </span>
        </td>
      );
    }
    row.push(<td key={c} className="spacer"></td>);
  }
  rows.push(<tr>{...row}</tr>);

  for (let c = 0; c < puzzle.categories.length - 1; ++c) {
    for (let i = 0; i < puzzle.numLabels; ++i) {
      const label = puzzle.labels[puzzle.numLabels * c + i];
      row = [];
      row.push(
        <td key={label} className="labelName left">
          <span>{label}</span>
        </td>
      );
      for (let c2 = puzzle.categories.length - 1; c2 > c; --c2) {
        for (let i2 = 0; i2 < puzzle.numLabels; ++i2) {
          const label2 = puzzle.labels[puzzle.numLabels * c2 + i2];
          row.push(
            <td key={puzzle.numLabels * c2 + i2} className="gridEntry">
              <span className="entryContainer">{doLookup(label, label2)}</span>
            </td>
          );
        }
        row.push(<td key={'spacer' + c2} className="spacer"></td>);
      }
      rows.push(<tr>{...row}</tr>);
    }
    rows.push(
      <tr className="spacer">
        <td className="spacer" />
      </tr>
    );
  }

  return (
    <table className="SolutionGrid">
      <tbody>{...rows}</tbody>
    </table>
  );
}

function StepList({ puzzle, solution, numSteps }) {
  const endRef = useRef(null);
  const steps = [];

  useEffect(() => {
    endRef.current?.scrollIntoView();
  }, [numSteps]);

  for (let i = 0; i < numSteps; ++i) {
    const step = solution.steps[i];
    steps.push(
      <li key={i} className="step">
        <span className="stepResult">
          {step.label1} {step.yes ? YES : NO} {step.label2}
        </span>
        {step.description && (
          <span className="stepDescription">{step.description}</span>
        )}
      </li>
    );
  }

  return (
    <ol>
      {...steps}
      <div ref={endRef}></div>
    </ol>
  );
}
