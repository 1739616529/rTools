
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";


/**
 *
 * @description 关闭webview
 * @param webview
 * @returns
 */
export const webview_close = function(webview = getCurrentWebviewWindow()) {
    // if (process.env.NODE_ENV === "production")
        return webview.close();
}

/**
 * @description 确保dom获取焦点
 */
export const webview_focus = async function(webview = getCurrentWebviewWindow()) {
    console.log(1, await webview.isFocused(), document.hasFocus())
    await webview.setFocus()
    window.focus()

    console.log(2, await webview.isFocused(), document.hasFocus())
}
