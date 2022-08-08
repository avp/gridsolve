import React, { useState } from 'react';

export default function Solution({ puzzle, solution }) {
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
