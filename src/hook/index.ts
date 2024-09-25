import { useEffect } from "react";


/**
 * @description 监听 事件
 * @param type
 * @param listener
 * @param options
 */
export const useAddEventListener = function <K extends keyof WindowEventMap>(type: K, listener: (this: Window, ev: WindowEventMap[K]) => any, options?: boolean | AddEventListenerOptions) {
    window.addEventListener(type, listener, options);
    const dispose = function () {
        window.removeEventListener(type, listener, options);
    };
    useEffect(() => dispose)
    return dispose;
};
