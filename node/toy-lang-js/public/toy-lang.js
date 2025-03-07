// Define your Monarch language
export const toyLanguage = {
  // Set defaultToken to invalid to see what you do not tokenize yet
  defaultToken: 'invalid',

  // C# style comments
  comments: {
    lineComment: '//',
    blockComment: ['/*', '*/'],
  },

  // Regular expressions
  identifiers: /[a-zA-Z_$][\w$]*/,
  numbers: /\d+(\.\d+)?/,

  keywords: [
    // Declarations
    'let',
    'fn',
    'obj',

    // Control Flow
    'if',
    'else',
    'for',
    'in',
    'has',
    'return',

    // Object References
    'this',
    'super',
    'new',
  ],

  // Literals
  literals: ['true', 'false', 'null', 'undefined'],

  operators: [
    '+',
    '-',
    '*',
    '/',
    '!',
    '=',
    '&',
    '|',
    '>',
    '<',
    '==',
    '!=',
    '>=',
    '<=',
    '&&',
    '||',
    '+=',
    '-=',
    '*=',
    '/=',
    '&=',
    '|=',
  ],

  // we include these common regular expressions
  symbols: /[=><!~?:&|+\-*\/\^%]+/,
  escapes:
    /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

  // The main tokenizer for our languages
  tokenizer: {
    root: [
      // Identifiers and keywords
      [
        /@identifiers/,
        {
          cases: {
            '@keywords': 'keyword',
            '@literals': 'constant.language',
            '@default': 'identifier',
          },
        },
      ],

      // Numbers
      [/@numbers/, 'number'],

      // Whitespace
      { include: '@whitespace' },

      // Delimiters and operators
      [/[{}()\[\]]/, '@brackets'],
      [/[<>](?!@symbols)/, '@brackets'],
      [
        /@symbols/,
        {
          cases: {
            '@operators': 'operator',
            '@default': 'delimiter',
          },
        },
      ],

      // Punctuation
      [/[;,.]/, 'delimiter'],

      // Strings
      [/"([^"\\]|\\.)*$/, 'string.invalid'], // non-terminated string
      [/"/, { token: 'string.quote', bracket: '@open', next: '@string' }],

      // Single quote strings
      [/'([^'\\]|\\.)*$/, 'string.invalid'], // non-terminated string
      [/'/, { token: 'string.quote', bracket: '@open', next: '@stringSingle' }],
    ],

    string: [
      [/[^\\"]+/, 'string'],
      [/@escapes/, 'string.escape'],
      [/\\./, 'string.escape.invalid'],
      [/"/, { token: 'string.quote', bracket: '@close', next: '@pop' }],
    ],

    stringSingle: [
      [/[^\\']+/, 'string'],
      [/@escapes/, 'string.escape'],
      [/\\./, 'string.escape.invalid'],
      [/'/, { token: 'string.quote', bracket: '@close', next: '@pop' }],
    ],

    whitespace: [
      [/[ \t\r\n]+/, 'white'],
      [/\/\*/, 'comment', '@comment'],
      [/\/\/.*$/, 'comment'],
    ],

    comment: [
      [/[^\/*]+/, 'comment'],
      [/\*\//, 'comment', '@pop'],
      [/[\/*]/, 'comment'],
    ],
  },
};
