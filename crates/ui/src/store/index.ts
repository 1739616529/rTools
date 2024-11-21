import { create } from 'zustand'


export interface StoreState {
    bears: number,
    increasePopulation: () => void,
    removeAllBears: () => void,
    updateBears: (newBears: number) => void
}

export const useStore = create<StoreState>((set) => ({
    bears: 0,
    increasePopulation: () => set((state) => ({ bears: state.bears + 1 })),
    removeAllBears: () => set({ bears: 0 }),
    updateBears: (newBears: number) => set({ bears: newBears }),



}))


export type SearchStoreState = {
    search: string,
    set_search_value: (new_search_value: string) => void
}
export const useSearchStore = create<SearchStoreState>((set) => ({
    search: '',
    set_search_value: (new_search_value) => set({ search: new_search_value }),
}))
