function define(n, o) {
    console.log(o);
    // TODO: For each rust component manually register anonymous proxy class
    customElements.define(n, class extends HTMLElement {
        constructor() {
            super();
        }
    })
}


class Proxy {

}