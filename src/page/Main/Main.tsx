import { useSearchStore, useStore } from "@/store";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalPosition, LogicalSize } from '@tauri-apps/api/window';
import { format_ww_size_mode, listener_el_resize } from "@/lib/window"
import { ChangeEvent, DetailedHTMLProps, FormEventHandler, InputHTMLAttributes, useEffect, useState } from "react";

const current_window = getCurrentWebviewWindow()
current_window.hide()


export function Component() {
    useEffect(() => {
        const { clientHeight, clientWidth } = document.getElementById("main-window")!
        const { availHeight, availWidth } = screen
        current_window.setAlwaysOnTop(true)
        current_window.setFocus()

        current_window.setSize(new LogicalSize(availWidth / 3, clientHeight))
        current_window.setPosition( new LogicalPosition(availWidth / 3, availHeight / 4) )

        current_window.show()
    }, [])


    const search_store = useSearchStore()
    const [focus, set_focus] = useState(document.hasFocus())




    return (
        <div
            id="main-window"
            data-tauri-drag-region
            className="h-[min-content]"
        >
            <SearchInput
                onInput={(e: ChangeEvent<HTMLInputElement>) => {
                    console.log(e)
                    search_store.set_search_value(e.target.value)
                }}
                // onBlur={() => current_window.close()}
            />
            <span>1
            { focus + "" }</span>
        </div>
    );
}




type SearchInputProps = DetailedHTMLProps<InputHTMLAttributes<HTMLInputElement>, HTMLInputElement> & {

}

function SearchInput(prop: SearchInputProps) {

    useEffect(() => {
        const el = document.getElementById("search-input") as HTMLInputElement

        el?.focus()
    }, [])

    return <input
            data-tauri-drag-region
            id="search-input"
            className="w-full outline-none h-14 text-3xl p-2 cursor-pointer"
            placeholder="Hi, rTools"
            type="text"
            { ... prop}
        />
}



// function
