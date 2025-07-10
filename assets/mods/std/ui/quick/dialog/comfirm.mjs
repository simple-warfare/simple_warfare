import { Signal } from "std:signal/signal.mjs"
import { QuickDialogType } from "std:ui/quick/dialog.mjs"

export class ComfirmDialog {
    constructor(title, context) {
        this.title = title
        this.context = context
        this.type = QuickDialogType.Comfirm
        this.onPressCancel = new Signal()
        this.onPressComfirm = new Signal()
        sw.create_quick_ui(this)
    }
};