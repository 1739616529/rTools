import { useEffect } from "react";
import { dispatch_event } from "src/util";
import { webview_close } from "src/util/window";

/**
 * @description 监听 事件
 * @param type
 * @param listener
 * @param options
 */
export const useAddEventListener = function <K extends keyof WindowEventMap>(
    type: K,
    listener: (this: Window, ev: WindowEventMap[K]) => any,
    options?: boolean | AddEventListenerOptions
) {
    window.addEventListener(type, listener, options);
    const dispose = function () {
        window.removeEventListener(type, listener, options);
    };
    useEffect(() => dispose);
    return dispose;
};

/**
 * @description 失去焦点关闭窗口
 */
export const useBlurCloseWebviewHook = () => {
    const dispatch_focus = dispatch_event(webview_close, 10);
    return function () {
        useAddEventListener("blur", dispatch_focus.exec);
        useAddEventListener("focus", dispatch_focus.cancel);
    };
};
