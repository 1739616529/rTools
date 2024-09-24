
import {
    createBrowserRouter,
    RouteObject,
} from "react-router-dom";
import Layout from "@/layout"




export const page: RouteObject[] = [
    {
        index: true,
        lazy: () => import("@/page/Main/Main"),
    }
]



export const layout: RouteObject[] = [
    {
        path: "/",
        element: <Layout />,
        children: page,
    }
]


export const router = createBrowserRouter(layout);
