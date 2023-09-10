import React from 'react'
import ReactDOM from 'react-dom/client'
import './index.css'
import App from "./pages/app.tsx";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import Report from "./pages/report.tsx";
import Post from "./pages/post.tsx";

const router = createBrowserRouter([
    {
        path: "/",
        element: <App/>
    },
    {
        path: "/register",
        element: <Report/>
    },
    {
        path: "/post/:id",
        element: <Post/>
    }
])

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
      <RouterProvider router={router} />
    {/*<App />*/}
  </React.StrictMode>,
)
