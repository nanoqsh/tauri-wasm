export function is_tauri() {
    return 'isTauri' in window && !!window.isTauri;
}

const ek = ['', 'Any', 'AnyLabel', 'App', 'Window', 'Webview', 'WebviewWindow'];

export function eargs(event, payload, k, l) {
    let o = { event, payload };
    if (k) {
        o.target = { kind: ek[k] };
        if (l) o.target.label = l;
    }

    return o;
}
