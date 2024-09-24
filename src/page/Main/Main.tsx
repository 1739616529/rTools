import { useSearchStore, useStore } from "@/store";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalPosition, LogicalSize } from '@tauri-apps/api/window';
import { ChangeEvent, DetailedHTMLProps, FormEventHandler, InputHTMLAttributes, useEffect, useState } from "react";
import {dispatch_event} from "@/util";

const current_window = getCurrentWebviewWindow()

const set_window_focus = async function () {
    const is_focus = await current_window.isFocused()
    if (is_focus) return
    await current_window.setFocus()
}

set_window_focus()
export function Component() {
    useEffect(() => {
        const { clientHeight, clientWidth } = document.getElementById("main-window")!
        const { availHeight, availWidth } = screen
        current_window.setAlwaysOnTop(true)
        current_window.setSize(new LogicalSize(availWidth / 3, clientHeight))
        current_window.setPosition( new LogicalPosition(availWidth / 3, availHeight / 4) )
        current_window.show()

        const dispatch_focus = dispatch_event(() => {
            return current_window.close()
        }, 30)
        addEventListener("blur", dispatch_focus.exec)
        addEventListener("focus", dispatch_focus.cancel)
        return () => {
            removeEventListener("blur", dispatch_focus.exec)
            removeEventListener("focus", dispatch_focus.cancel)
        }
    }, [])


    const search_store = useSearchStore()




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
