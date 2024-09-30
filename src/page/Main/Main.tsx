import { useSearchStore } from "@/store";
import {
    ChangeEvent,
    DetailedHTMLProps,
    InputHTMLAttributes,
    useEffect,
    useRef,
    useState,
} from "react";
import { useAddEventListener, useBlurCloseWebviewHook } from "src/hook";
import { webview_close, webview_focus } from "src/util/window";
import "./main.css";






export function Component() {

    /**
     * @description blur close webview
     */
    useBlurCloseWebviewHook()
    /**
     * @description Press esc to close the page
     */
    useAddEventListener("keydown", (e: KeyboardEvent) => {
        if (e.key === "Escape") return webview_close();
    });


    const search_store = useSearchStore();

    return (
        <div
            id="main-window"
            className="h-[max-content] overflow-hidden rounded-lg bg-white dark:bg-black"

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
    const [focused, set_focused] = useState(document.hasFocus())

    useAddEventListener("focus", () => set_focused(() => true))
    useAddEventListener("blur", () => set_focused(() => false))
    useEffect(() => {

        webview_focus().then(() => {
            const el = document.getElementById("search-input") as HTMLInputElement;
            el?.focus();
        })

    }, []);


    return (
        <div className="h-[max-content]">
            <input
                data-tauri-drag-region
                id="search-input"
                className="w-full outline-none h-14 text-3xl cursor-default pl-4 bg-transparent placeholder:translate-x-1"
                placeholder={focused.toString()}
                // placeholder="Hi, rTools"
                type="text"
                {...prop}
            />
        </div>
    );
}

// function
