import { useSearchStore } from "@/store";
import {
    ChangeEvent,
    DetailedHTMLProps,
    InputHTMLAttributes,
    useEffect,
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
            data-tauri-drag-region
            className="h-[min-content] overflow-hidden rounded-lg bg-white dark:bg-black"

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
        webview_focus().then(() => {
            const el = document.getElementById("search-input") as HTMLInputElement;
            el?.focus();
        })

    }, []);

    return (
        <input
            data-tauri-drag-region
            id="search-input"
            className="w-full outline-none h-14 text-3xl pl-4 cursor-pointer bg-transparent"
            placeholder={document.hasFocus() + ""}
            // placeholder="Hi, rTools"
            type="text"
            {...prop}
        />
    );
}

// function
