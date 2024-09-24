

export const dispatch_event = function (fn: () => (void | Promise<void>), delay: number) {
    let timer: number
    return {
        exec() {
            return new Promise<void>((resolve, reject) => {
                // @ts-ignore
                timer = setTimeout(async () => {
                    await fn()
                    resolve()
                }, delay)
            })

        },
        cancel() {
            clearTimeout(timer)
        }
    }
}


const c = async () => {
    return 10
}
