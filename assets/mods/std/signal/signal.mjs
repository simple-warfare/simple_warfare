class Signal {
    constructor() {
        this.connectArray = new Array()
    }

    connect(func) {
        this.connectArray.push(func)
    }

    emit(args) {
        sw.signal_emit(this, args)
    }
}




export { Signal };