import { createHashRouter } from "react-router-dom";

import OAuth from "@/views/auth";
import Home from "@/views/home";
import Layout from "@/components/layout";

const router = createHashRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { index: true, element: <Home /> },
      {
        path: "contacts",
        element: <div>通讯录界面</div>,
      },
      {
        path: "settings",
        element: <div>通讯录界面</div>,
      },
    ],
  },
  {
    path: "/auth",
    element: <OAuth />,
  },
]);
export default router;
