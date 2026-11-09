// The copy button on each code sample: an icon, with a tooltip that confirms the
// copy. It registers the button itself, in place of Prism's copy plugin, whose
// text button would otherwise be registered under the same key.
const COPY_LABEL = 'Copy code';
const COPIED_LABEL = 'Copied!';
const FAILED_LABEL = 'Copy failed';
const RESET_MS = 1500;

const ICON = (path) => `<svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${path}</svg>`;
const COPY_ICON = ICON('<rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>');
const CHECK_ICON = ICON('<path d="M20 6 9 17l-5-5"/>');

Prism.plugins.toolbar.registerButton('copy-to-clipboard', (env) => {
    const button = document.createElement('button');
    button.className = 'copy-to-clipboard-button';
    button.type = 'button';
    button.setAttribute('aria-label', COPY_LABEL);
    button.innerHTML = `${COPY_ICON}${CHECK_ICON}<span class="copy-tooltip" role="status">${COPY_LABEL}</span>`;

    const tooltip = button.querySelector('.copy-tooltip');
    let timer;

    const show = (label, state) => {
        clearTimeout(timer);
        tooltip.textContent = label;
        button.dataset.state = state;
        timer = setTimeout(() => {
            delete button.dataset.state;
            tooltip.textContent = COPY_LABEL;
        }, RESET_MS);
    };

    button.addEventListener('click', () => {
        navigator.clipboard.writeText(env.element.textContent).then(
            () => show(COPIED_LABEL, 'copied'),
            () => show(FAILED_LABEL, 'failed'),
        );
    });

    return button;
});
