import React, { useState, useEffect } from 'react';

const KINDS = {
  yes: {
    name: 'yes',
    params: ['label', 'label'],
  },
  after: {
    name: 'after',
    params: ['label', 'category', 'label'],
  },
  or: {
    name: 'or',
    params: ['label', 'label', 'label'],
  },
};

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
      kind,
      params,
    };
  }

  useEffect(() => {
    onChange(makeClue());
  }, [name, kind, params, labels, categories]);

  useEffect(() => {
    // if (!paramInputs.length) return;
    // const copy = [];
    // for (const paramInput of paramInputs) {
    //   copy.push(paramInput.value);
    // }
    // setParams(copy);
  }, [labels, categories]);

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

  function categorySelect(paramIdx) {
    return paramSelect(categories, paramIdx);
  }

  function infoText(s) {
    return <span>{s}</span>;
  }

  switch (kind.name) {
    case 'yes':
      paramInputs.push(labelSelect(0));
      paramInputs.push(infoText('is'));
      paramInputs.push(labelSelect(1));
      break;
    case 'after':
      paramInputs.push(labelSelect(0));
      paramInputs.push(infoText('is after'));
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
  }
  return (
    <div className="clueInput">
      {kindSelect()}
      {...paramInputs}
    </div>
  );
}
