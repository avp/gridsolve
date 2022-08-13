import KINDS from './ClueKinds';

export function makePuzzleString({ categories, labels, numLabels, clues }) {
  let puzzleString = '[Categories]\n';
  let l = 0;
  for (let c = 0; c < categories.length; ++c) {
    puzzleString += categories[c] + '\n';
    for (let i = 0; i < numLabels; ++i) {
      puzzleString += labels[l] + '\n';
      ++l;
    }
    puzzleString += '\n';
  }
  puzzleString += '[Clues]\n';
  for (const clue of clues) {
    const kind = KINDS[clue.kind];
    puzzleString += clue.name + ',' + kind.name;
    for (let i = 0, e = kind.params.length; i < e; ++i) {
      puzzleString += ',';
      switch (kind.params[i]) {
        case 'label':
          puzzleString += labels[clue.params[i]];
          break;
        case 'category':
          puzzleString += categories[clue.params[i]];
          break;
        case 'number':
          puzzleString += clue.params[i].toString();
          break;
        default:
          break;
      }
    }
    puzzleString += '\n';
  }

  return puzzleString;
}
