import React, { useState, useEffect } from 'react';
import KINDS from './ClueKinds';

export default function ClueInput({ categories, labels, onChange }) {
  const [name, setName] = useState('');
  const [kind, setKind] = useState(KINDS.yes);
  const [params, setParams] = useState(initParams(KINDS.yes));
  const paramInputs = [];

  function initParams(k) {
    const copy = [];
    for (const param of k.params) {
      copy.push(0);
    }
    return copy;
  }

  function makeClue() {
    return {
      name,
      kind: kind.name,
      params,
    };
  }

  useEffect(() => {
    onChange(makeClue());
  }, [name, kind, params, labels, categories]);

  function setParamAt(i, newVal) {
    let copy = params.slice();
    copy[i] = newVal;
    setParams(copy);
  }

  function paramSelect(optStrings, paramIdx) {
    let options = [];
    for (let i = 0, n = optStrings.length; i < n; ++i) {
      options.push(
        <option key={i} value={i}>
          {optStrings[i]}
        </option>
      );
    }
    return (
      <select
        onChange={(e) => {
          setParamAt(paramIdx, e.target.value);
        }}
        value={params[paramIdx]}
      >
        {...options}
      </select>
    );
  }

  function kindSelect() {
    let options = [];
    for (let name of Object.keys(KINDS)) {
      options.push(
        <option key={name} value={name}>
          {name}
        </option>
      );
    }
    return (
      <>
        <input
          type="text"
          placeholder="name"
          onChange={(s) => setName(s.target.value)}
          required
        />
        <select
          onChange={(e) => {
            let k = KINDS[e.target.value];
            setKind(k);
            setParams(initParams(k));
          }}
          className="kindSelect"
        >
          {...options}
        </select>
      </>
    );
  }

  function labelSelect(paramIdx) {
    return paramSelect(labels, paramIdx);
  }

  function numberSelect(paramIdx) {
    const nums = [];
    for (let i = 1; i < categories.length-1; ++i) {
      nums.push(i);
    }
    return paramSelect(nums, paramIdx);
  }

  function categorySelect(paramIdx) {
    return paramSelect(categories, paramIdx);
  }

  function infoText(s) {
    return <span className="infoText">{s}</span>;
  }

  switch (kind.name) {
    case 'yes':
      paramInputs.push(labelSelect(0));
      paramInputs.push(infoText('is'));
      paramInputs.push(labelSelect(1));
      break;
    case 'no':
      paramInputs.push(labelSelect(0));
      paramInputs.push(infoText('is not'));
      paramInputs.push(labelSelect(1));
      break;
    case 'after':
      paramInputs.push(labelSelect(0));
      paramInputs.push(infoText('is after'));
      paramInputs.push(labelSelect(2));
      paramInputs.push(infoText('in'));
      paramInputs.push(categorySelect(1));
      break;
    case 'afterexactly':
      paramInputs.push(labelSelect(0));
      paramInputs.push(infoText('is exactly'));
      paramInputs.push(labelSelect(3));
      paramInputs.push(infoText('spots after'));
      paramInputs.push(labelSelect(2));
      paramInputs.push(infoText('in'));
      paramInputs.push(categorySelect(1));
      break;
    case 'or':
      paramInputs.push(labelSelect(0));
      paramInputs.push(infoText('is either'));
      paramInputs.push(labelSelect(1));
      paramInputs.push(infoText('or'));
      paramInputs.push(labelSelect(2));
      break;
    case 'xor':
      paramInputs.push(labelSelect(0));
      paramInputs.push(infoText('is exactly one of'));
      paramInputs.push(labelSelect(1));
      paramInputs.push(infoText('or'));
      paramInputs.push(labelSelect(2));
      break;
    case 'twobytwo':
      paramInputs.push(infoText('either'));
      paramInputs.push(labelSelect(0));
      paramInputs.push(infoText('is'));
      paramInputs.push(labelSelect(2));
      paramInputs.push(infoText('and'));
      paramInputs.push(labelSelect(1));
      paramInputs.push(infoText('is'));
      paramInputs.push(labelSelect(3));
      paramInputs.push(infoText('or vice versa'));
      break;
  }
  return (
    <div className="clueInput">
      {kindSelect()}
      {...paramInputs}
    </div>
  );
}
