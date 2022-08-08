export default class Dialog {
    constructor(settings = {}) {
        this.setting = Object.assign(
            {
                /* default settings */
            },
            settings
        );
        this.init();
    }

    init() {
        // Testing for <dialog> support
        this.dialogSupported = typeof HTMLDialogElement === "function";
        this.dialog = document.createElement("dialog");
        this.dialog.dataset.component = this.dialogSupported
            ? "dialog"
            : "no-dialog";
        this.dialog.role = "dialog";

        // HTML template
        this.dialog.innerHTML = `
        <form method="dialog" data-ref="form">
          <fieldset data-ref="fieldset" role="document">
            <legend data-ref="message" id="${Math.round(Date.now()).toString(
                36
            )}">
            </legend>
            <div data-ref="template"></div>
          </fieldset>
          <menu>
            <button data-ref="cancel" value="cancel"></button>
            <button data-ref="accept" value="default"></button>
          </menu>
        </form>`;

        document.body.appendChild(this.dialog);

        // ...
    }
}
