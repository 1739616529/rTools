
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalSize } from '@tauri-apps/api/window';



export enum EWebviewWindowSizeMode {
    main,

}


/**
 * @description foramt webview window size by mode
 * @param mode
 * @returns
 */
export const format_ww_size_mode = function(mode = EWebviewWindowSizeMode.main) {
    switch (mode) {
        case EWebviewWindowSizeMode.main:
            return get_main_ww_size();
    }
}


/**
 * @description get main window size
 * @returns LogicalSize
 */
export const get_main_ww_size = function() {
    const { clientWidth, clientHeight } = document.getElementById("main-window")!;
    console.log(clientWidth, clientHeight)
    return new LogicalSize(clientWidth, clientHeight)
}



export const listener_el_resize = function(el: HTMLElement, cb: (entry: ResizeObserverEntry) => void): () => void {
    const obs = new ResizeObserver(([entry]) => cb(entry));
    obs.observe(el);
    return () => obs.unobserve(el)
}
