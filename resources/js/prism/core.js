// Prism's language components attach to a global `Prism`, so it has to exist
// before they load. This module is imported first, for that reason.
import Prism from 'prismjs';

globalThis.Prism = Prism;
