import { useSearchStore } from "@/store";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalPosition, LogicalSize } from "@tauri-apps/api/window";
import {
    ChangeEvent,
    DetailedHTMLProps,
    InputHTMLAttributes,
    useEffect,
} from "react";
import { dispatch_event } from "@/util";
import { useAddEventListener } from "src/hook";

/**
 * @description current webview window
 */
const current_window = getCurrentWebviewWindow();


/**
 * @description if window is not focused, set window focus
 * @returns
 */
const set_window_focus = async function () {
    const is_focus = await current_window.isFocused();
    if (is_focus) return;
    await current_window.setFocus();
};
set_window_focus();

/**
 * @description close window
 * @returns
 */
const window_close = async function () {
    if (process.env.NODE_ENV === "production")
        await current_window.close()
};



/**
 * @description When the DOM is in focus, there is a very short period of loss of focus when clicking on it. A time delay is given. If the focus is regained within this time, the page will not be closed.
 */
const dispatch_focus = dispatch_event(() => {
    return window_close();

/**
 * @description Delay time at least 10ms
 */
//  this here
//      |
//  ┌───┘
//  ↓
}, 10);

export function Component() {

    useAddEventListener("blur", dispatch_focus.exec);
    useAddEventListener("focus", dispatch_focus.cancel);


    /**
     * @description Press esc to close the page
     */
    useAddEventListener("keydown", (e: KeyboardEvent) => {
        if (e.key === "Escape") return window_close();
    });


    useEffect(() => {
        const { clientHeight, clientWidth } =
            document.getElementById("main-window")!;
        const { availHeight, availWidth } = screen;
        current_window.setAlwaysOnTop(true);

        // width is screen avail width one third, height with #main-window height
        current_window.setSize(new LogicalSize(availWidth / 3, clientHeight));

        // position is screen avail width one third, height with half of the window
        current_window.setPosition(
            new LogicalPosition(availWidth / 3, availHeight / 4)
        );

        // create window is hidden, change position and size after show window
        current_window.show();
    }, []);


    const search_store = useSearchStore();

    return (
        <div
            id="main-window"
            data-tauri-drag-region
            className="h-[min-content]"

        >
            <SearchInput
                onInput={(e: ChangeEvent<HTMLInputElement>) => {
                    search_store.set_search_value(e.target.value);
                }}
            />
        </div>
    );
}

type SearchInputProps = DetailedHTMLProps<
    InputHTMLAttributes<HTMLInputElement>,
    HTMLInputElement
> & {};

function SearchInput(prop: SearchInputProps) {
    useEffect(() => {
        // input focus
        const el = document.getElementById("search-input") as HTMLInputElement;
        el?.focus();
    }, []);

    return (
        <input
            data-tauri-drag-region
            id="search-input"
            className="w-full outline-none h-14 text-3xl p-2 cursor-pointer"
            placeholder="Hi, rTools"
            type="text"
            {...prop}
        />
    );
}

// function
