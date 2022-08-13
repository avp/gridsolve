const KINDS = {
  yes: {
    name: 'yes',
    params: ['label', 'label'],
  },
  no: {
    name: 'no',
    params: ['label', 'label'],
  },
  after: {
    name: 'after',
    params: ['label', 'category', 'label'],
  },
  afterexactly: {
    name: 'after',
    params: ['label', 'category', 'label', 'number'],
  },
  or: {
    name: 'or',
    params: ['label', 'label', 'label'],
  },
  xor: {
    name: 'xor',
    params: ['label', 'label', 'label'],
  },
  twobytwo: {
    name: 'twobytwo',
    params: ['label', 'label', 'label', 'label'],
  },
};
export default KINDS;
