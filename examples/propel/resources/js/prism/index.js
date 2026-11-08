// Syntax highlighting for the code samples in the documentation. Languages load
// in this order: core first, then the grammars, then Blade, which extends markup.
import './core.js';
import 'prismjs/components/prism-markup';
import 'prismjs/components/prism-rust';
import 'prismjs/components/prism-bash';
import 'prismjs/components/prism-css';
import 'prismjs/components/prism-toml';
import 'prismjs/plugins/toolbar/prism-toolbar';
import 'prismjs/plugins/toolbar/prism-toolbar.css';
import 'prismjs/plugins/copy-to-clipboard/prism-copy-to-clipboard';
import 'prismjs/plugins/line-numbers/prism-line-numbers';
import 'prismjs/plugins/line-numbers/prism-line-numbers.css';

// Blade: markup, plus directives (@if, @foreach) and echoes ({{ }} and {!! !!}).
Prism.languages.blade = Prism.languages.extend('markup', {});
Prism.languages.insertBefore('blade', 'tag', {
    'blade-echo': {
        pattern: /\{\{[\s\S]*?\}\}|\{!![\s\S]*?!!\}/,
        inside: {
            punctuation: /\{\{|\}\}|\{!!|!!\}/,
            variable: /\$?[\w.]+/,
        },
    },
    directive: {
        pattern: /@\w+/,
        alias: 'keyword',
    },
});
