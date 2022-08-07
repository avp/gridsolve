import React, { useState, useEffect } from 'react';

const KINDS = {
  yes: {
    name: 'yes',
    params: ['label', 'label'],
  },
};

export default function ClueInput({ categories, labels, onChange }) {
  const [name, setName] = useState('');
  const [kind, setKind] = useState(KINDS.yes);
  const [params, setParams] = useState(() => initParams(kind));

  function initParams(kind) {
    const params = [];
    for (const param of kind.params) {
      switch (param) {
        case 'label':
          params.push(labels[0]);
          break;
        case 'category':
          params.push(categories[0]);
          break;
        default:
          break;
      }
    }
    return params;
  }

  function makeString() {
    console.log(params);
    return [name, kind.name, ...params].join(',');
  }

  useEffect(() => {
    onChange(makeString());
  }, [name, kind, params]);

  function setParamAt(i, newVal) {
    let copy = params.slice();
    copy[i] = newVal;
    setParams(copy);
    console.log(i, copy, params);
  }

  function makeSelect(optStrings, callback) {
    let options = [];
    for (let i = 0, n = optStrings.length; i < n; ++i) {
      options.push(
        <option key={i} value={optStrings[i]}>
          {optStrings[i]}
        </option>
      );
    }
    return (
      <select
        onChange={(e) => {
          console.log(e.target.value);
          callback(e.target.value);
        }}
      >
        {...options}
      </select>
    );
  }

  function kindSelect() {
    let options = [];
    for (let i of Object.keys(KINDS)) {
      options.push(
        <option key={i} value={KINDS[i]}>
          {KINDS[i].name}
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
            setKind(e.target.value);
            setParams(initParams(e.target.value));
          }}
        >
          {...options}
        </select>
      </>
    );
  }

  function labelSelect(paramIdx) {
    return makeSelect(labels, (s) => {
      console.log(s);
      setParamAt(paramIdx, s);
    });
  }

  switch (kind.name) {
    case 'yes':
      return (
        <div className="clueInput">
          {kindSelect()}
          {labelSelect(0)}
          {labelSelect(1)}
        </div>
      );
    default:
      break;
  }
}
