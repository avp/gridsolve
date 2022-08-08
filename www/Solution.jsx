import React, { useState } from 'react';

export default function Solution({ puzzle, solution }) {
  return (
    <div className="Solution">
      <SolutionTable puzzle={puzzle} solution={solution} />
      <SolutionGrid puzzle={puzzle} solution={solution} />
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
      <table border="1">
        <thead>
          <tr>{topRow}</tr>
        </thead>
        <tbody>{...rows}</tbody>
      </table>
    </div>
  );
}

function SolutionGrid({ puzzle, solution }) {
  const rows = [];

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
          const label = puzzle.labels[puzzle.numLabels * c + i];
          row.push(
            <td key={puzzle.numLabels * c2 + i2} className="gridEntry">
              <span></span>
            </td>
          );
        }
        row.push(<td key={'spacer' + c2} className="spacer"></td>);
      }
      rows.push(<tr>{...row}</tr>);
    }
    rows.push(<tr className="spacer"><td className="spacer" /></tr>);
  }

  return (
    <table className="SolutionGrid">
      <tbody>{...rows}</tbody>
    </table>
  );
}
